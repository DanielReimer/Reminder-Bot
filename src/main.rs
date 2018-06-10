extern crate reminder_bot;
extern crate irc;
extern crate diesel;
extern crate regex;
extern crate chrono;
extern crate atoi;

//use std::default::Default;
use std::time::{SystemTime, UNIX_EPOCH};
use std::thread;
use std::time;
//use std::str;
use std::path::Path;

use self::models::*;
use self::diesel::prelude::*;
use reminder_bot::*;
use reminder_bot::schema::reminders::dsl::*;

use irc::client::prelude::*;
use irc::client::data::Config;
use irc::error;

use regex::Regex;
use chrono::prelude::*;
//use atoi::atoi;

fn main() {
    // configure and connect to irc
    let (client, mut reactor) = connect_to_irc();

    // debug thread to temporarly show all entries in database
    thread::spawn(move || {
        let connection = establish_connection();

        thread::sleep(time::Duration::from_millis(5000));
        loop {

            println!("----------------Printing---------------------");
            let results = reminders
                .load::<Reminder>(&connection)
                .expect("Error loading reminders");

            for reminder in results {
                println!("{}", reminder.remind_time);
             }
            thread::sleep(time::Duration::from_millis(15_000));
        }
    });

    check_for_reminders(client.clone());
    delete_old_entries();

    reactor.register_client_with_handler(client, process_msg);
    reactor.run().expect("Could not run the IRC client");
}

/// Load IRC config and establish a connection
fn connect_to_irc() -> (IrcClient, IrcReactor) {
    // load the config for IRC server
    let path = Path::new("irc.toml");
    let config = Config::load(path).expect("Could not load IRC configuration file");

    // connect to IRC
    let mut reactor = IrcReactor::new().expect("Could not make new IrcReactor");
    let client = reactor.prepare_client_and_connect(&config).expect("Could not prepare client");
    client.identify().expect("Could not identify client");
    (client, reactor)
}

/// thread to check for reminders to print
fn check_for_reminders(client: IrcClient) -> () {
    thread::spawn(move || {
        // conncect to the database
        let connection = establish_connection();
        let frequency: u64 = 10_000;

        loop {
            // check for ready reminders every 30 seconds
            thread::sleep(time::Duration::from_millis(frequency));
            print_reminders(&connection, &client);
        }
    });
}

fn print_reminders(connection: &PgConnection, client: &IrcClient) -> () {
    // get all reminders that have not been reminded yet
    let results = reminders
        .filter(reminded.eq(false))
        .load::<Reminder>(connection)
        .expect("Error loading reminders");

    // get current time
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    for mut reminder in results {
        if reminder.remind_time <= (since_the_epoch.as_secs() as i64) {
            // subtract 25200 to appear in correct timezone
            //let dt = NaiveDateTime::from_timestamp(reminder.set_time - 25200, 0);
            let dt = (NaiveDateTime::from_timestamp(reminder.set_time, 0), Local);
            let time_message = dt.format("%I %p on %b %-d").to_string();
            let format_message = format!("{}: Around {}, you asked me to remind you {}",
                                         reminder.nick, time_message, reminder.remind_message);
            print_msg(&client.clone(), &reminder.channel.clone(), &format_message);

            // update the entry to say that it has been reminded
            diesel::update(reminders.filter(id.eq(reminder.id)))
                .set(reminded.eq(true))
                .execute(connection)
                .expect("Error updating posts");
        }
    }
}

/// thread to occasionaly clean up old reminders
/// it should be 10+ minutes so users have a chance to 'snooze' the reminder
fn delete_old_entries() -> () {
    thread::spawn(move || {
        // conncect to the database
        let connection = establish_connection();
        let frequency: u64 = 50_000;

        loop {
            // check for old reminders every so often
            thread::sleep(time::Duration::from_millis(frequency));
            delete_entry(frequency as i64, &connection);
        }
    });
}

fn delete_entry(frequency: i64, connection: &PgConnection) -> () {
    println!("--------------------------Deleting Reminders-----------------------");
    let results = reminders
        .filter(reminded.eq(true))
        .load::<Reminder>(connection)
        .expect("Error loading reminders");

    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    for reminder in results {
        if reminder.remind_time <= ((since_the_epoch.as_secs() as i64) - ((frequency as i64)/1000)) {
            println!("Getting rid of reminder at {} for time is {}", reminder.remind_time, since_the_epoch.as_secs() as i32);
            diesel::delete(reminders.filter(id.eq(reminder.id)))
                .execute(connection)
                .expect("Error deleting posts");
        }
        else {
            println!("Not getting rid of reminder at {} for time is {} and expected removal is {}",
                     reminder.remind_time, since_the_epoch.as_secs() as i32, (since_the_epoch.as_secs() as i64) - 50_000/1000);
        }
    }
}

/// Prints out a message to console and IRC
///
/// # Arguments
///
/// * `client` - An IrcClient object reference
/// * `target` - A string slice of where the message is sent
/// * `message` - A string slice of the message to send
fn print_msg(client: &IrcClient, target: &str, message: &str) {
    let local: DateTime<Local> = Local::now();
    let print_str = local.format("%a %b %e %T").to_string();

    println!("{}  Target: {}   Message: {}", print_str, target, message);
    client.send_privmsg(target, message).expect("Unable to send message to IRC");
}

fn process_msg(client: &IrcClient, message: Message) -> error::Result<()> {
    print!("{}", message);

    let connection = establish_connection();

    if let Command::PRIVMSG(ref target, ref msg) = message.command {
        //println!("{:?}", message.source_nickname());
        let mut nick_handle;
        match message.source_nickname() {
            Some(s) => nick_handle = s,
            None => nick_handle = "",
        }

        // matches for source
        let source = Regex::new(r"^(?i)reminderbot:? source(?-i)").unwrap();
        if source.is_match(msg) {
            print_msg(&client.clone(), &target.clone(), "Source: https://github.com/DanielReimer/Reminder-Bot");
        }

        // matches for help display
        let help = Regex::new(r"^(?i)reminderbot:? help(?-i)").unwrap();
        if help.is_match(msg) {
            print_msg(&client.clone(), &target.clone(), "Help message");
            //client.send_privmsg(target, message.response_target().unwrap_or(target)).unwrap();
        }

        //matches for a new reminder
        let reminder_set_regex = Regex::new(r"^\s*(?i)(reminderbot: )?remind (?-i)").unwrap();
        if reminder_set_regex.is_match(msg) {
            //remove begining bit
            let reminder = &reminder_set_regex.replace(msg, "");

            match parse_reminder(reminder)  {
                Some(reminder_meta) => {
                    let (reminder_time, reminder_message) = reminder_meta;
                    create_post(&connection, &nick_handle, &target, &current_time(), &reminder_time, reminder_message);
                    ()
                },
                None => print_msg(&client.clone(), &target.clone(), "Sorry, I could not set your reminder"),
            }
        }
    }
    Ok(())
}
