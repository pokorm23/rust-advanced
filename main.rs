use std::cell::RefCell;
use std::future::{Future};
use std::io::{self, BufRead};
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

struct Doubler { n: i32 }

impl Future for Doubler {
    type Output = i32;
    fn poll(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<i32> {
        Poll::Ready(self.n * 2)
    }
}

fn double(n: i32) -> Doubler {
    Doubler { n: n }
}

fn main() -> io::Result<()> {
    //let stdin = io::stdin();
    //let s: Vec<String> = stdin.lock().lines().collect::<Result<_,_>>()?;
    //let nums: Vec<i32> = s[0].split_whitespace().map(|s| s.parse().unwrap()).collect();

    let d = double(7);

    println!("{}", "created future");


    Ok(())
}