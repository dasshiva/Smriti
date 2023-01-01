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
    pub fn(&self) -> bool {
        match self {
            ExprParts::AddOp | ExprParts::SubOp | ExprParts::MulOp 
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

fn parse(tokens: &Vec<ExprParts>) {
    let mut parsed: Vec<ExprParts> = Vec::with_capacity(tokens.len());
    for token in tokens.iter() {

    }
}
