#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut result = vec![];

    for i in inputs.iter() {
        match i {
            CalculatorInput::Add => match (result.pop(), result.pop()) {
                (Some(a), Some(b)) => result.push(b + a),
                _ => return None,
            },
            CalculatorInput::Subtract => {
                let a = result.pop().unwrap();
                let b = result.pop().unwrap();
                result.push(b - a);
            }
            CalculatorInput::Multiply => {
                let a = result.pop().unwrap();
                let b = result.pop().unwrap();
                result.push(b * a);
            }
            CalculatorInput::Divide => {
                let a = result.pop().unwrap();
                let b = result.pop().unwrap();
                result.push(b / a);
            }
            CalculatorInput::Value(value) => result.push(*value),
        };
    }

    if result.len() > 1 {
        return None;
    }

    result.pop()
}
