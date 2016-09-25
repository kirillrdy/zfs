use std::process::Command;

extern crate regex;

fn main() {
    let output = Command::new("zfs").arg("list").output().unwrap().stdout;
    println!("{:?}", output);
}
