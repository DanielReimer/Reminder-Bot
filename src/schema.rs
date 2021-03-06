// Copyright (c) 2018 Daniel Reimer
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms

// Autogenerated by Diesel
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
