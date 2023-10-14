use base64::{engine::general_purpose, Engine as _};
use clap::Parser;
use std::fs::OpenOptions;
use std::io::stderr;
use std::io::{Read, Write};
use std::process::exit;

const URANDOM: &str = "/dev/urandom";

/// Password generator for Linux
#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    #[arg[short,long]]
    len: usize,
}

fn main() {
    let args = Args::parse();
    let mut buf = vec![0; args.len];
    if let Ok(f) = OpenOptions::new().read(true).open(URANDOM) {
        let a = f.take(args.len as u64).read(&mut buf);
        match a {
            Ok(value) => {
                if value != args.len {
                    let _ = stderr().lock().write_all(
                        format!("Read from {}: {} != {}", URANDOM, args.len, value).as_bytes(),
                    );
                    exit(1);
                };
            }
            Err(e) => {
                let _ = stderr().lock().write_all(format!("{}", e).as_bytes());
                exit(1);
            }
        }
    }
    let b64 = general_purpose::STANDARD.encode(&buf[0..(args.len as usize)]);
    println!("{}", &b64[0..args.len]);
}
