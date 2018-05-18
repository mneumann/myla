use ast;
use keyword::Keyword;
use stream::Stream;
use tokenizer::Token;

#[derive(PartialEq, Eq, Debug)]
pub struct ParseError {
    reason: String,
}

fn expect_next(token_stream: &mut Stream<Token>) -> Result<Token, ParseError> {
    token_stream.next().ok_or(ParseError {
        reason: "premature end".to_string(),
    })
}

fn expect_next_eql(token_stream: &mut Stream<Token>, exp: Token) -> Result<(), ParseError> {
    let tok = expect_next(token_stream)?;
    if tok != exp {
        Err(ParseError {
            reason: format!("expected token: {:?}. actual: {:?}", exp, tok),
        })
    } else {
        Ok(())
    }
}

pub fn parse_expr(token_stream: &mut Stream<Token>) -> Result<ast::Expr, ParseError> {
    let tok = expect_next(token_stream)?;
    match tok {
        Token::NumLiteral(s) => Ok(ast::Expr::NumLiteral(s)),
        _ => Err(ParseError {
            reason: "parse_expr: Expect NumLiteral".to_string(),
        }),
    }
}

pub fn parse_statement(token_stream: &mut Stream<Token>) -> Result<ast::Statement, ParseError> {
    let _ = expect_next_eql(token_stream, Token::Symbol("return".to_string()))?;
    let expr = parse_expr(token_stream)?;
    let _ = expect_next_eql(token_stream, Token::Semi)?;
    return Ok(ast::Statement::Return(Box::new(expr)));
}

pub fn parse_block(token_stream: &mut Stream<Token>) -> Result<ast::Block, ParseError> {
    let _ = expect_next_eql(token_stream, Token::OpenCur)?;
    let mut statements = Vec::new();
    loop {
        match token_stream.peek().ok_or(ParseError {
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

    let _ = expect_next_eql(token_stream, Token::CloseCur)?;

    Ok(ast::Block { statements })
}

pub fn parse_function(token_stream: &mut Stream<Token>) -> Result<ast::Function, ParseError> {
    let return_typename = expect_next(token_stream)?
        .get_symbol_string()
        .and_then(|name| {
            match Keyword::from_str(&name) {
                Some(_) => None, // reject keywords
                None => Some(name),
            }
        })
        .ok_or(ParseError {
            reason: "invalid return typename type".to_string(),
        })?;
    let function_name = expect_next(token_stream)?
        .get_symbol_string()
        .and_then(|name| {
            match Keyword::from_str(&name) {
                Some(_) => None, // reject keywords
                None => Some(name),
            }
        })
        .ok_or(ParseError {
            reason: "invalid function name type".to_string(),
        })?;

    let _ = expect_next_eql(token_stream, Token::OpenPar)?;
    let _ = expect_next_eql(token_stream, Token::ClosePar)?;
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

pub fn parse_program(token_stream: &mut Stream<Token>) -> Result<ast::Program, ParseError> {
    let mut functions = vec![];

    while !token_stream.is_exhausted() {
        functions.push(parse_function(token_stream)?);
    }

    return Ok(ast::Program { functions });
}

#[test]
fn test_parser() {
    let tokens = vec![
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

    let mut token_stream = Stream::new(tokens);
    let ast = parse_program(&mut token_stream).unwrap();
    assert_eq!(true, token_stream.is_exhausted());

    assert_eq!(exp_ast, ast);
}
