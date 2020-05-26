use std::env;

mod hosts;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut name = "/etc/hosts";
    
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        name = &args[1];
    }

    let data = hosts::read_hosts(name)?.into_iter().map(|(_, v)| v).collect::<Vec<hosts::Host>>();
    let ips = hosts::get_ips(data);

    hosts::print_free_ips(ips);
    Ok(())
}
