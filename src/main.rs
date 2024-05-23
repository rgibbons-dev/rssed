use std::io::BufRead;
use std::{error::Error, io};
use rss::Channel;


async fn fetch_feed() -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get("https://www.nhc.noaa.gov/xml/MIMATS.xml")
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

async fn get_descr() -> Option<String> {
    if let Ok(channel) = fetch_feed().await {
        let v_chan = channel.into_items();
        if let Some(descr) = v_chan[0].description.clone() {
            Some(descr)
        } else {
            None
        }
    } else {
        None
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    loop {
        let mut user_input = String::new();
        let stdin = io::stdin();
        stdin.lock().read_line(&mut user_input).unwrap();
    }
    Ok(())
}

// repl
// commands:
// add feed :: 1 -> a <url>
// get from all feeds :: 1 -> g
// delete feed :: 1 -> <number>d
// delete all feeds :: 1 -> ,d
// print :: 1 -> p
// print nth line :: 1 -> <n>p
// print all lines :: 1 -> ,p
// for now, not persisted

// future:

// select feed :: 2 -> s <int>
// -> view titles of all entries :: 2
// -> view content of entry by (title? number?) :: 2

// focus :: 2 -> f <mode>
// -> feed mode :: 2 -> f f
// -> entry mode :: 2 -> f <f_num> e

// save entry (persist to file) :: 3
// otherwise store last n entries :: 3
