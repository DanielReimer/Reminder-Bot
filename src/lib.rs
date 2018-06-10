#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate chrono;
extern crate atoi;
extern crate regex;

pub mod schema;
pub mod models;

use std::time::{SystemTime, UNIX_EPOCH};
use regex::Regex;
//use std::time;
use std::str;
use atoi::atoi;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use self::models::{Reminder, NewReminder};
use chrono::prelude::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_post<'a>(conn: &PgConnection, nick: &'a str, channel: &'a str, set_time: &'a i64, remind_time: &'a i64, remind_message: &'a str) -> Reminder {
    use schema::reminders;

    let new_reminder = NewReminder {
        nick: nick,
        channel: channel,
        set_time: set_time,
        remind_time: remind_time,
        remind_message: remind_message,
    };

    diesel::insert_into(reminders::table)
        .values(&new_reminder)
        .get_result(conn)
        .expect("Error saving new reminder")
}

/// Returns the current time as an i64 times
pub fn current_time() -> i64 {
    // get current time in timestamp
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let current_time: i64 = since_the_epoch.as_secs() as i64;
    current_time
}

/// Returns the timestamp and message to remind
///
/// # Arguments
///
/// * `msg` - A string slice of a reminder to parse with beginning meta information removed
///
/// # Examples
///
/// ```
/// use reminder_bot::parse_reminder;
/// assert_eq!(parse_reminder("me never to do something").is_some(), false);
/// ```
pub fn parse_reminder(msg: &str) -> Option<(i64, &str)> {
    //let reminder_set_regex = Regex::new(r"^\s*(?i)(reminderbot: )?remind (?-i)").unwrap();
    //let reminder = reminder_set_regex.replace(msg, "");

    //find out who is reminder is for
    let reminder_next_regex = Regex::new(r"\W+").unwrap();
    let fields: Vec<&str> = reminder_next_regex.splitn(msg, 3).collect();
    //let reminder_time_message = fields[1];

    //let for_user = fields[0];
    /*if for_user != "me" {
        match client.list_users(target) {
            Some(list) => println!("{}", list[1].get_nickname()),//client.send_privmsg(target, "setting reminder").unwrap(),
             None => client.send_privmsg(target, "Sorry, could not set reminder").unwrap(),
        }
    }*/

    //fields = reminder_next_regex.splitn(reminder_time_message, 2).collect();

    if fields.len() <= 2 {
        return None;
    }

    let time_type = fields[1];

    //println!("{:?}", fields);
    let remaining_msg = fields[2];
    if time_type == "in" {
        return in_routine(remaining_msg);
    }
    if time_type == "tomorrow" {
        return tomorrow_routine(remaining_msg);
    }

    None
}


/// Returns the timestamp and message to remind
///
/// # Arguments
///
/// * `msg` - A string slice of time and message
///
/// # Examples
///
/// ```
/// use reminder_bot::in_routine;
/// assert_eq!(in_routine("NAN seconds to do stuff").is_some(), false);
/// ```
fn in_routine(msg: &str) -> Option<(i64, &str)> {
    let reminder_next_regex = Regex::new(r"\W+").unwrap();
    let fields: Vec<&str> = reminder_next_regex.splitn(msg, 2).collect();
    let reminder_message = fields[1];

    println!("{}", reminder_message);
    match atoi::<i64>(fields[0].as_bytes()) {
        Some(s) => {
            return Some((current_time()+s, &reminder_message));
        },
        None => (),
    };
    //println!("\nSaved draft {} with id {}", nick_name, reminder.id);
    None
}

/// Returns the timestamp and message to remind
///
/// # Arguments
///
/// * `msg` - A string slice of time and message
///
/// # Examples
///
/// ```
/// use reminder_bot::tomorrow_routine;
/// assert_eq!(tomorrow_routine("today seconds to do stuff").is_some(), false);
/// ```
fn tomorrow_routine(msg: &str) -> Option<(i64, &str)> {
    //let reminder_message = fields[1];

    //calculate next day (10am)
    let local: DateTime<Local> = Local::now();
    let tomorrow_midnight = (local + chrono::Duration::days(1)).date().and_hms(9, 30, 0);
    println!("{:?}", tomorrow_midnight);
    None
}
