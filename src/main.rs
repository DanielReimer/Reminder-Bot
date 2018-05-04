extern crate irc;

use irc::client::prelude::*;

fn main() {
    // We can also load the Config at runtime via Config::load("path/to/config.toml")
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

    reactor.register_client_with_handler(client, |client, message| {
        print!("{}", message);
        // And here we can do whatever we want with the messages.
        if let Command::PRIVMSG(ref target, ref msg) = message.command {
            if msg.contains("pickles") {
                client.send_privmsg(target, "Hi!").unwrap();
            }
        }
        Ok(())
    });

    reactor.run().unwrap();
}