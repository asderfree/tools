use std::fs::{self, File, OpenOptions};
use std::io::{prelude::*, BufReader};

fn if_in_list(list: &mut Vec<String>, item:&mut String) -> bool {
    for i in list {
        if i == item {
            return true;
        }
    }
    return false;
}

fn check_file_exists(file_name: String) -> bool {
    match fs::metadata(file_name) {
        Ok(_) => return true,
        Err(_) => return false,
    }
}

fn read_all_contents(file:&mut File) -> Vec<String> {
    let mut contents: Vec<String> = Vec::new();
    let buf_reader = BufReader::new(file);
    for line in buf_reader.lines() {
        let mut line = line.expect("read failed");
        if !line.is_empty() && !if_in_list(&mut contents, &mut line) {
            contents.push(line);
        }
    }
    return contents;
}

fn write_to_file(file_name: &mut String, to_be_write: &mut Vec<String>) {
    // open the file with write permition.
    let f = OpenOptions::new()
        .create(true)
        .read(true)
        .append(true)
        .open(file_name.clone());
    let mut file = match f {
        Ok(file) => file,
        Err(_) => {
            println!("open file {} failed", file_name);
            return;
        },
    };
    // check if open ok.
    if !file.metadata().expect("metadata failed").is_file() {
        println!("{} is not a file", file_name);
        return;
    }
    // read all contents line by line from the file.
    let mut contents = read_all_contents(&mut file);
    // write to the file.
    // check if the file can be written.
    if file.metadata().expect("metadata failed").permissions().readonly() {
        println!("{} is readonly", file_name);
        return;
    }
    for i in to_be_write {
        if !if_in_list(&mut contents, i) {
            let s = format!("\n{}", i);
            file.write_all(s.as_bytes()).expect("write failed");
        }
    }
}

fn main() {
    // try to get the first argument.
    let mut args = std::env::args();
    let mut to_be_write ;
    // show args1
    match args.nth(1) {
        Some(arg) => {
            to_be_write = arg;
        },
        None => {
            println!("usage: anew file1 file2");
            return;
        },
    }
    let another_file;
    match args.next() {
        Some(arg) => another_file = arg,
        None => {
            println!("usage: anew file1 file2");
            return;
        },
    }
    // check if the file exists.
    if !check_file_exists(another_file.clone()) {
        println!("{} does not exist", another_file.clone());
        return;
    }
    println!("going to append {}'s unique lines to {}", another_file, to_be_write);
    // read another files contents:
    let mut file = File::open(another_file).expect("open failed");
    let mut contents = read_all_contents(&mut file);
    // close the file.
    drop(file);
    // write to the file.
    write_to_file(&mut to_be_write, &mut contents);
}
