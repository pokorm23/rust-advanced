use std::io::{self, BufRead};
use std::rc::Rc;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let s: Vec<String> = stdin.lock().lines().collect::<Result<_,_>>()?;
    let nums: Vec<i32> = s[0].split_whitespace().map(|s| s.parse().unwrap()).collect();

    let vec = Rc::new(nums);
    let a = Rc::clone(&vec);
    let b = Rc::clone(&vec);

    println!("count: {}", Rc::strong_count(&a));
    println!("sum: {}", a.iter().sum::<i32>());

    Ok(())
}