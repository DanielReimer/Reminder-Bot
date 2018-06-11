// Copyright (c) 2018 Daniel Reimer
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms

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
use atoi::atoi;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use self::models::{Reminder, NewReminder};
use chrono::prelude::*;

/// Returns an active PgConnection
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

/// Returns the Reminder struct from the supplied arguments
///
/// # Arguments
///
/// * `conn` - A postgres database connection
/// * `nick` - A string slice representing the nick who the reminder is for
/// * `channel` - A string slice of the target channel for the reminder
/// * `set_time` - An i64 ref of the timestamp of when the time was set
/// * `remind_time` - An i64 ref of the timestamp of when the reminder should be reminded
/// * `remind_message` - A string slice of the message to be reminded
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

/// Returns the current time as an i64 timestamp
///
/// # Examples
///
/// ```
/// use reminder_bot::current_time;
/// use std::time::{SystemTime, UNIX_EPOCH};
/// let time = SystemTime::now().duration_since(UNIX_EPOCH)
///     .unwrap().as_secs() as i64;
/// assert_eq!(current_time(), time);
/// ```
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
/// use std::time::{SystemTime, UNIX_EPOCH};
/// let time = SystemTime::now().duration_since(UNIX_EPOCH)
///     .unwrap().as_secs() as i64;
/// assert_eq!(parse_reminder("me in 100 seconds to never to do something").unwrap(), (time+100, "to never to do something"));
/// ```
pub fn parse_reminder(msg: &str) -> Result<(i64, &str), &'static str> {
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
        return Err("Not enough arguments");
    }

    let time_type = fields[1];

    let remaining_msg = fields[2];
    if time_type == "in" {
        return in_routine(remaining_msg);
    }
    if time_type == "tomorrow" {
        return tomorrow_routine(remaining_msg);
    }

    Err("something went wrong")
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
/// use std::time::{SystemTime, UNIX_EPOCH};
/// let time = SystemTime::now().duration_since(UNIX_EPOCH)
///     .unwrap().as_secs() as i64;
/// assert_eq!(in_routine("50 seconds to sleep").unwrap(), (time+50, "to sleep"));
/// ```
pub fn in_routine(msg: &str) -> Result<(i64, &str), &'static str> {
    // validate right number of arguments
    let reminder_next_regex = Regex::new(r"\W+").unwrap();
    let fields: Vec<&str> = reminder_next_regex.splitn(msg, 2).collect();

    if fields.len() < 2 {
        return Err("Not enough arguments");
    }

    // add the current time
    match extract_in_time(&msg) {
        Ok((reminder_time, reminder_message)) => return Ok((reminder_time+current_time(), reminder_message)),
        Err(e) => return Err(e),
    }
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
/// assert_eq!(tomorrow_routine("at 9 pm to do stuff").is_err(), true);
/// ```
pub fn tomorrow_routine(_msg: &str) -> Result<(i64, &str), &'static str> {
    //calculate next day (10am)
    let local: DateTime<Local> = Local::now();
    let tomorrow_midnight = (local + chrono::Duration::days(1)).date().and_hms(10, 0, 0);
    println!("{:?}", tomorrow_midnight);
    Err("Tomorrow not yet implemented")
}

/// Returns the difference in time to remind and mesage
///
/// # Arguments
///
/// * `msg` - A string slice of the time and message
///
/// # Examples
///
/// ```
/// use reminder_bot::extract_in_time;
/// assert_eq!(extract_in_time("100 seconds to say hi").unwrap(), (100, "to say hi"));
/// ```
pub fn extract_in_time(msg: &str) -> Result<(i64, &str), &'static str> {
    let next_regex = Regex::new(r"\W+").unwrap();
    let fields: Vec<&str> = next_regex.splitn(msg, 3).collect();

    if fields.len() <= 2 {
        return Err("Not enough arguments");
    }

    // convert the number to a i64
    match atoi::<i64>(fields[0].as_bytes()) {
        Some(s) => {
            match fields[1] {
                "seconds" => return Ok((s, fields[2])),
                "minutes" => return Ok((s*60, fields[2])),
                "hours" => return Ok((s*3600, fields[2])),
                "days" => return Ok((s*86400, fields[2])),
                "weeks" => return Ok((s*604800, fields[2])),
                _ => return Err("Expected time specifier"),
            }
        },
        None => return Err("expected number"),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_time() {
        let time = SystemTime::now().duration_since(UNIX_EPOCH)
            .unwrap().as_secs() as i64;
        assert_eq!(current_time(), time);
        // current time should always be greater than then
        assert!(time > 1528679549);
    }

    #[test]
    fn test_parse_reminder() {
        assert_eq!(parse_reminder("me never to do something").is_err(), true);
                assert_eq!(parse_reminder("me").is_err(), true);
        assert_eq!(parse_reminder("me in 100 seconds to never to do something").unwrap(),
                   (current_time()+100, "to never to do something"));
        assert_eq!(parse_reminder("me in 100 hours to never to do something").unwrap(),
                   (current_time()+360000, "to never to do something"));
    }

    #[test]
    fn test_in_routine() {
        assert_eq!(in_routine("-10 seconds to do stuff").is_err(), true);
        assert_eq!(in_routine("50 seconds to sleep").unwrap(), (current_time()+50, "to sleep"));
        assert_eq!(in_routine("1000 minutes to sleep").unwrap(), (current_time()+60_000, "to sleep"));
    }

    #[test]
    fn test_tomorrow_routine() {
        assert_eq!(tomorrow_routine("today seconds to do stuff").is_err(), true);
    }

    #[test]
    fn test_extract_in_time() {
        assert_eq!(extract_in_time("-100 seconds to say hi").is_err(), true );
        assert_eq!(extract_in_time("100").is_err(), true );
        assert_eq!(extract_in_time("100 seconds to say hi").unwrap(), (100, "to say hi"));
        assert_eq!(extract_in_time("10 minutes to say hi").unwrap(), (600, "to say hi"));
        assert_eq!(extract_in_time("10 hours to say hi").unwrap(), (36000, "to say hi"));
        assert_eq!(extract_in_time("2 days to say hi").unwrap(), (2*86400, "to say hi"));
        assert_eq!(extract_in_time("3 weeks to say hi").unwrap(), (3*604800, "to say hi"));
        assert_eq!(extract_in_time("3 nanoseconds to say hi").is_err(), true);
    }
}
