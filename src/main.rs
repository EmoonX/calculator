use std::env::args;

fn main() {
    // Get program arguments
    let mut args = args();

    // Pick first arg after binary path (which is skipped)
    let first = args.nth(1).unwrap();

    // Pick following args (iterator is resumed from last access)
    let operator = args.nth(0).unwrap();
    let second = args.nth(0).unwrap();

    println!("{} {} {}", first, operator, second);
}
