extern crate threadpool;
extern crate ua_generator;
extern crate ureq;

use crate::types::Proxy;
use std::io::Write;
use std::sync::mpsc::channel;
use ua_generator::ua::spoof_ua;

#[derive(Debug)]
pub struct Checker {
    timeout: u32,
    retries: u32,
    verbose: bool,
    proxies: Vec<Proxy>,
    pool: threadpool::ThreadPool,
    tx: std::sync::mpsc::Sender<Option<Proxy>>,
    rx: std::sync::mpsc::Receiver<Option<Proxy>>,
}

impl Checker {
    pub fn new(
        proxies: Vec<Proxy>,
        threads: u32,
        timeout: u32,
        retries: u32,
        verbose: bool,
    ) -> Self {
        let (tx, rx) = channel();
        let pool = threadpool::ThreadPool::new(threads as usize);
        Checker {
            timeout,
            retries,
            verbose,
            proxies,
            pool,
            tx,
            rx,
        }
    }

    pub fn run<F: Fn(Proxy, u32, u32) -> bool + Send + Sync>(&self, check_func: &'static F) {
        for proxy in self.proxies.clone() {
            let tx = self.tx.clone();
            let timeout = self.timeout.clone();
            let retries = self.retries.clone();
            self.pool.execute(move || {
                if check_func(proxy.clone(), timeout, retries) {
                    tx.send(Some(proxy)).unwrap();
                } else {
                    tx.send(None).unwrap();
                }
            });
        }
    }

    pub fn get(&self) -> Vec<Proxy> {
        let mut proxies = Vec::new();
        for i in 0..self.proxies.len() {
            let proxy = self.rx.recv().unwrap();
            match proxy {
                Some(proxy) => {
                    if self.verbose {
                        println!("[{}] Found valid proxy: {}", i, proxy);
                    }
                    proxies.push(proxy);
                }
                None => {
                    if self.verbose {
                        println!("[{}] Invalid proxy", i);
                    }
                    continue;
                }
            }
            if !self.verbose {
                print!("\rChecked {} proxies", i + 1);
                std::io::stdout().flush().unwrap();
            }
        }
        if !self.verbose {
            println!();
        }
        proxies
    }

    pub fn run_httpbin(&self) -> Vec<Proxy> {
        self.run(&check_httpbin);
        self.get()
    }
}

fn configure_proxy(proxy: Proxy) -> String {
    if proxy.username.is_some() && proxy.password.is_some() {
        format!(
            "{}://{}:{}@{}:{}",
            proxy.protocol,
            proxy.username.unwrap(),
            proxy.password.unwrap(),
            proxy.host,
            proxy.port
        )
    } else {
        format!("{}://{}:{}", proxy.protocol, proxy.host, proxy.port)
    }
}

#[allow(dead_code)]
fn check_httpbin(proxy: Proxy, timeout: u32, retries: u32) -> bool {
    for _ in 0..retries {
        let proxy = ureq::Proxy::new(&configure_proxy(proxy.clone())).unwrap();
        let agent = ureq::AgentBuilder::new().proxy(proxy).build();
        let resp = agent
            .get("https://httpbin.org/ip")
            .set("User-Agent", spoof_ua())
            .set("Accept", "application/json")
            .set("Proxy-Connection", "Keep-Alive")
            .timeout(std::time::Duration::new(timeout as u64, 0))
            .call();
        match resp {
            Ok(resp) => match resp.into_string() {
                Ok(body) => {
                    if body.contains("origin") {
                        return true;
                    }
                }
                Err(_) => continue,
            },
            Err(_) => continue,
        }
    }
    false
}
