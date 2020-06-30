#[cfg(test)]
mod tests {
    use crate::lexer::*;

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

    #[test]
    fn test_booleans() {
        let mut tokens = tokenize("true false");

        assert_eq!(
            tokens.nth(0).unwrap(),
            Token {
                len: 4,
                kind: TokenKind::Identifier {
                    kind: IdentifierKind::Boolean
                },
                raw: "true".to_owned()
            }
        );

        assert_eq!(
            tokens.nth(1).unwrap(),
            Token {
                len: 5,
                kind: TokenKind::Identifier {
                    kind: IdentifierKind::Boolean
                },
                raw: "false".to_owned()
            }
        );
    }

    #[test]
    fn test_functions() {
        let mut tokens = tokenize("fn fib n:");

        assert_eq!(
            tokens.nth(0).unwrap(),
            Token {
                len: 2,
                kind: TokenKind::Identifier {
                    kind: IdentifierKind::Function
                },
                raw: "fn".to_owned()
            }
        );
    }

    #[test]
    fn test_comments() {
        let mut tokens = tokenize(
            "
        -- foo
        fn fib n:
        ",
        )
        .filter(|t| {
            t.kind != TokenKind::Whitespace
                && t.kind != TokenKind::Tab
                && t.kind != TokenKind::CarriageReturn
        });

        assert_eq!(
            tokens.nth(0).unwrap(),
            Token {
                len: 6,
                kind: TokenKind::Comment,
                raw: "-- foo".to_owned(),
            }
        );

        assert_eq!(
            tokens.nth(0).unwrap(),
            Token {
                len: 2,
                kind: TokenKind::Identifier {
                    kind: IdentifierKind::Function
                },
                raw: "fn".to_owned(),
            }
        );
    }
}
