#[cfg(test)]
mod tests {
    use crate::*;
    use lexer::*;

    #[test]
    fn test_basic_tokenizing() {
        let mut tokens = tokenize("1 = 2");

        assert_eq!(
            tokens.nth(0).unwrap(),
            Token {
                len: 1,
                kind: TokenKind::Literal {
                    kind: LiteralKind::Int
                },
                raw: "1".to_owned()
            }
        );

        assert_eq!(
            tokens.nth(0).unwrap(),
            Token {
                len: 1,
                kind: TokenKind::Whitespace,
                raw: " ".to_owned()
            }
        );

        assert_eq!(
            tokens.nth(0).unwrap(),
            Token {
                len: 1,
                kind: TokenKind::Equals,
                raw: "=".to_owned()
            }
        );

        assert_eq!(
            tokens.nth(0).unwrap(),
            Token {
                len: 1,
                kind: TokenKind::Whitespace,
                raw: " ".to_owned()
            }
        );

        assert_eq!(
            tokens.nth(0).unwrap(),
            Token {
                len: 1,
                kind: TokenKind::Literal {
                    kind: LiteralKind::Int
                },
                raw: "2".to_owned()
            }
        );
    }
}
