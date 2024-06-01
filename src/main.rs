use std::io::BufRead;
use std::{error::Error, io};
use rss::{Channel, Item};
use html2text::from_read;


async fn fetch_feed(feed: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(feed)
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

async fn get_title(url: &str) -> Option<String> {
    if let Ok(channel) = fetch_feed(url).await {
        let v_chan = channel.into_items();
        return Some(v_chan[0].title.clone()?)
    }
    None
}

fn process_item(item: &Item) -> Option<String> {
    let title = item.title.clone()?;
    let description = item.description.clone()?.replace("\n", "");
    let b_description = description.as_bytes();
    let fmt_description = from_read(b_description, 80);
    let display = format!(r#"
        {}
        ===============
        {}
    "#, title, fmt_description);
    Some(display)
}

fn process_range(range_str: String, store_len: usize) -> Option<(usize, usize)> {
    let mut src_dest = range_str.split(",");
    let src = src_dest.next()?;
    let dest = src_dest.next()?;
    if let Ok(start) = src.parse::<usize>() {
        if let Ok(end) = dest.parse::<usize>() {
            if start < end && end <= store_len - 1 {
                return Some((start, end))
            }
        }
    } 
    None
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

async fn cmd_p(url: String) {
    if let Some(title) = get_title(url.as_str()).await {
        println!("{}", title);
    } else {
        println!("?");
    }
}

#[tokio::main]
async fn main() {
    let mut store: Vec<(String, Channel)> = Vec::new();
    let mut current_line = 0;
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
            let mut current_addr = LineAddress::One(Single { addr: current_line });
            let line_address: Vec<char> = cmd.chars().clone().collect();
            let all_but_cmd = &line_address[..line_address.len()-1];
            let la_len = all_but_cmd.len();
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
            } else {
                let addrs = &all_but_cmd[..la_len-1];
                let addrs_str = String::from_iter(addrs.iter().clone());
                // there are two remaining types of line addressing that we want to handle:
                // 1. x,y such that x < y and x nor y exceed the bounds of the feed store
                // 2. numbers greater than 9
                if addrs_str.contains(",") {
                    if let Some((rg_start, rg_end)) = process_range(addrs_str, store.len()) {
                        current_addr = LineAddress::Many(Range { start: rg_start, end: rg_end });
                    } else {
                        println!("?");
                    }
                } else if let Ok(addrs_num) = addrs_str.parse::<usize>() {
                    if addrs_num < store.len() - 1 {
                        current_addr = LineAddress::One(Single { addr: addrs_num });
                    } else {
                        println!("?");
                    }
                } else {
                    println!("?");
                }
            }
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
            if let Some(cmd_la) = line_address.clone().pop() {
                if cmd_la == 'p' {
                    // it's a range, so we iterate
                    if addresses.len() > 1 {
                        for index in addresses[0]..=addresses[1] {
                            let (cur_url, _cur_chan) = store[index].to_owned();
                            cmd_p(cur_url).await;
                        }
                    } else {
                        let addr = addresses[0];
                        let (cur_url, _cur_chan) = store[addr].to_owned();
                        cmd_p(cur_url).await;
                    }
                } else if cmd_la == 'd' {
                    if addresses.len() > 1 {
                        for index in addresses[0]..=addresses[1] {
                            store.remove(index);
                        }
                    } else {
                        let addr = addresses[0];
                        store.remove(addr);
                    }
                    if store.len() - 1 < current_line {
                        current_line = store.len() - 1
                    } else {
                        println!("?");
                    }
                } else if cmd_la == 'o' {
                    let (_o_url, o_chan) = &store[current_line];
                    let mut item_store = Vec::new();
                    o_chan.items().into_iter().for_each(|item| item_store.push(item));
                    if addresses.len() > 1 {
                        for index in addresses[0]..=addresses[1] {
                            if let Some(pretty_item) = process_item(item_store[index]) {
                                println!("{}", pretty_item);
                            } else {
                                println!("?");
                                break;
                            }
                        }
                    } else {
                        let addr = addresses[0];
                        if let Some(pretty_item) = process_item(item_store[addr]) {
                            println!("{}", pretty_item);
                        } else {
                            println!("?");
                        }
                    }
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
