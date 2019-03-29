#[macro_use]
extern crate serenity;

use serenity::client::Client;
use serenity::client::Context;
use serenity::prelude::EventHandler;
use serenity::model::channel::Message;
use serenity::framework::standard::StandardFramework;

use serde_json::{Value};

use std::env;
use std::error::Error;

use phils_close_bot::*;
use serenity::http;

struct Handler;

impl EventHandler for Handler {
    fn message(&self, _ctx: Context, new_message: Message) {
        if new_message.is_own() {
            return
        }

        println!("Received message: {:#?}", new_message.content);

        if is_phil(&new_message.author) && is_close_message(&new_message.content) {
            let msg: Value = serde_json::from_str(r#"{
                "content": "Phil did you mean: \nhttps://tenor.com/view/kill-fire-kiev-killitwithfire-simpsons-gif-4729144",
                "tts": false
            }"#).unwrap();

            match http::send_message(new_message.channel_id.0, &msg) {
                Ok(_) => {},
                Err(e) => println!("Error {}", e.description())
            }
        }

        if is_perfect(&new_message.author) && new_message.content.contains("phil ping-pong") {
            let msg: Value = serde_json::from_str(r#"{
                "content": "Ping-pong alert",
                "tts": false
            }"#).unwrap();

            match http::send_message(new_message.channel_id.0, &msg) {
                Ok(_) => {},
                Err(e) => println!("Error {}", e.description())
            }
        }
    }
}

    /// possible update
//    fn message_update(&self, _ctx: Context, _new_data: MessageUpdateEvent) {
//        if let Some(content) = _new_data.content {
//            _new_data
//        }
//    }

fn main() {
    let token = env::var("DISCORD_TOKEN")
        .expect("Provide the DISCORD_TOKEN env");

    // Login with a bot token from the environment
    let mut client = Client::new(&token, Handler)
        .expect("Error creating client");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .cmd("ping", ping);

    client.with_framework(framework);

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

command!(ping(_context, message) {
    let _ = message.reply("Pong!");
});

