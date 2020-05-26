use std::env;
use std::net::IpAddr;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut name = "/etc/hosts";
    
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        name = &args[1];
    }

    read_hosts(name)?.iter()
    .for_each(|(_, value)| {
        println!("{}", value.host);
        value.addresses.iter().for_each(|ip| println!("- {}", ip.to_string()));
    } );
    Ok(())
}

struct Host {
    host: String,
    addresses: Vec<IpAddr>
}

fn read_hosts(file: &str) -> Result<HashMap<String, Host>, io::Error> {
    let mut results = HashMap::new();
    let lines = read_lines(file)?;
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

    return Ok(results);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}