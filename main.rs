use std::cell::RefCell;
use std::io::{self, BufRead};
use std::rc::Rc;

macro_rules! sum {
    ($($x:expr),*) => {
        0 $(+$x)*
    };
}

fn main() -> io::Result<()> {
    /*let stdin = io::stdin();
    let s: Vec<String> = stdin.lock().lines().collect::<Result<_,_>>()?;
    let nums: Vec<i32> = s[0].split_whitespace().map(|s| s.parse().unwrap()).collect();*/

    println!("{}", sum!(1,2,3,4,5));

    Ok(())
}