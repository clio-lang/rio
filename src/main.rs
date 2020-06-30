mod lexer;
mod parser;

fn main() {
    let tokens = lexer::tokenize(&"-- fn double n:\n\tn * 2");
    let ast = parser::parse(tokens);

    println!("{:?}", ast)
}
