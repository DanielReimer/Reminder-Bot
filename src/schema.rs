table! {
    reminders (id) {
        id -> Int4,
        nick -> Varchar,
        channel -> Varchar,
        set_time -> Int4,
        remind_time -> Int4,
        scheduled -> Bool,
        recurring_number -> Int4,
        recurring_time -> Int4,
    }
}
