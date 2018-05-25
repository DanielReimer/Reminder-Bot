#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use self::models::{Reminder, NewReminder};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_post<'a>(conn: &PgConnection, nick: &'a str, channel: &'a str, set_time: &'a i32, remind_time: &'a i32, scheduled: &'a bool, recurring_number: &'a i32, recurring_time: &'a i32) -> Reminder {
    use schema::reminders;

    let new_reminder = NewReminder {
        nick: nick,
        channel: channel,
        set_time: set_time,
        remind_time: remind_time,
        scheduled: scheduled,
        recurring_number: recurring_number,
        recurring_time: recurring_time,
    };

    diesel::insert_into(reminders::table)
        .values(&new_reminder)
        .get_result(conn)
        .expect("Error saving new reminder")
}
