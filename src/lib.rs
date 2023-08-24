use pest::{Parser, iterators::Pair};
use pest_derive::Parser;
use calculator::{Operation, evaluator::evaluate};
use anyhow::Result;

#[derive(Parser)]
#[grammar = "latex.pest"]
struct LatexParser;

pub fn parse(equation: String) -> Result<Vec<Operation>> {
    let parsed = LatexParser::parse(Rule::equation, &equation)?.next().unwrap();
    handleparsed(parsed)

}

pub fn eval(equation: String) -> Result<f64> {
    evaluate(parse(equation)?)

}

fn handlerule(operation: Pair<'_, Rule>) -> Result<Vec<Operation>> {
    let mut ops = Vec::new();

    match operation.as_rule() {
        Rule::number =>  {
            let numstr: String = operation
                .as_str()
                .chars()
                .map(|x| if x == ',' { '.'} else { x })
                .collect();
            let number: f64 = numstr 
                .parse::<f64>()?;
            ops.push(Operation::Number(number));

        },
        Rule::multiply => ops.push(Operation::Multiply),
        Rule::plus => ops.push(Operation::Plus),
        Rule::exponent => ops.push(Operation::Exponent),
        Rule::minus => ops.push(Operation::Minus),
        Rule::openparen => ops.push(Operation::OpenParenthesis),
        Rule::closedparen => ops.push(Operation::ClosedParenthesis),
        Rule::raised_number => {
            ops.push(Operation::Exponent);
            let uuuh = parse_raised(operation.as_str())?;
            ops.push(Operation::Number(uuuh));
        },
        Rule::sqrt => {
            let mut inner_rules = operation.into_inner();

            let bla1 = inner_rules.next().unwrap();


            let mut first = handleparsed(bla1)?;

            ops.push(Operation::Sqrt);
            ops.push(Operation::OpenParenthesis);
            ops.append(&mut first);
            ops.push(Operation::ClosedParenthesis);

        },
       Rule::fraction => {
            let mut inner_rules = operation.into_inner();

            let bla1 = inner_rules.next().unwrap();

            let bla2 = inner_rules.next().unwrap();

            let mut first = handleparsed(bla1)?;
            let mut last = handleparsed(bla2)?;


            ops.push(Operation::OpenParenthesis);
            ops.append(&mut first);
            ops.push(Operation::ClosedParenthesis);
            ops.push(Operation::Divide);
            ops.push(Operation::OpenParenthesis);
            ops.append(&mut last);
            ops.push(Operation::ClosedParenthesis);

        },
        _ => unreachable!(),
    }
    Ok(ops)

}

fn handleparsed(equation: Pair<'_, Rule>) -> Result<Vec<Operation>> {

    let mut ops = Vec::new();

    for operation in equation.into_inner() {
        let mut bla = handlerule(operation)?;
        ops.append(&mut bla);
            
    }
    Ok(ops)
}

fn parse_raised(num: &str) -> Result<f64> {
    Ok(num.chars().map(|x| {
        match x {
            '¹' => '1',
            '²' => '2',
            '³' => '3',
            '⁴' => '4',
            '⁵' => '5',
            '⁶' => '6',
            '⁷' => '7',
            '⁸' => '8',
            '⁹' => '9',
            '⁰' => '0',
            _   => x,
        }
        }).collect::<String>().parse::<f64>()?)
}
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn parse_fraction() -> Result<()> {
        let latex = r"\frac{2}{3}".to_string();
        let out = vec![Operation::OpenParenthesis, Operation::Number(2.0), Operation::ClosedParenthesis, Operation::Divide, Operation::OpenParenthesis, Operation::Number(3.0), Operation::ClosedParenthesis];
        assert_eq!(parse(latex)?, out);
        Ok(())
    }

    #[test]
    fn parse_simple() -> Result<()> {
        let latex = r"2*\frac{2*2}{2}".to_string();
        let out = vec![Operation::Number(2.0), Operation::Multiply, Operation::OpenParenthesis, Operation::Number(2.0), Operation::Multiply, Operation::Number(2.0), Operation::ClosedParenthesis, Operation::Divide, Operation::OpenParenthesis, Operation::Number(2.0), Operation::ClosedParenthesis];
        assert_eq!(parse(latex)?, out);
        Ok(())
    }

    #[test]
    fn parse_exponent() -> Result<()> {
        let latex = "2^2".to_string();
        let out = vec![Operation::Number(2.0),Operation::Exponent,  Operation::Number(2.0)];
        assert_eq!(parse(latex)?, out);
        Ok(())
    }

    #[test]
    fn evaluate_sqrt() -> Result<()> {
        let latex = r"\sqrt{4}".to_string();
        assert_eq!(evaluate(parse(latex)?)?, 2.0);
        Ok(())
    }
    #[test]
    fn evaluate_frac() -> Result<()> {
        let latex = r"\frac{-5}{-2}".to_string();
        let parsed = parse(latex)?;
        println!("{:?}", parsed);
        assert_eq!(evaluate(parsed)?, 2.5);
        Ok(())
    }
    #[test]
    fn evaluate_full() -> Result<()> {
        let latex = r"2*\frac{2}{2}".to_string();
        assert_eq!(evaluate(parse(latex)?)?, 2.0);
        Ok(())
    }


}
