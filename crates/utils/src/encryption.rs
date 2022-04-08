use data_encoding::HEXUPPER;
use ring::error::Unspecified;
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use std::num::NonZeroU32;
const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
const ITERATIONS: u32 = 1_000;

pub fn hash_password(password: &str) -> Result<(String, String), Unspecified> {
    let n_iter = NonZeroU32::new(ITERATIONS).unwrap();
    let rng = rand::SystemRandom::new();

    let mut salt = [0u8; CREDENTIAL_LEN];
    rng.fill(&mut salt)?;

    let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        password.as_bytes(),
        &mut pbkdf2_hash,
    );
    Ok((HEXUPPER.encode(&pbkdf2_hash), HEXUPPER.encode(&salt)))
}

pub fn verify_password(
    password: &str,
    salt: &str,
    expected_password: &[u8],
) -> Result<bool, Unspecified> {
    let n_iter = NonZeroU32::new(ITERATIONS).unwrap();
    let salt = HEXUPPER.decode(salt.as_bytes()).unwrap();
    let should_ok = pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        password.as_bytes(),
        expected_password,
    );
    Ok(should_ok.is_ok())
}

#[cfg(test)]
mod tests {}
