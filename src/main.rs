use std::env::args;

fn main() {
    // Get program arguments
    let mut args = args();

    // Pick first arg after binary path (which is skipped)
    let first = args.nth(1).unwrap();

    // Pick following args (iterator is resumed from last access)
    let operator = args.nth(0).unwrap();
    let second = args.nth(0).unwrap();

    // Parse number strings into floats (with "turbofish" help)
    let first_number = first.parse::<f32>().unwrap();
    let second_number = second.parse::<f32>().unwrap();

    println!("{} {} {}", first_number, operator, second_number);
}
