use std::fmt;

#[derive(Debug, Copy, Clone)]
enum ExprParts {
    Int(i64),
    Double(f64),
    AddOp,
    SubOp,
    MulOp,
    DivOp,
}

impl ExprParts {
    pub fn is_op(&self) -> bool {
        match self {
            ExprParts::AddOp | ExprParts::SubOp | ExprParts::MulOp |
                ExprParts::DivOp => true,
            _ => false
        }
    }

    pub fn op_prio(&self) -> u8 {
        match self {
            ExprParts::SubOp => 1,
            ExprParts::AddOp => 2,
            ExprParts::MulOp => 3,
            ExprParts::DivOp => 4,
            _ => unreachable!() // should not happen 
        }
    }
}

impl fmt::Display for ExprParts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExprParts::Int(i) => write!(f, "{i}"),
            ExprParts::Double(d) => write!(f, "{d}"),
            ExprParts::AddOp => write!(f, "+"),
            ExprParts::SubOp => write!(f, "-"),
            ExprParts::MulOp => write!(f, "*"),
            ExprParts::DivOp => write!(f, "/")
        }
    }
}

pub fn calc (expr: &str) -> Result<(), String> {
    let mut index  = 0usize;
    let mut inter: Vec<ExprParts> = Vec::new();
    for i in expr.chars() {
        match i {
            ' ' => {},
            '+' => inter.push(ExprParts::AddOp),
            '-' => inter.push(ExprParts::SubOp),
            '*' => inter.push(ExprParts::MulOp),
            '/' => inter.push(ExprParts::DivOp),
            '0'..='9' => {
                let temp = &expr[index..find_next(expr, index)];
                if temp.contains(".") {
                    match temp.parse::<f64>() {
                        Ok(s) => inter.push(ExprParts::Double(s)),
                        Err(e) => return Err(format!("Could not parse {temp} as decimal value: {e}"))
                    }

                    continue;
                }

                match temp.parse::<i64>() {
                    Ok(s) => inter.push(ExprParts::Int(s)),
                    Err(e) => return Err(format!("Could not parse {temp} as integer value: {e}"))
                }
            }

            _ => return Err(format!("Invalid character {i}"))
        }
        index += 1;
    }

    println!("{:?}", inter);
    Ok(())
}

fn find_next(line: &str, from: usize) -> usize {
    let mut index = from;
    while index < line.len() {
        if !line.chars().nth(index).unwrap().is_digit(10) {
            return index;
        }

        index += 1;
    }

    line.len()
}

fn parse(tokens: &Vec<ExprParts>) -> Result<Vec<ExprParts>, String> {
    let mut parsed: Vec<ExprParts> = Vec::with_capacity(tokens.len());
    for i in 0..tokens.len() {
        if parsed.len() > 2 && parsed.len() % 3 == 0 {
            for i in 0..parsed.len() {
                if i % 3 == 0 {
                    if !tokens[i].is_op() {
                        return Err(format!("Expected operator but got  {}", tokens[i]));
                    }
                }
            }
        }

        if tokens[i].is_op() {
            if parsed.is_empty() {
                return Err(format!("Insufficient arguments for operator {}", tokens[i]));
            }

            parsed.insert(0, tokens[i]);
        }

        parsed.push(tokens[i]);
    }

    Ok(parsed)
}
