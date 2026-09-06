use std::cell::RefCell;
use std::io::{self, BufRead};
use std::rc::Rc;

fn main() -> io::Result<()> {
    /*let stdin = io::stdin();
    let s: Vec<String> = stdin.lock().lines().collect::<Result<_,_>>()?;
    let nums: Vec<i32> = s[0].split_whitespace().map(|s| s.parse().unwrap()).collect();*/

    let counter = Rc::new(RefCell::new(0));

    for _ in 0..3 {
        let a = Rc::clone(&counter);
        *a.borrow_mut() += 1;
    }

    println!("{}", *counter.borrow());

    Ok(())
}