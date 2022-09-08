#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    use CalculatorInput::*;

    if inputs.len() == 0 {
        return None;
    }

    let mut rpn_stack: Vec<i32> = Vec::new();
    for item in inputs {
        match item {
            Value(x) => { 
                rpn_stack.push(*x) 
            },
            _ => {
                if rpn_stack.len() < 2 {
                    return None;    
                }

                let y = rpn_stack.pop().unwrap();
                let x = rpn_stack.pop().unwrap();

                let result;
                match item {
                    Add => { result = x + y; },
                    Subtract => { result = x - y; },
                    Multiply => { result = x * y; },
                    Divide => { result = x / y; },
                    _=> { return None; }
                };
                rpn_stack.push(result);
            }
        }
    }

    if rpn_stack.len() != 1 {
        return None;
    } 
    rpn_stack.pop()
}
