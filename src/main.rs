#![allow(dead_code,unused_imports,unused_must_use,unused_assignments)]

extern crate dns;
extern crate time;

use std::io::net::udp::UdpSocket;
use std::io::net::ip::{Ipv4Addr, SocketAddr};
use std::io::BufReader;
use std::io;
use std::rand;
use std::sync::{Arc,Mutex};

//use dns::msg::{DNSMessageReader,Message};
//use dns::msg::record::{Question,ResourceRecord};
//use dns::number;

use dns::hp::read_dns_message;

fn main() {
    println!("===================");
    println!("Rust DNS Benchmarks");
    println!("===================\n");
    let buf = include_bin!("packets/net1-rs.bin");
    bench("net1", buf);
    let buf = include_bin!("packets/comns1-rs.bin");
    bench("comns1", buf);
    let buf = include_bin!("packets/fb1-rq.bin");
    bench("fb1-rq", buf);
    let buf = include_bin!("packets/fb1-rs.bin");
    bench("fb1-rs", buf);
}

fn bench(s: &str, buf: &[u8]) {
    print!("Parse {} ({} bytes) sample packet 1,000,000 times..", s, buf.len()); io::stdio::flush();
    let start = time::precise_time_ns();
    for i in range(0, 1000000i) {
        let buf2 = buf.clone();
        let m = read_dns_message(buf2).ok().unwrap();
        m.id;
        i + 1;
    }
    let end = time::precise_time_ns();
    let elapsed = (end - start) as f64 / 1000000000.;
    println!("..completed in {}s ({}s/byte)", elapsed, elapsed / buf.len() as f64);
}
