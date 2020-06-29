mod lexer;

fn main() {
    let tokens = lexer::tokenize(&"if : else '123'");

    for token in tokens {
        println!("{:?}", token)
    }
}
