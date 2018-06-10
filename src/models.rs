use super::schema::reminders;

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

#[derive(Insertable)]
#[table_name = "reminders"]
pub struct NewReminder<'a> {
    pub nick: &'a str,
    pub channel: &'a str,
    pub set_time: &'a i64,
    pub remind_time: &'a i64,
    pub remind_message: &'a str,
}
