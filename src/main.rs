extern crate reminder_bot;
extern crate irc;
extern crate diesel;

use reminder_bot::*;
use irc::client::prelude::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use reminder_bot::schema::reminders::dsl::*;

    let connection = establish_connection();

    //let nick_name = "pigs";

    //let reminder = create_post(&connection, &nick_name, &100, &200);
    //println!("\nSaved draft {} with id {}", nick_name, reminder.id);

    let config = Config {
        nickname: Some("reminderbot".to_owned()),
        server: Some("irc.cat.pdx.edu".to_owned()),
        channels: Some(vec!["#pigspen".to_owned()]),
        use_ssl: Some(true),
        ..Config::default()
    };

    let mut reactor = IrcReactor::new().unwrap();
    let client = reactor.prepare_client_and_connect(&config).unwrap();
    client.identify().unwrap();

    reactor.register_client_with_handler(client, move |client, message| {
        print!("{}", message);
        // And here we can do whatever we want with the messages.
        if let Command::PRIVMSG(ref target, ref msg) = message.command {
            if msg.contains("pickles") {
                client.send_privmsg(target, "Hi!").unwrap();
                let results = reminders/*.filter(published.eq(true))*/
                    .limit(5)
                    .load::<Reminder>(&connection)
                    .expect("Error loading reminders");

                for reminder in results {
                    client.send_privmsg(target, &reminder.nick);
                    println!("{}", reminder.nick);
                    println!("----------\n");
                    println!("{}", reminder.set_time);
                    println!("{}", reminder.remind_time);
                }
            }
        }
        Ok(())
    });

    reactor.run().unwrap();
}
