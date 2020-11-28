use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let res = copy();
    if let Err(e) = res {
        println!("Error: {}", e);
    }

    let entries = read();

    for line in entries.iter() {
        println!("{}", line);
    }
}

fn copy() -> std::io::Result<()> {
    fs::copy("/var/log/emerge.log", "/tmp/emerge.log");
    println!("copied log to /tmp");
    Ok(())
}

fn read() -> Vec<String> {
    let mut v: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines("/tmp/emerge.log") {
        for line in lines {
            if let Ok(entry) = line {
                v.push(entry);
            }
        }
    }

    return v;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}