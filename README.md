# Reminder Bot

## Getting Setup

Install Diesel

`cargo install diesel_cli`

And then initialize the database by

`diesel setup`
`diesel migration run`

Next, we need to tell it where the database lives

`echo DATABASE_URL=postgres://username:password@localhost/database > .env`
