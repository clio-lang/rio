extern crate regex;
use regex::Regex;

fn main() {
    let tokens = tokenize("hello 'World' 1 23 345".to_string()).unwrap();

    for token in tokens {
        println!("{:?}", token);
    }
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

        if let Some(m) = Regex::new(r#"^'([^\\]|\\.)*?'|^"([^\\]|\\.)*?"#)
            .unwrap()
            .captures(&rest)
        {
            let token = Token {
                name: "string".to_string(),
                raw: m.get(0).unwrap().as_str().to_string(),
            };
            index += token.raw.len();
            println!("{:?}", token.raw.len());
            tokens.push(token);
        } else if let Some(m) = Regex::new(r#"^(0|-?[1-9][0-9']*)(n|(\.[0-9']+))?"#)
            .unwrap()
            .captures(&rest)
        {
            let token = Token {
                name: "number".to_string(),
                raw: m.get(0).unwrap().as_str().to_string(),
            };
            index += token.raw.len();
            tokens.push(token);
        } else {
            index += 1;
        }
        continue 'mainloop;
    }

    Ok(tokens)
}
