use std::{collections::HashMap, fs};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use chrono::{Datelike, Local, TimeZone};

struct Log {
    emerges: HashMap<i32, i32>,
    packages: HashMap<String, i32>,
}
fn main() {
    let res = copy();
    if let Err(e) = res {
        println!("Error: {}", e);
    }

    let entries = read();

    let log: Log = parse(entries);

    display(log.emerges, log.packages);
}

fn copy() -> std::io::Result<()> {
    fs::copy("/var/log/emerge.log", "/tmp/emerge.log").expect("could not copy");
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

fn parse(entries: Vec<String>) -> Log {
    let mut start = 0;
    let mut is_world = false;
    let mut emerges: HashMap<i32, i32> = HashMap::new();
    let mut packages: HashMap<String, i32> = HashMap::new();

    for entry in entries.iter() {
        let splits: Vec<&str> = entry.split(" ").collect();

        if entry.contains("Started") {
            start = splits[0][..splits[0].len()-1].parse::<i32>().unwrap();
        }
        else if entry.contains("@world") {
            is_world = true;
        }
        else if entry.contains("terminating") {
            let end = splits[0][..splits[0].len()-1].parse::<i32>().unwrap();
            let duration = end - start;

            if is_world {
                emerges.insert(start, duration);
            }

            is_world = false;
        }
        else if entry.contains(">>>") && entry.contains("emerge") {
            let package_name = splits[7];
            let mut clean_package_name = String::from("");
         
            for (i, c) in package_name.chars().enumerate() {
                if c == '-' && package_name.chars().nth(i+1).unwrap().is_digit(10) {
                    break;
                }

                clean_package_name.push(c);
            }

            *packages.entry(clean_package_name).or_insert(0) += 1;
        }
    }

    return Log {emerges: emerges, packages: packages};
}

fn display(timestamps: HashMap<i32, i32>, packages: HashMap<String, i32>) {
    let mut monthly_time: HashMap<String, i32> = HashMap::new();
    let mut monthly_count: HashMap<String, i32> = HashMap::new();

    for (start, duration) in timestamps {
        let readable = Local.timestamp(start as i64,0);

        *monthly_time.entry(format!("{0}/{1}", readable.month(), readable.year())).or_insert(0) += duration;
        *monthly_count.entry(format!("{0}/{1}", readable.month(), readable.year())).or_insert(0) += 1;
    }

    let mut count_vec: Vec<_> = monthly_time.iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1));

    println!("Monthly Total Time:");
    for entry in count_vec {
        println!("{}, {} times for {} hours", entry.0, monthly_count[entry.0], entry.1/3600);
    }

    let mut packages_counts: HashMap<String, i32> = HashMap::new();

    for (package_name, count) in packages {
        *packages_counts.entry(package_name).or_insert(0) += count;
    }

    let mut pkg_count_vec: Vec<_> = packages_counts.iter().collect();
    pkg_count_vec.sort_by(|a, b| b.1.cmp(a.1));

    println!("Top 20 Packages:");
    if pkg_count_vec.len() <= 20 {
        for entry in pkg_count_vec {
            println!("{} {}", entry.0, entry.1);
        }
    }
    else {
      
        for i in 0..20 {
            println!("{} {}", pkg_count_vec[i].0, pkg_count_vec[i].1);
        }
    }
}