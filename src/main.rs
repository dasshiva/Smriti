use std::env;

mod calc;
fn main() {
    let mut buf = String::new();
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    for i in args.iter() {
        buf += &i.trim();
    }
    match calc::calc(&buf) {
        Ok(..) => {},
        Err(s) => { eprintln!("{s}"); std::process::exit(1) }
    }
}
