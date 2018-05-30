# Reminder Bot

## Getting Setup

### Database Setup
Install Diesel

`cargo install diesel_cli`

And then initialize the database by

`diesel setup`
`diesel migration run`

Another possible way of setting up the database is by manually making a new table. The table is defined in `migrations/reminders/up.sql`

Next, we need to tell it where the database lives

`echo DATABASE_URL=postgres://username:password@localhost/database > .env`

## IRC Setup

Inside `main.rs`, specify the nickname, server, and channels that the bot will connect to.