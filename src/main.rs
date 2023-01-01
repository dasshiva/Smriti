use std::env;

mod calc;
mod engine;
mod expr;

fn main() {
    let mut buf = String::new();
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    for i in args.iter() {
        buf += &i.trim();
    }
    match calc::calc(&buf) {
        Ok(s) => { println!("{s}")},
        Err(s) => { eprintln!("In expression: {buf}\nError:{s}"); std::process::exit(1) }
    }
}
