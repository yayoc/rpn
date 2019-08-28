extern crate rpn;
use std::io;

fn main() {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");
        println!("{:?}", rpn::RPN::calc(input));
    }
}
