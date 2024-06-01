use std::io::BufRead;
use std::{error::Error, io};
use rss::Channel;


async fn fetch_feed(feed: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(feed)
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    //println!("{:#?}", channel);
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
#[derive(Copy, Clone)]
struct Single {
    addr: usize
}
#[derive(Copy, Clone)]
struct Range {
    start: usize,
    end: usize
}

#[derive(Copy, Clone)]
enum LineAddress {
    One(Single),
    Many(Range)
}

#[tokio::main]
async fn main() {
    let mut store: Vec<(String, Channel)> = Vec::new();
    let mut current_line = 0;
    let mut current_addr = LineAddress::One(Single { addr: 0 }); // TODO: this isn't mutating
    loop {
        // read user input
        let mut user_input = String::new();
        let stdin = io::stdin();
        stdin.lock().read_line(&mut user_input).unwrap();
        let tkns: Vec<&str> = user_input.split(' ').collect();
        // command is the first token, many args may follow
        let cmd = tkns[0].trim();
        let cmd_len = cmd.len();
        // two branches: 
        // 1. commands that require line addressing,
        // 2. and those that don't
        if cmd_len > 1 {
            let line_address: Vec<char> = cmd.chars().clone().collect();
            let all_but_cmd = &line_address[..line_address.len()-1];
            let la_len = all_but_cmd.len();
            println!("{}", la_len);
            // two types of symbols for line addressing:
            // 1. single character
            // 2. multiple characters (a range or a number larger than 9)
            if la_len == 1 {
                let addr = all_but_cmd[0];
                let bounds = if store.len() == 0 { 0 } else { store.len() - 1 } ;
                current_addr = match addr {
                    '.' => LineAddress::One(Single { addr: current_line }),
                    '$' => LineAddress::One(Single { addr: bounds }),
                    ';' => LineAddress::Many(Range { start: current_line, end: bounds }),
                    ',' => LineAddress::Many(Range { start: 0, end: bounds }),
                    _ => current_addr
                };
            }
            if let Some(cmd_la) = line_address.clone().pop() {
                if cmd_la == 'p' {
                    let addresses = match current_addr.clone() {
                        LineAddress::One(state) => {
                            let mut v = Vec::new();
                            v.push(state.addr);
                            v
                        },
                        LineAddress::Many(state) => {
                            let mut v = Vec::new();
                            v.push(state.start);
                            v.push(state.end);
                            v
                        }
                    };
                    // it's a range, so we iterate
                    if addresses.len() > 1 {
                        for index in addresses[0]..=addresses[1] {
                            let (cur_url, _cur_chan) = store[index].to_owned();
                            if let Some(title) = get_title(cur_url.as_str()).await {
                                println!("{}", title);
                            } else {
                                println!("?");
                            }
                        }
                    } else {
                        let addr = addresses[0];
                        let (cur_url, _cur_chan) = store[addr].to_owned();
                        if let Some(title) = get_title(cur_url.as_str()).await {
                            println!("{}", title);
                        } else {
                            println!("?");
                        }
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
                if let Ok(chan) = fetch_feed(&url).await {
                    let bytes = chan.items().len();
                    store.push((url, chan));
                    println!("{}", bytes);
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
            } else if let Ok(n) = cmd.to_string().parse::<usize>() {
                current_line = n;
                if n > store.len() {
                    println!("?");
                } else {
                    let (cl_url, _cl_chan) = store[current_line].clone();
                    println!("{}", cl_url);
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
