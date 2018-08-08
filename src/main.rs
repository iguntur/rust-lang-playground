mod calculator;

fn count(operator: char, x: f32, y: f32) {
    let result = calculator::calculate(operator, x, y);

    if operator == '+' {
        println!("add: {} + {} = {}", x, y, result);
    } else if operator == '*' {
        println!("multiply: {} * {} = {}", x, y, result);
    } else if operator == '-' {
        println!("subtract: {} - {} = {}", x, y, result);
    } else if operator == '/' {
        println!("divide: {} / {} = {}", x, y, result);
    }
}

fn main() {
    count('*', 10.0, 5.0);
    count('+', 42.0, 85.0);
    count('-', 101.0, 77.0);
    count('/', 22.0, 7.0);
}
