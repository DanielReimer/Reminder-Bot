table! {
    reminders (id) {
        id -> Int4,
        nick -> Varchar,
        channel -> Varchar,
        set_time -> Int8,
        remind_time -> Int8,
        reminded -> Bool,
        remind_message -> Text,
    }
}
