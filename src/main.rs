use std::net::IpAddr;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;


fn main() {
    // let contents = fs::read_to_string("hosts")
    //     .expect("Something went wrong reading the file");

    //let lines = contents.split('\n');
    //println!("With text:\n{}", contents);

    read_hosts().iter()
    .for_each(|(key, value)| {
        println!("{} - {}", key, value.host);
        value.addresses.iter().for_each(|ip| println!("- {}", ip.to_string()));
    } );
}

struct Host {
    host: String,
    addresses: Vec<IpAddr>
}

fn read_hosts() -> HashMap<String, Host> {
    let mut results = HashMap::new();
    if let Ok(lines) = read_lines("/etc/hosts") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if !ip.trim().starts_with("#") && !ip.trim().is_empty() {
                    let mut res = ip.trim().split_whitespace();
                    let address = res.next().unwrap();
                    let host = res.next().unwrap();
                    let inet = address.parse::<IpAddr>();
                    match inet {
                        Ok(inet) => { 
                            results.entry(host.to_string())
                                .or_insert_with(|| Host {host: host.to_string(), addresses: Vec::new()})
                                .addresses.push(inet)
                    },
                        Err(e) => {
                            println!("{} - {}", address, e.to_string());
                            continue;
                        }
                    };
                }
            }
        }
    }
    return results;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}