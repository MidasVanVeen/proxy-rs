use clap::Parser;
use proxy_rs::cli::Cli;
use proxy_rs::types::{Protocol, Proxy};

fn main() {
    let args = Cli::parse();
    // read proxy list from cli.file
    let proxies = std::fs::read_to_string(args.file)
        .unwrap()
        .lines()
        .map(|line| {
            let mut parts = line.split(':');
            let protocol = parts.next().unwrap();
            let host = String::from(parts.next().unwrap());
            let port = parts
                .next()
                .unwrap()
                .parse::<u16>()
                .expect("Could not parse port");
            let username = match parts.next() {
                Some(username) => Some(String::from(username)),
                None => None,
            };
            let password = match parts.next() {
                Some(password) => Some(String::from(password)),
                None => None,
            };
            Proxy {
                host,
                port,
                protocol: Protocol::from_str(protocol),
                username,
                password,
            }
        })
        .collect::<Vec<Proxy>>();
    // check proxies
    let checker = proxy_rs::checker::Checker::new(
        proxies,
        args.threads,
        args.timeout,
        args.retries,
        args.verbose,
    );
    let proxies = checker.run_httpbin();
    println!("Found {} working proxies", proxies.len());
    // write proxies to cli.outfile
    let mut file = std::fs::File::create(args.outfile).unwrap();
    for proxy in proxies {
        let line = proxy.to_string() + "\n";
        std::io::Write::write_all(&mut file, line.as_bytes()).unwrap();
    }
}
