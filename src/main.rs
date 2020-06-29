mod lexer;

fn main() {
    let tokens = lexer::tokenize(&"ifa : else '123'");

    for token in tokens {
        println!("{:?}", token)
    }
}
