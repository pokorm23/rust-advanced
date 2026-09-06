use std::cell::RefCell;
use std::io::{self, BufRead};
use std::rc::Rc;

fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    }
    else {
        b
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let s: Vec<String> = stdin.lock().lines().collect::<Result<_,_>>()?;
    //let nums: Vec<i32> = s[0].split_whitespace().map(|s| s.parse().unwrap()).collect();

    let a = &s[0];
    let b = &s[1];

    println!("{}", longer(a,b ));

    Ok(())
}