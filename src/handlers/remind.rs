use actix_web::web::{Query, Json};
use serde::Deserialize;


async fn get_reminders(todo_id:Query<String>){}
#[derive(Deserialize)]
pub struct RelativeFireMinute{
    relative_fire_minute:i32,
}
async fn add_reminder(todo_id:Query<String>, min:Json<RelativeFireMinute>){}
async fn delete_reminder(todo_id:Query<String>){}