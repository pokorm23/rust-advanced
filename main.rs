use std::cell::RefCell;
use std::io::{self, BufRead};
use std::rc::Rc;

trait Shape {fn area(&self) -> f64;}
struct Square {side:f64}
struct Triange {base:f64, height: f64}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side.powi(2)
    }
}

impl Shape for Triange {
    fn area(&self) -> f64 {
        self.base * self.height * 0.5
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let s: Vec<String> = stdin.lock().lines().collect::<Result<_,_>>()?;
    //let nums: Vec<i32> = s[0].split_whitespace().map(|s| s.parse().unwrap()).collect();

    let mut shapes: Vec<Box<dyn Shape>> = vec![];

    shapes.push(Box::new(Square {side: 3.0}));
    shapes.push(Box::new(Triange {base: 4.0, height:5.0}));

    for n in &shapes {
        println!("{:.2}", n.area());
    }


    Ok(())
}