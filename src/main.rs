use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "latex.pest"]
struct SnitParser;

fn main() {
    let success = SnitParser::parse(Rule::fraction, r"\frac{-254}{12}");
    println!("{:?}", success);
}
