use std::io::BufRead;
use std::{error::Error, io};
use rss::Channel;


async fn fetch_feed(feed: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(feed)
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    println!("{:#?}", channel);
    Ok(channel)
}

async fn get_title(url: &str) -> Option<String> {
    if let Ok(channel) = fetch_feed(url).await {
        let v_chan = channel.into_items();
        if let Some(title) = v_chan[0].title.clone() {
            Some(title)
        } else {
            None
        }
    } else {
        None
    }
}

struct Single(usize);

struct Range {
    start: usize,
    end: usize
}

enum LineAddress {
    Single(usize),
    Many(Range)
}

#[tokio::main]
async fn main() {
    let mut store: Vec<(String, Channel)> = Vec::new();
    let mut current_line = 0;
    let mut current_addr = LineAddress::Single(0);
    loop {
        let mut user_input = String::new();
        let stdin = io::stdin();
        stdin.lock().read_line(&mut user_input).unwrap();
        let tkns: Vec<&str> = user_input.split(' ').collect();
        let cmd = tkns[0].trim();
        let len = cmd.len();
        if len > 1 {
            let line_address: Vec<char> = cmd.chars().clone().collect();
            let la_len = line_address.len();
            if la_len == 1 {
                let addr = line_address[0];
                if addr == '.' {
                    current_addr = LineAddress::Single(current_line);
                } else if addr == '$' {
                    current_addr = LineAddress::Single(store.len());
                } else if addr == ';' {
                    current_addr = LineAddress::Many(Range { start: current_line, end: store.len() });
                } else if addr == ',' {
                    current_addr = LineAddress::Many(Range { start: 0, end: store.len() });
                } else if let Ok(x) = addr.to_string().parse::<usize>() {
                    current_addr = LineAddress::Single(x);
                }
            }
            if let Some(cmd_la) = line_address.clone().pop() {
                if cmd_la == 'p' {
                    let (cur_url, _cur_chan) = store[0].to_owned();
                    if let Some(title) = get_title(cur_url.as_str()).await {
                        println!("{}", title);
                    } else {
                        println!("?");
                    }
                } else if cmd_la == 'd' {
                    println!("d");
                } else {
                    println!("?");
                }
            } else {
                println!("?");
            }
        } else {
            if cmd == "a" {
                let url = tkns[1].trim().to_owned();
                println!("{}", url);
                if let Ok(chan) = fetch_feed(&url).await {
                    store.push((url, chan));
                } else {
                    println!("?");
                }
            } else if cmd == "g" {
                let store_cpy = store.clone();
                let store_iter = store_cpy.iter();
                for (index, (feed_url, _)) in store_iter.enumerate() {
                    if let Ok(new_chan) = fetch_feed(feed_url.as_str()).await {
                        store[index] = (feed_url.to_string(), new_chan);
                    } else {
                        println!("?");
                        break;
                    }
                }
            } else if cmd == "q" {
                break;
            } else {
                println!("?");
            }
        }
    }
}

// repl
// commands:
// add feed :: 1 -> a <url>
// get from all feeds :: 1 -> g
// delete feed :: 1 -> <number>d
// delete all feeds :: 1 -> ,d
// delete feed at current index :: 1 -> .d
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
