# Reminder Bot

Copyright (c) 2018 Daniel Reimer

Reminder Bot is an IRC bot written in Rust that allows people to set reminders.

## Getting Setup

### Database Setup
Install Diesel

`cargo install diesel_cli`

And then initialize the database by

`diesel setup`
`diesel migration run`

An alternative to using Diesel is to setup up the database manually. The table schema is defined in `migrations/reminders/up.sql`

Next, we need to tell it where the database lives

`echo DATABASE_URL=postgres://username:password@serverhost/database > .env`

## IRC Setup

Inside `irc.toml`, specify the nickname, server, and channels that the bot will connect to.

## License

This program is licensed under the "MIT License". Please see the file LICENSE in the source distribution of this software for lincense terms.