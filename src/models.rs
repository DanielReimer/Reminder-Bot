// Copyright (c) 2018 Daniel Reimer
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms

use super::schema::reminders;

/// Holds the entire reminder
#[derive(Queryable)]
pub struct Reminder {
    pub id: i32,
    pub nick: String,
    pub channel: String,
    pub set_time: i64,
    pub remind_time: i64,
    pub reminded: bool,
    pub remind_message: String,
}

/// Holds everything that is required to set a new reminder
#[derive(Insertable)]
#[table_name = "reminders"]
pub struct NewReminder<'a> {
    pub nick: &'a str,
    pub channel: &'a str,
    pub set_time: &'a i64,
    pub remind_time: &'a i64,
    pub remind_message: &'a str,
}
