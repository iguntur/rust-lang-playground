fn multiply(x: f32, y: f32) -> f32 {
    x * y
}

fn add(x: f32, y: f32) -> f32 {
    x + y
}

fn subtract(x: f32, y: f32) -> f32 {
    x - y
}

fn divide(x: f32, y: f32) -> f32 {
    x / y
}

pub fn calculate(operator: char, x: f32, y: f32) -> f32 {
    if operator == '+' {
        return add(x, y);
    } else if operator == '*' {
        return multiply(x, y);
    } else if operator == '-' {
        return subtract(x, y);
    } else if operator == '/' {
        return divide(x, y);
    } else {
        return 0 as f32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_multiply_correctly() {
        assert_eq!(multiply(5.0, 4.0), 20.0);
    }

    #[test]
    fn it_should_adder_correctly() {
        assert_eq!(add(5.0, 4.0), 9.0);
    }

    #[test]
    fn it_should_subtract_correctly() {
        assert_eq!(subtract(5.0, 4.0), 1.0);
    }

    #[test]
    fn it_should_divide_correctly() {
        assert_eq!(divide(22.0, 7.0), 3.142857);
    }
}
