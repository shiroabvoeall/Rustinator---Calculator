use std::io;
use std::io::Write;

fn main() {
    let mut dig_first = String::new();
    let mut dig_second = String::new();
    let mut dig_operator = String::new();

    print!("Enter your first digit: ");
    text_push();
    read_text(&mut dig_first);

    print!("Enter your second digit: ");
    text_push();
    read_text(&mut dig_second);

    print!("Enter your operator (+, -, *, /, %): ");
    text_push();
    read_text(&mut dig_operator);

    match calculate_num(&dig_first, &dig_second, &dig_operator) {
        Some(result) => println!("Result: {}", result),
        None => println!("Invalid input!"),
    }
}

fn calculate_num(firstn: &str, secondn: &str, dig_operator: &str) -> Option<i8> {
    let num1 = firstn.trim().parse::<i8>().ok()?;
    let num2 = secondn.trim().parse::<i8>().ok()?;

    match dig_operator.trim() {
        "+" => Some(num1 + num2),
        "-" => Some(num1 - num2),
        "*" => Some(num1 * num2),
        "/" => {
            if num2 != 0 {
                Some(num1 / num2)
            } else {
                println!("Error: Division by zero!");
                None
            }
        }
        "%" => {
            if num2 != 0 {
                Some(num1 % num2)
            } else {
                println!("Error: Modulo by zero!");
                None
            }
        }
        _ => None,
    }
}

fn text_push() {
    io::stdout().flush().expect("Error flushing stdout");
}

fn read_text(text_cont: &mut String) {
    text_cont.clear();
    io::stdin().read_line(text_cont).expect("Error reading input");
}