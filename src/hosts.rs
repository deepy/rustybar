use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

pub struct Host {
    pub host: String,
    pub addresses: Vec<IpAddr>
}

fn one_larger(a: Ipv4Addr, b: Ipv4Addr) -> bool {
    return (u32::from(a) + 1) == u32::from(b)
}

fn one_larger_v6(a: Ipv6Addr, b: Ipv6Addr) -> bool {
    return (u128::from(a) + 1) == u128::from(b)
}

fn print_free(start: IpAddr, end: IpAddr) {
    match (start, end) {
        (IpAddr::V4(start), IpAddr::V4(end)) => {
            let a = start.octets();
            let b = end.octets();
            if start.octets()[0] != end.octets()[0] {
                println!("{} -> {}.{}.{}.{}", start, b[0], b[1], b[2], b[3])
            } else if start.octets()[1] != end.octets()[1] {
                println!("{}-> {}.{}.{}", start, b[1], b[2], b[3])
            } else if start.octets()[2] != end.octets()[2] {
                println!("{} -> {}.{}", start, b[2],  b[3])
            } else if start.octets()[3] != end.octets()[3] {
                println!("{} -> {}", start, b[3])
            }
        },
        (IpAddr::V6(start), IpAddr::V6(end)) => {
            println!("{} -> {}", start, end)
        },
        (IpAddr::V4(start), IpAddr::V6(end)) => {},
        (IpAddr::V6(start), IpAddr::V4(end)) => {}
    }
}

pub fn print_free_ips(data: Vec<IpAddr>) {
    let mut start: Option<&IpAddr> = None;
    let mut iter = data.iter().peekable();
    while let Some(current) = iter.next() {
        // println!("ip: {}", current);
        if start.is_none() { start = Some(current); }
        if start == Some(current) { continue }
        if let Some(&next) = iter.peek() {
            match (current, next) {
                (IpAddr::V4(cur), IpAddr::V4(nex)) => {
                    if one_larger(*cur, *nex) {
                        continue
                    } else { 
                        print_free(*start.unwrap(), *current);
                        start = Some(&*next);
                    }
                },
                (IpAddr::V6(cur), IpAddr::V6(nex)) => {
                    
                },
                (IpAddr::V4(cur), IpAddr::V6(nex)) => {
                    print_free(*start.unwrap(), *current);
                    start = Some(&*next);
                },
                (IpAddr::V6(cur), IpAddr::V4(nex)) => {
                    continue
                }
            }
        }
    }
}

pub fn get_ips(data: Vec<Host>) -> Vec<IpAddr> {
    let mut ips = data.iter()
        .flat_map(|h| h.addresses.clone())
        .filter(|i| match i {
            IpAddr::V4(ipv4) => { !ipv4.is_loopback() && ipv4.is_private() }
            IpAddr::V6(ipv6) => { !ipv6.is_loopback() }
        })
        .collect::<Vec<IpAddr>>();
    ips.sort_unstable();
    return ips;
}

pub fn read_hosts(file: &str) -> Result<HashMap<String, Host>, io::Error> {
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

    Ok(results)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}