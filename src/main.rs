use pratt::pratt::Parser;
use pratt::pratt::Token::*;

fn main() {
    let tokens = vec![Number(15), Plus, Number(6), Minus, Number(2), EOF];
    let mut parser = Parser::new(&tokens);
    let exp = parser.parse();

    println!("{}", exp.evaluate())
}
