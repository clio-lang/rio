mod lexer;

#[cfg(test)]
mod tests;

fn main() {
    let tokens = lexer::tokenize(&"0: 'asd' -> 1 => 2");

    for token in tokens {
        println!("{:?}", token)
    }
}
