use ast;
use scanner::Token;

trait Emitter {
    type T;
    fn emit(&self, into: &mut Vec<Self::T>);
}

impl Emitter for ast::Program {
    type T = Token;
    fn emit(&self, into: &mut Vec<Self::T>) {
        for f in self.functions.iter() {
            f.emit(into);
        }
    }
}

impl Emitter for ast::Function {
    type T = Token;
    fn emit(&self, into: &mut Vec<Self::T>) {
        self.return_type.emit(into);
        into.push(Token::Symbol(self.name.clone()));
        into.push(Token::OpenPar);
        // TODO: parameters
        into.push(Token::ClosePar);
        self.block.emit(into);
    }
}

impl Emitter for ast::Parameter {
    type T = Token;
    fn emit(&self, into: &mut Vec<Self::T>) {
        unimplemented!();
    }
}

impl Emitter for ast::Type {
    type T = Token;
    fn emit(&self, into: &mut Vec<Self::T>) {
        into.push(Token::Symbol(self.name.clone()));
    }
}

impl Emitter for ast::Block {
    type T = Token;
    fn emit(&self, into: &mut Vec<Self::T>) {
        into.push(Token::OpenCur);
        for stmt in self.statements.iter() {
            stmt.emit(into);
        }
        into.push(Token::CloseCur);
    }
}

impl Emitter for ast::Statement {
    type T = Token;
    fn emit(&self, into: &mut Vec<Self::T>) {
        match self {
            &ast::Statement::Return(ref expr) => {
                into.push(Token::Symbol("return".to_string()));
                expr.emit(into);
                into.push(Token::Semi);
            }
        }
    }
}

impl Emitter for ast::Expr {
    type T = Token;
    fn emit(&self, into: &mut Vec<Self::T>) {
        match self {
            &ast::Expr::NumLiteral(ref s) => {
                into.push(Token::NumLiteral(s.clone()));
            }
        }
    }
}

fn tokens_to_string(tokens: &[Token]) -> String {
    let mut s = String::new();
    let mut last_token_num_or_sym = false;
    for t in tokens {
        match t {
            Token::OpenPar => {
                s.push_str("(");
                last_token_num_or_sym = false;
            }
            Token::ClosePar => {
                s.push_str(")");
                last_token_num_or_sym = false;
            }
            Token::OpenCur => {
                s.push_str("{");
                last_token_num_or_sym = false;
            }
            Token::CloseCur => {
                s.push_str("}");
                last_token_num_or_sym = false;
            }
            Token::Semi => {
                s.push_str(";");
                last_token_num_or_sym = false;
            }
            Token::Symbol(ref name) => {
                if last_token_num_or_sym {
                    s.push_str(" ");
                }
                s.push_str(name);
                last_token_num_or_sym = true;
            }
            Token::NumLiteral(ref num) => {
                if last_token_num_or_sym {
                    s.push_str(" ");
                }
                s.push_str(num);
                last_token_num_or_sym = true;
            }
        }
    }
    return s;
}

#[test]
fn test_codegen_c() {
    let ast = ast::Program {
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

    let mut tokens = vec![];
    ast.emit(&mut tokens);

    assert_eq!(exp_tokens, tokens);

    let exp_code = "int main(){return 0;}";
    let code = tokens_to_string(&tokens[..]);

    assert_eq!(exp_code, &code);
}
