use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;
use std::net::IpAddr;

fn main() {
    let f = File::open("hello.txt");

    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            other_error => panic!("There was a problem opening the file: {:?}", other_error),
        },
    };

    parse_ip_addr();
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn parse_ip_addr() {
    let home: IpAddr = "127.0.0.1".parse().unwrap();

    println!("{:?}", home);
}
