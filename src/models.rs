use super::schema::reminders;

#[derive(Queryable)]
pub struct Reminder {
    pub id: i32,
    pub nick: String,
    pub channel: String,
    pub set_time: i32,
    pub remind_time: i32,
    pub scheduled: bool,
    pub recurring_number: i32,
    pub recurring_time: i32,
}

#[derive(Insertable)]
#[table_name = "reminders"]
pub struct NewReminder<'a> {
    pub nick: &'a str,
    pub channel: &'a str,
    pub set_time: &'a i32,
    pub remind_time: &'a i32,
    pub scheduled: &'a bool,
    pub recurring_number: &'a i32,
    pub recurring_time: &'a i32,
}
