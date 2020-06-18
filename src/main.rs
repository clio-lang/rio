extern crate regex;
use regex::Regex;

fn main() {
    let tokens = tokenize("hello 'World'".to_string()).unwrap();
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
        let rest: String = string.chars().skip(index).collect();

        println!("Rest: {:?}", rest);
        if let Some(m) = Regex::new(r#"^'([^\\]|\\.)*?'|^"([^\\]|\\.)*?"#)
            .unwrap()
            .captures(&rest)
        {
            let token = Token {
                name: "string".to_string(),
                raw: m.get(0).unwrap().as_str().to_string(),
            };
            tokens.push(token);
        }
        index += 1;
        continue 'mainloop;
    }

    Ok(tokens)
}
