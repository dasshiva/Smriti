use crate::engine::Engine;
use crate::expr::ExprParts;

pub fn calc (expr: &str) -> Result<ExprParts, String> {
    let mut index  = 0usize;
    let mut skip = 0;
    let mut inter: Vec<ExprParts> = Vec::new();
    for i in expr.chars() {
        if skip > 0 {
            skip -= 1;
            index += 1;
            continue;
        }
        match i {
            ' ' => {},
            '+' => inter.push(ExprParts::AddOp),
            '-' => inter.push(ExprParts::SubOp),
            '*' => inter.push(ExprParts::MulOp),
            '/' => inter.push(ExprParts::DivOp),
            '0'..='9' => {
                let temp = &expr[index..find_next(expr, index)];
                if temp.contains('.') {
                    match temp.parse::<f64>() {
                        Ok(s) => inter.push(ExprParts::Double(s)),
                        Err(e) => return Err(format!("Could not parse {temp} as decimal value: {e}"))
                    }
                    skip = temp.len() - 1;
                    continue;
                }

                match temp.parse::<i64>() {
                    Ok(s) => inter.push(ExprParts::Int(s)),
                    Err(e) => return Err(format!("Could not parse {temp} as integer value: {e}"))
                }

                skip = temp.len();
            }

            _ => return Err(format!("Invalid character {i}"))
        }
        println!("{:?}", inter);
        index += 1;
    }

    //println!("{:?}", inter);
    match check(&inter) {
        Ok(..) => {
            println!("{:?}", parse(&inter));
            Ok(Engine::new().eval(&parse(&inter)))
        },
        Err(e) => Err(e)
    }
}

fn find_next(line: &str, from: usize) -> usize {
    let mut index = from;
    while index < line.len() {
        let c = line.chars().nth(index).unwrap();
        if c == '.' {
            index += 1;
            continue;
        }
        if !c.is_ascii_digit() {
            return index;
        }

        index += 1;
    }

    line.len()
}

fn check(tokens: &Vec<ExprParts>) -> Result<(), String> {
    for i in 0..tokens.len() {
        if tokens[i].is_op() {
            if (i == tokens.len() - 1) || (i % 2 != 1) {
                return Err(format!("Expected constant here but got operator {}", tokens[i]));
            }

            if tokens[i - 1].is_op() || tokens[i + 1].is_op() {
                return Err("Two operators cannot occur one after the other".to_string());
            }
        }

        else if i % 2 != 0 {
            return Err(format!("Expected operator here but got constant {}", tokens[i]));
        }
    }
    Ok(())
}
fn parse(tokens: &Vec<ExprParts>) -> Vec<ExprParts> {
    let mut parsed: Vec<ExprParts> = Vec::with_capacity(tokens.len());
    let mut ind = 0;
    loop {
        if ind == tokens.len() {
            break;
        }
        if tokens[ind].is_op() {
            if parsed.len() == 1 {
                parsed.insert(0, tokens[ind]);
                ind += 1;
            }
            else {
                let s = find_more_prio(&parsed, tokens[ind], ind);
                parsed.insert(s, tokens[ind]);
                parsed.insert(s + 1, tokens[ind + 1]);
                ind += 2;
                continue;
            }
        }

        parsed.push(tokens[ind]);
        ind += 1;
    }

    parsed
}

fn find_more_prio(parsed: &Vec<ExprParts>, op: ExprParts, lim: usize) -> usize {
    for i in 0..parsed.len() {
        if parsed[i].is_op() && parsed[i].op_prio() > op.op_prio() {
            return i;
        }
    }

    lim - 1
}