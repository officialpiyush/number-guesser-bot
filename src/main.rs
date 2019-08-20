/**
 * Copyright (C) 2019 Piyush Bhangale
 *
 * This file is part of number-guesser-bot.
 *
 * number-guesser-bot is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * number-guesser-bot is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with number-guesser-bot.  If not, see <http://www.gnu.org/licenses/>.
 */
// #[macro_use]
// extern crate lazy_static;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fs::File;
use std::path::Path;

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

static mut GENERATE_NEW: bool = true;
static mut SECRET_NUMBER: u32 = 0;
static mut CHANNEL_ID: &'static str = "";

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        unsafe {
            if msg.channel_id.to_string() == CHANNEL_ID {
                if GENERATE_NEW {
                    change_number();
                    GENERATE_NEW = false;
                }
                let guess: u32 = match msg.content.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        return;
                    }
                };
                match guess.cmp(&SECRET_NUMBER) {
                    Ordering::Less => {
                        if let Err(why) = msg.channel_id.say(&ctx.http, "Number too small.") {
                            println!("Error sending message: {:?}", why);
                        }
                    }
                    Ordering::Greater => {
                        if let Err(why) = msg.channel_id.say(&ctx.http, "Number too big.") {
                            println!("Error sending message: {:?}", why);
                        }
                    }
                    Ordering::Equal => {
                        if let Err(why) =
                            msg.channel_id.say(&ctx.http, "Yay you guessed it right!!!")
                        {
                            println!("Error sending message: {:?}", why);
                        };
                        GENERATE_NEW = true;
                    }
                }
            }
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {
    let json_file_path = Path::new("config.json");
    let json_file = File::open(json_file_path).expect("config file not found");

    let config: Config =
        serde_json::from_reader(json_file).expect("error while reading from config.json");

    let token = config.token;

    unsafe {
        CHANNEL_ID = string_to_static_str(config.channelID);
    }

    let mut client = Client::new(&token, Handler).expect("Err creating client");

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}

fn change_number() {
    unsafe {
        SECRET_NUMBER = rand::thread_rng().gen_range(1, 101);
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
struct Config {
    token: String,
    channelID: String,
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
