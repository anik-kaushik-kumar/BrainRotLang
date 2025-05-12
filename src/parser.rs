// parser.rs
use crate::lexer::Token;

#[derive(Debug)]
pub enum Expr {
    Number(i32),
    Ident(String),
}

#[derive(Debug)]
pub enum Statement {
    Declare(String, Expr),
    Add(String, String, String),
    Print(String),
    Input(String),
    If {
        cond: String,
        then_branch: Vec<Statement>,
        else_branch: Vec<Statement>,
    },
    Loop {
        count_var: String,
        body: Vec<Statement>,
    },
    Sleep(u64),
    Clear(String),
}

pub fn parse(tokens: &[Token]) -> Vec<Statement> {
    let mut stmts = Vec::new();
    let mut i = 0;

    fn parse_block(tokens: &[Token], i: &mut usize) -> Vec<Statement> {
        let mut block = Vec::new();
        while *i < tokens.len() && tokens[*i] != Token::EndStmt {
            let inner = parse(&tokens[*i..]);
            if let Some(s) = inner.into_iter().next() {
                block.push(s);
            }
            while *i < tokens.len() && tokens[*i] != Token::EndStmt {
                *i += 1;
            }
            *i += 1;
        }
        block
    }

    while i < tokens.len() {
        match &tokens[i] {
            Token::Declare => {
                if let (Token::Ident(name), Token::Assign, Token::Number(val), Token::EndStmt) =
                    (&tokens[i + 1], &tokens[i + 2], &tokens[i + 3], &tokens[i + 4])
                {
                    stmts.push(Statement::Declare(name.clone(), Expr::Number(*val)));
                    i += 5;
                } else {
                    i += 1;
                }
            }
            Token::Ident(a) => {
                if let (Token::Add, Token::Ident(b), Token::Into, Token::Ident(c), Token::EndStmt) =
                    (&tokens[i + 1], &tokens[i + 2], &tokens[i + 3], &tokens[i + 4], &tokens[i + 5])
                {
                    stmts.push(Statement::Add(a.clone(), b.clone(), c.clone()));
                    i += 6;
                } else {
                    i += 1;
                }
            }
            Token::Print => {
                if let (Token::Ident(name), Token::EndStmt) = (&tokens[i + 1], &tokens[i + 2]) {
                    stmts.push(Statement::Print(name.clone()));
                    i += 3;
                } else {
                    i += 1;
                }
            }
            Token::Input => {
                if let (Token::Ident(name), Token::EndStmt) = (&tokens[i + 1], &tokens[i + 2]) {
                    stmts.push(Statement::Input(name.clone()));
                    i += 3;
                } else {
                    i += 1;
                }
            }
            Token::If => {
                if let Token::Ident(cond) = &tokens[i + 1] {
                    i += 2;
                    let then_branch = parse_block(tokens, &mut i);
                    let else_branch = if let Token::Else = tokens[i] {
                        i += 1;
                        parse_block(tokens, &mut i)
                    } else {
                        Vec::new()
                    };
                    stmts.push(Statement::If {
                        cond: cond.clone(),
                        then_branch,
                        else_branch,
                    });
                    i += 1;
                } else {
                    i += 1;
                }
            }
            Token::Loop => {
                if let Token::Ident(var) = &tokens[i + 1] {
                    i += 2;
                    let body = parse_block(tokens, &mut i);
                    stmts.push(Statement::Loop {
                        count_var: var.clone(),
                        body,
                    });
                    i += 1;
                } else {
                    i += 1;
                }
            }
            Token::Sleep => {
                if let (Token::Number(ms), Token::EndStmt) = (&tokens[i + 1], &tokens[i + 2]) {
                    stmts.push(Statement::Sleep(*ms as u64));
                    i += 3;
                } else {
                    i += 1;
                }
            }
            Token::Clear => {
                if let (Token::Ident(name), Token::EndStmt) = (&tokens[i + 1], &tokens[i + 2]) {
                    stmts.push(Statement::Clear(name.clone()));
                    i += 3;
                } else {
                    i += 1;
                }
            }
            _ => i += 1,
        }
    }
    stmts
}