use stream::Stream;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Token {
    OpenPar,
    ClosePar,
    OpenCur,
    CloseCur,
    Semi,
    Symbol(String),
    NumLiteral(String),
}

impl Token {
    pub fn get_symbol_string(self) -> Option<String> {
        match self {
            Token::Symbol(s) => Some(s),
            _ => None,
        }
    }
}

pub fn tokenize(s: &str) -> Vec<Token> {
    let mut tokens = vec![];
    let mut chars: Stream<char> = Stream::new(s.chars().collect());

    loop {
        if let Some(ch) = chars.peek() {
            if ch.is_whitespace() {
                chars.skip();
            } else if ch.is_alphabetic() {
                let mut tok = String::new();
                loop {
                    match chars.peek() {
                        Some(ch) if ch.is_alphanumeric() => {
                            chars.skip();
                            tok.push(ch);
                        }
                        _ => {
                            break;
                        }
                    }
                }
                assert!(tok.len() > 0);
                tokens.push(Token::Symbol(tok));
            } else if ch.is_numeric() {
                let mut tok = String::new();
                loop {
                    match chars.peek() {
                        Some(ch) if ch.is_numeric() => {
                            chars.skip();
                            tok.push(ch);
                        }
                        _ => {
                            break;
                        }
                    }
                }
                assert!(tok.len() > 0);
                tokens.push(Token::NumLiteral(tok));
            } else {
                match ch {
                    '(' => {
                        chars.skip();
                        tokens.push(Token::OpenPar);
                    }
                    ')' => {
                        chars.skip();
                        tokens.push(Token::ClosePar);
                    }
                    '{' => {
                        chars.skip();
                        tokens.push(Token::OpenCur);
                    }
                    '}' => {
                        chars.skip();
                        tokens.push(Token::CloseCur);
                    }
                    ';' => {
                        chars.skip();
                        tokens.push(Token::Semi);
                    }
                    _ => {
                        panic!("invalid character: {} at position: {}", ch, chars.get_pos());
                    }
                }
            }
        } else {
            break;
        }
    }

    return tokens;
}

#[test]
fn test_tokenizer() {
    let sourcecode: &str = "
		int main() {
			return 0;
		}
	";

    let exp_tokens = vec![
        Token::Symbol("int".to_string()),
        Token::Symbol("main".to_string()),
        Token::OpenPar,
        Token::ClosePar,
        Token::OpenCur,
        Token::Symbol("return".to_string()),
        Token::NumLiteral("0".to_string()),
        Token::Semi,
        Token::CloseCur,
    ];

    let tokens = tokenize(sourcecode);

    assert_eq!(&exp_tokens[..], &tokens[..]);
}
