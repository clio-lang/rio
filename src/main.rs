fn main() {
    let tokens = tokenize("Hello World".to_string()).unwrap();
    println!("{:?}", tokens);
}

#[derive(Debug)]
struct Token {
    name: String,
    raw: String,
}

fn tokenize(string: String) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();

    let mut index = 0;

    'mainloop: while index < string.len() {
        let token = Token {
            name: "Foo".to_string(),
            raw: string.chars().nth(index).unwrap().to_string(),
        };
        tokens.push(token);
        index += 1;
    }

    Ok(tokens)
}
