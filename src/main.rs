extern crate reminder_bot;
extern crate irc;
extern crate diesel;
extern crate regex;
extern crate chrono;
extern crate atoi;

use std::default::Default;
use std::time::{SystemTime, UNIX_EPOCH};
use std::thread;
use std::time;
use std::str;

use self::models::*;
use self::diesel::prelude::*;
use reminder_bot::*;
use reminder_bot::schema::reminders::dsl::*;

use irc::client::prelude::*;
use irc::error;

use regex::Regex;
use chrono::prelude::*;
use atoi::atoi;

fn main() {

    //let nick_name = "pigs";
    //let channel_name = "#pigspen";
    //let reminder = create_post(&connection, &nick_name, &channel_name, &100, &200, &true, &1, &1000);
    //println!("\nSaved draft {} with id {}", nick_name, reminder.id);


    // config for IRC server
    let config = Config {
        nickname: Some("reminder".to_owned()),
        server: Some("irc.cat.pdx.edu".to_owned()),
        channels: Some(vec!["#pigspen".to_owned()]),
        use_ssl: Some(true),
        ..Default::default()
    };

    let mut reactor = IrcReactor::new().unwrap();
    let client = reactor.prepare_client_and_connect(&config).unwrap();
    client.identify().unwrap();

    thread::spawn(move || {
        // conncect to the database
        let connection = establish_connection();

        loop {
            let results = reminders
                .load::<Reminder>(&connection)
                .expect("Error loading reminders");

            //let start = SystemTime::now();
            //let since_the_epoch = start.duration_since(UNIX_EPOCH)
            //    .expect("Time went backwards");

            for reminder in results {
                println!("{}", reminder.nick);
                println!("{}", reminder.channel);
                println!("{}", reminder.set_time);
                println!("-----------------------------------");
             }

            // check for ready reminders every 30 seconds
            thread::sleep(time::Duration::from_millis(30_000));
        }
    });

    // make a clone for the thread
    let client_clone = client.clone();

    // thread to check for reminders to print
    thread::spawn(move || {
        // conncect to the database
        let connection = establish_connection();

        // wait till connected to IRC before printing
        thread::sleep(time::Duration::from_millis(5000));
        let _ = client_clone.send_privmsg("#pigspen", "yooooo");

        let frequency: u64 = 30_000;

        loop {

            let results = reminders
                .load::<Reminder>(&connection)
                .expect("Error loading reminders");

            let start = SystemTime::now();
            let since_the_epoch = start.duration_since(UNIX_EPOCH)
                .expect("Time went backwards");

            for reminder in results {
                if (since_the_epoch.as_secs() - frequency/100) as i32 <= reminder.remind_time {
                    if reminder.remind_time <= (since_the_epoch.as_secs() as i32) {
                        client_clone.send_privmsg(&reminder.channel, "message was found");
                    }
                }
            }

            // check for ready reminders every 30 seconds
            thread::sleep(time::Duration::from_millis(frequency));
        }
    });

    reactor.register_client_with_handler(client, process_msg);
    reactor.run().unwrap();

}

fn process_msg(client: &IrcClient, message: Message) -> error::Result<()> {
    print!("{}", message);


    let connection = establish_connection();

    if let Command::PRIVMSG(ref target, ref msg) = message.command {
        //println!("{:?}", message.source_nickname());
        let mut nick_handle = "";
        match message.source_nickname() {
            Some(s) => nick_handle = s,
            None => nick_handle = "",
        }

        // matches for source
        let source = Regex::new(r"^(?i)reminderbot:? source(?-i)").unwrap();
        if source.is_match(msg) {
            client.send_privmsg(target, "Source: https://github.com/DanielReimer/Reminder-Bot").unwrap();
        }

        // matches for help display
        let help = Regex::new(r"^(?i)reminderbot:? help(?-i)").unwrap();
        if help.is_match(msg) {
            use chrono::TimeZone;
            use chrono::offset::LocalResult;

            // Construct a datetime from epoch:
            let dt = Local::today();
            println!("{}", dt);
            let start = SystemTime::now();
            let since_the_epoch = start.duration_since(UNIX_EPOCH)
                .expect("Time went backwards");
            println!("{}", since_the_epoch.as_secs());
            client.send_privmsg(target, message.response_target().unwrap_or(target)).unwrap();
        }
        //matches for a new reminder
        let reminder_set_regex = Regex::new(r"^(?i)(reminderbot: )?remind (?-i)").unwrap();
        if reminder_set_regex.is_match(msg) {
            //remove begining bit
            let reminder = &reminder_set_regex.replace(msg, "");

            //find out who is reminder is for
            let reminder_next_regex = Regex::new(r"\W+").unwrap();
            let mut fields: Vec<&str> = reminder_next_regex.splitn(reminder, 2).collect();
            let reminder_time_message = fields[1];
            let for_user = fields[0];

            println!("{:?}", &fields);

            if for_user != "me" {
                match client.list_users(target) {
                    Some(list) => println!("{}", list[1].get_nickname()),//client.send_privmsg(target, "setting reminder").unwrap(),
                    None => client.send_privmsg(target, "Sorry, could not set reminder").unwrap(),
                }
            } else {
                println!("meeeeee");

            }

            fields = reminder_next_regex.splitn(reminder_time_message, 2).collect();
            let time_type = fields[0];

            println!("{:?}", &fields);

            if time_type == "in" {
                println!("it was in");
                let reminder_time: Vec<&str> = reminder_next_regex.splitn(fields[1], 2).collect();

                let start = SystemTime::now();
                let since_the_epoch = start.duration_since(UNIX_EPOCH)
                    .expect("Time went backwards");

                println!("{} and {:?}", fields[1], fields[1].as_bytes());
                match atoi::<i32>(fields[1].as_bytes()) {
                    Some(s) => {create_post(&connection, &nick_handle, &target, &(since_the_epoch.as_secs() as i32), &(((since_the_epoch.as_secs()as i32)+s) as i32), &false, &0, &0); ()}
                    None => println!("Error"),
                };
                //println!("\nSaved draft {} with id {}", nick_name, reminder.id);
                //println!("reminder was set for: {}", (since_the_epoch.as_secs()+fields[0].parse().unwrap()));
            }

            //client.send_privmsg(target, SystemTime::now().as_secs()).unwrap();
            //let start = SystemTime::now();
            //let since_the_epoch = start.duration_since(UNIX_EPOCH)
            //    .expect("Time went backwards");
            //println!("{}", since_the_epoch.as_secs());
            //lient.send_privmsg(target, &(since_the_epoch.as_secs().to_string())).unwrap();
        }
    }
    Ok(())
}
