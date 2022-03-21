use std::env::args;

fn operate(first: f32, operator: char, second: f32) -> f32 {
    match operator {
        '+'         => first + second,
        '-'         => first - second,
        '*'|'X'|'x' => first * second,
        '/'         => first / second,
        _ => panic!("Invalid operator used.")
    }
}

fn main() {
    // Get program arguments
    let mut args = args();
    if args.len() != 4 {
        println!("Usage: calculator FIRST OPERATOR SECOND");
        println!("   Ex: calculator 3 + 5");
        return
    }
    // Pick first arg after binary path (which is skipped)
    let first_str = args.nth(1).unwrap();

    // Pick following args (iterator is resumed from last access)
    let operator_str = args.next().unwrap();
    let second_str = args.next().unwrap();

    // Parse number strings into floats (with "turbofish" help);
    // also parse operator string into single char.
    let first = first_str.parse::<f32>().unwrap();
    let operator = operator_str.chars().next().unwrap();
    let second = second_str.parse::<f32>().unwrap();

    // Calculate and print result
    let result = operate(first, operator, second);
    println!("{} {} {} = {}", first, operator, second, result);
}
