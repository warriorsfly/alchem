use crate::{settings::structs::RateLimitConfig, utils::get_ip, IpAddr};
use futures::future::{ok, Ready};
use rate_limiter::{RateLimitType, RateLimiter};
use std::{
  future::Future,
  pin::Pin,
  sync::Arc,
  task::{Context, Poll},
};
use tokio::sync::Mutex;

pub mod rate_limiter;

#[derive(Debug, Clone)]
pub struct RateLimit {
  // it might be reasonable to use a std::sync::Mutex here, since we don't need to lock this
  // across await points
  pub rate_limiter: Arc<Mutex<RateLimiter>>,
  pub rate_limit_config: RateLimitConfig,
}

#[derive(Debug, Clone)]
pub struct RateLimited {
  rate_limiter: Arc<Mutex<RateLimiter>>,
  rate_limit_config: RateLimitConfig,
  type_: RateLimitType,
}

pub struct RateLimitedMiddleware<S> {
  rate_limited: RateLimited,
  service: S,
}

impl RateLimit {
  pub fn message(&self) -> RateLimited {
    self.kind(RateLimitType::Message)
  }

  pub fn post(&self) -> RateLimited {
    self.kind(RateLimitType::Post)
  }

  pub fn register(&self) -> RateLimited {
    self.kind(RateLimitType::Register)
  }

  pub fn image(&self) -> RateLimited {
    self.kind(RateLimitType::Image)
  }

  pub fn comment(&self) -> RateLimited {
    self.kind(RateLimitType::Comment)
  }

  fn kind(&self, type_: RateLimitType) -> RateLimited {
    RateLimited {
      rate_limiter: self.rate_limiter.clone(),
      rate_limit_config: self.rate_limit_config.clone(),
      type_,
    }
  }
}

impl RateLimited {
  pub async fn wrap<T, E>(
    self,
    ip_addr: IpAddr,
    fut: impl Future<Output = Result<T, E>>,
  ) -> Result<T, E>
  where
    E: From<DoubleZeroError>,
  {
    // Does not need to be blocking because the RwLock in settings never held across await points,
    // and the operation here locks only long enough to clone
    let rate_limit = self.rate_limit_config;

    // before
    {
      let mut limiter = self.rate_limiter.lock().await;

      match self.type_ {
        RateLimitType::Message => {
          limiter.check_rate_limit_full(
            self.type_,
            &ip_addr,
            rate_limit.message,
            rate_limit.message_per_second,
            false,
          )?;

          drop(limiter);
          return fut.await;
        }
        RateLimitType::Post => {
          limiter.check_rate_limit_full(
            self.type_,
            &ip_addr,
            rate_limit.post,
            rate_limit.post_per_second,
            true,
          )?;
        }
        RateLimitType::Register => {
          limiter.check_rate_limit_full(
            self.type_,
            &ip_addr,
            rate_limit.register,
            rate_limit.register_per_second,
            true,
          )?;
        }
        RateLimitType::Image => {
          limiter.check_rate_limit_full(
            self.type_,
            &ip_addr,
            rate_limit.image,
            rate_limit.image_per_second,
            false,
          )?;
        }
        RateLimitType::Comment => {
          limiter.check_rate_limit_full(
            self.type_,
            &ip_addr,
            rate_limit.comment,
            rate_limit.comment_per_second,
            false,
          )?;
        }
      };
    }

    let res = fut.await;

    // after
    {
      let mut limiter = self.rate_limiter.lock().await;
      if res.is_ok() {
        match self.type_ {
          RateLimitType::Post => {
            limiter.check_rate_limit_full(
              self.type_,
              &ip_addr,
              rate_limit.post,
              rate_limit.post_per_second,
              false,
            )?;
          }
          RateLimitType::Register => {
            limiter.check_rate_limit_full(
              self.type_,
              &ip_addr,
              rate_limit.register,
              rate_limit.register_per_second,
              false,
            )?;
          }
          _ => (),
        };
      }
    }

    res
  }
}

impl<S> Transform<S, ServiceRequest> for RateLimited
where
  S: Service<ServiceRequest, Response = ServiceResponse, Error = actix_web::Error>,
  S::Future: 'static,
{
  type Response = S::Response;
  type Error = actix_web::Error;
  type InitError = ();
  type Transform = RateLimitedMiddleware<S>;
  type Future = Ready<Result<Self::Transform, Self::InitError>>;

  fn new_transform(&self, service: S) -> Self::Future {
    ok(RateLimitedMiddleware {
      rate_limited: self.clone(),
      service,
    })
  }
}

type FutResult<T, E> = dyn Future<Output = Result<T, E>>;

impl<S> Service<ServiceRequest> for RateLimitedMiddleware<S>
where
  S: Service<ServiceRequest, Response = ServiceResponse, Error = actix_web::Error>,
  S::Future: 'static,
{
  type Response = S::Response;
  type Error = actix_web::Error;
  type Future = Pin<Box<FutResult<Self::Response, Self::Error>>>;

  fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    self.service.poll_ready(cx)
  }

  fn call(&self, req: ServiceRequest) -> Self::Future {
    let ip_addr = get_ip(&req.connection_info());

    let fut = self
      .rate_limited
      .clone()
      .wrap(ip_addr, self.service.call(req));

    Box::pin(async move { fut.await.map_err(actix_web::Error::from) })
  }
}
