//! This is where old code that is no longer used but I don't want to delete goes.
//! These do not work on their own, they are only reference.

// Check mimetype for TOML
pub fn check_mimetype () {

        let content_type: String = match response.headers.get::<hyper::header::ContentType>() {
            Some(v) => Option::Some(format!("{}", v)),
            None => return Error::InvalidFileError
        }.unwrap();
        if content_type != "text/plain; charset=utf-8" && content_type != "text/x-toml; charset=utf-8" {
            return Error::InvalidFileError;
        }
}

pub fn download_old () {
    
    let mut result: String = String::new();

    let mut handle = curl::easy::Easy::new();
    handle.url(url).unwrap();
    handle.progress(true).unwrap();
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|data| {

            result = match std::str::from_utf8(data).map(|v| v.to_string());
            Ok(data.len());

        }).unwrap();

        match transfer.perform() {
            Ok(_) => ();
            Err(e) => return e;
        };
    }

    if &result.is_ok()
        let parsed: Repository = toml::from_str(&result);
    
    return Ok(parsed);
}

fn symlink() {

extern crate toml;
extern crate ansi_term;
extern crate users;

use std::io::prelude::*;
use std::env;
use std::fs;
use std::mem;
use std::os::unix;

use ansi_term::Style;

fn dont_return(a: toml::Value) {
    return
}

fn open_package(filename: &String) -> toml::Value {

    // Read file from disk
    let file = fs::File::open(filename);
    let mut contents = String::new();
    file.unwrap().read_to_string(&mut contents);

    // Parse TOML
    let parsed = contents.parse::<toml::Value>().unwrap();
    println!("---------");
    println!(">> {}", Style::new().bold().paint(parsed["package"]["name"].as_str().unwrap()));
    println!("{}", parsed["package"]["description"].as_str().unwrap());
    println!("---------");

    return parsed;
}

fn get(filename: &String) {

    if users::get_user_by_uid(users::get_current_uid()).unwrap().uid() != 0 {
        println!("Installing software requires root privileges.");
        return
    }

    let package_file: toml::Value = open_package(filename);

    for binary in package_file["package"]["binaries"].as_array().unwrap() {
        let src = &binary.as_str().unwrap();
        let dest = format!("/usr/bin/{}", &binary.as_str().unwrap());
        unix::fs::symlink(&src, &dest);
        println!("Symlinked '{}' to '{}'", &src, &dest);
    }
}

fn trash(package: &String) {
    return
}

fn list() {
    return
}

fn parse_args() {

    let args: Vec<String> = env::args().collect();

    match args[1].as_ref() {
        "get" => get(&args[2]),
        "trash" => trash(&args[2]),
        "info" => dont_return(open_package(&args[2])),
        "list" => list(),
        x => println!("'{}' is not a valid command", x)
    }
}

fn main() {
    parse_args();
}

}
