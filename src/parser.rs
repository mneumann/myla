use ast;
use tokenizer::{Keyword, Token};

#[derive(PartialEq, Eq, Debug)]
pub struct ParseError {
    reason: String,
}

pub struct TokenStream<'a> {
    tokens: &'a [Token],
    current_pos: usize,
}

impl<'a> TokenStream<'a> {
    fn new(tokens: &'a [Token]) -> Self {
        Self {
            tokens,
            current_pos: 0,
        }
    }

    fn next(&mut self) -> Option<Token> {
        if self.is_exhausted() {
            return None;
        } else {
            let token = self.tokens[self.current_pos].clone();
            self.current_pos += 1;
            return Some(token);
        }
    }

    fn expect_next(&mut self) -> Result<Token, ParseError> {
        self.next().ok_or(ParseError {
            reason: "premature end".to_string(),
        })
    }

    fn expect_next_eql(&mut self, exp: Token) -> Result<(), ParseError> {
        let tok = self.expect_next()?;
        if tok != exp {
            Err(ParseError {
                reason: format!("expected token: {:?}. actual: {:?}", exp, tok),
            })
        } else {
            Ok(())
        }
    }

    fn current(&mut self) -> Option<Token> {
        if self.is_exhausted() {
            return None;
        } else {
            let token = self.tokens[self.current_pos].clone();
            return Some(token);
        }
    }

    fn is_exhausted(&self) -> bool {
        self.current_pos >= self.tokens.len()
    }
}

pub fn parse_expr(token_stream: &mut TokenStream) -> Result<ast::Expr, ParseError> {
    let tok = token_stream.next().ok_or(ParseError {
        reason: "exhaused parse_expr".to_string(),
    })?;
    match tok {
        Token::NumLiteral(s) => Ok(ast::Expr::NumLiteral(s)),
        _ => Err(ParseError {
            reason: "parse_expr: Expect NumLiteral".to_string(),
        }),
    }
}

pub fn parse_statement(token_stream: &mut TokenStream) -> Result<ast::Statement, ParseError> {
    let _ = token_stream.expect_next_eql(Token::Keyword(Keyword::Return))?;
    let expr = parse_expr(token_stream)?;
    let _ = token_stream.expect_next_eql(Token::Semi)?;
    return Ok(ast::Statement::Return(Box::new(expr)));
}

pub fn parse_block(token_stream: &mut TokenStream) -> Result<ast::Block, ParseError> {
    let _ = token_stream.expect_next_eql(Token::OpenCur)?;
    let mut statements = Vec::new();
    loop {
        match token_stream.current().ok_or(ParseError {
            reason: "Premature end".to_string(),
        })? {
            Token::CloseCur => {
                break;
            }
            _ => {
                let stmt = parse_statement(token_stream)?;
                statements.push(stmt);
            }
        }
    }

    let _ = token_stream.expect_next_eql(Token::CloseCur)?;

    Ok(ast::Block { statements })
}

pub fn parse_function(token_stream: &mut TokenStream) -> Result<ast::Function, ParseError> {
    let return_typename = token_stream
        .expect_next()?
        .get_ident_string()
        .ok_or(ParseError {
            reason: "invalid return typename type".to_string(),
        })?;
    let function_name = token_stream
        .expect_next()?
        .get_ident_string()
        .ok_or(ParseError {
            reason: "invalid function name type".to_string(),
        })?;

    let _ = token_stream.expect_next_eql(Token::OpenPar)?;
    let _ = token_stream.expect_next_eql(Token::ClosePar)?;
    let block = parse_block(token_stream)?;

    Ok(ast::Function {
        return_type: ast::Type {
            name: return_typename,
        },
        name: function_name,
        parameters: Vec::new(),
        block,
    })
}

pub fn parse_program(token_stream: &mut TokenStream) -> Result<ast::Program, ParseError> {
    let mut functions = vec![];

    while !token_stream.is_exhausted() {
        functions.push(parse_function(token_stream)?);
    }

    return Ok(ast::Program { functions });
}

#[test]
fn test_parser() {
    use tokenizer::Keyword;
    let tokens = vec![
        Token::Ident("int".to_string()),
        Token::Ident("main".to_string()),
        Token::OpenPar,
        Token::ClosePar,
        Token::OpenCur,
        Token::Keyword(Keyword::Return),
        Token::NumLiteral("0".to_string()),
        Token::Semi,
        Token::CloseCur,
    ];

    let exp_ast = ast::Program {
        functions: vec![
            ast::Function {
                return_type: ast::Type {
                    name: "int".to_string(),
                },
                name: "main".to_string(),
                parameters: vec![],
                block: ast::Block {
                    statements: vec![
                        ast::Statement::Return(Box::new(ast::Expr::NumLiteral("0".to_string()))),
                    ],
                },
            },
        ],
    };

    let mut token_stream = TokenStream::new(&tokens[..]);
    let ast = parse_program(&mut token_stream).unwrap();
    assert_eq!(true, token_stream.is_exhausted());

    assert_eq!(exp_ast, ast);
}
