mod lexer;

fn main() {
    let tokens = lexer::tokenize(&"fn double n: n * 2");

    for token in tokens {
        println!("{:?}", token)
    }
}
