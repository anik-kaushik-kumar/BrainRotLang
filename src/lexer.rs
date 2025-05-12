use crate::parser::{Statement, Expr};
use std::collections::HashMap;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

pub struct Interpreter {
    env: HashMap<String, i32>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            env: HashMap::new(),
        }
    }

    pub fn run(&mut self, stmts: &[Statement]) {
        for stmt in stmts {
            self.exec(stmt);
        }
    }

    fn exec(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Declare(name, Expr::Number(n)) => {
                self.env.insert(name.clone(), *n);
            }

            Statement::Declare(name, Expr::Ident(var)) => {
                if let Some(val) = self.env.get(var) {
                    self.env.insert(name.clone(), *val);
                } else {
                    eprintln!("Undefined variable '{}'", var);
                }
            }

            Statement::Add(a, b, c) => {
                let val = self.env.get(a).unwrap_or(&0) + self.env.get(b).unwrap_or(&0);
                self.env.insert(c.clone(), val);
            }

            Statement::Sub(a, b, c) => {
                let val = self.env.get(a).unwrap_or(&0) - self.env.get(b).unwrap_or(&0);
                self.env.insert(c.clone(), val);
            }

            Statement::Mul(a, b, c) => {
                let val = self.env.get(a).unwrap_or(&0) * self.env.get(b).unwrap_or(&0);
                self.env.insert(c.clone(), val);
            }

            Statement::Div(a, b, c) => {
                let denominator = self.env.get(b).unwrap_or(&1);
                if *denominator != 0 {
                    let val = self.env.get(a).unwrap_or(&0) / denominator;
                    self.env.insert(c.clone(), val);
                } else {
                    eprintln!("Division by zero in {} / {}", a, b);
                }
            }

            Statement::Print(var) => {
                if let Some(val) = self.env.get(var) {
                    println!("{}", val);
                } else {
                    eprintln!("Undefined variable '{}'", var);
                }
            }

            Statement::Input(var) => {
                print!("> ");
                io::stdout().flush().unwrap();
                let mut buffer = String::new();
                if io::stdin().read_line(&mut buffer).is_ok() {
                    if let Ok(n) = buffer.trim().parse::<i32>() {
                        self.env.insert(var.clone(), n);
                    } else {
                        eprintln!("Invalid input");
                    }
                }
            }

            Statement::If {
                cond,
                then_branch,
                else_branch,
            } => {
                let condition = self.env.get(cond).unwrap_or(&0);
                if *condition != 0 {
                    for stmt in then_branch {
                        self.exec(stmt);
                    }
                } else {
                    for stmt in else_branch {
                        self.exec(stmt);
                    }
                }
            }

            Statement::Loop { count_var, body } => {
                let count = *self.env.get(count_var).unwrap_or(&0);
                for _ in 0..count {
                    for stmt in body {
                        self.exec(stmt);
                    }
                }
            }

            Statement::Sleep(ms) => {
                thread::sleep(Duration::from_millis(*ms));
            }

            Statement::Clear(var) => {
                self.env.remove(var);
            }
        }
    }
}
