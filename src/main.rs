use latexcalc::eval;
use std::io;

fn main() {
    loop {
        let mut equation = String::new();
        io::stdin().read_line(&mut equation).expect("Failed to read line");
        let evaluated = eval(equation);
        match evaluated {
            Ok(n) => println!("{:?}", n),
            Err(s) => println!("{}", s),
        }

    } 

}
