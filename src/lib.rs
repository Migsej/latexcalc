use pest::{Parser, iterators::Pair};
use pest_derive::Parser;
use calculator::Operation;
use anyhow::Result;

#[derive(Parser)]
#[grammar = "latex.pest"]
struct LatexParser;

pub fn parse(equation: String) -> Result<Vec<Operation>> {
    let parsed = LatexParser::parse(Rule::equation, &equation)?.next().unwrap();
    handleparsed(parsed)

}

pub fn handleparsed(equation: Pair<'_, Rule>) -> Result<Vec<Operation>> {

    let mut ops = Vec::new();

    for operation in equation.into_inner() {
        match operation.as_rule() {
            Rule::number =>  {
                let number: f64 = operation 
                    .as_str()
                    .parse::<f64>()?;
                ops.push(Operation::Number(number));

            },
            Rule::multiply => ops.push(Operation::Multiply),
            Rule::fraction => {
                let mut inner_rules = operation.into_inner();

                let bla1 = inner_rules.next().unwrap();

                let bla2 = inner_rules.next().unwrap();

                //TODO this is stupid
                let mut first = parse(bla1.as_str().to_string())?;
                let mut last = parse(bla2.as_str().to_string())?;

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
            
    }
    Ok(ops)
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
        let latex = r"2*\frac{2}{2}".to_string();
        let out = vec![Operation::Number(2.0), Operation::Multiply, Operation::OpenParenthesis, Operation::Number(2.0), Operation::ClosedParenthesis, Operation::Divide, Operation::OpenParenthesis, Operation::Number(2.0), Operation::ClosedParenthesis];
        assert_eq!(parse(latex)?, out);
        Ok(())
    }



}
