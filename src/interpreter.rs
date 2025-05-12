// interpreter.rs
use crate::parser::{Statement, Expr};
use std::collections::HashMap;
use std::{io, thread, time};

pub struct Interpreter {
    pub env: HashMap<String, i32>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter { env: HashMap::new() }
    }

    pub fn run(&mut self, stmts: &[Statement]) {
        for stmt in stmts {
            match stmt {
                Statement::Declare(name, Expr::Number(n)) => {
                    self.env.insert(name.clone(), *n);
                }
                Statement::Add(a, b, c) => {
                    let val = self.env.get(a).unwrap_or(&0) + self.env.get(b).unwrap_or(&0);
                    self.env.insert(c.clone(), val);
                }
                Statement::Print(name) => {
                    if let Some(val) = self.env.get(name) {
                        println!("{}", val);
                    }
                }
                Statement::Input(name) => {
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    if let Ok(num) = input.trim().parse::<i32>() {
                        self.env.insert(name.clone(), num);
                    }
                }
                Statement::If { cond, then_branch, else_branch } => {
                    if *self.env.get(cond).unwrap_or(&0) != 0 {
                        self.run(then_branch);
                    } else {
                        self.run(else_branch);
                    }
                }
                Statement::Loop { count_var, body } => {
                    let count = *self.env.get(count_var).unwrap_or(&0);
                    for _ in 0..count {
                        self.run(body);
                    }
                }
                Statement::Sleep(ms) => {
                    thread::sleep(time::Duration::from_millis(*ms));
                }
                Statement::Clear(name) => {
                    self.env.remove(name);
                }
            }
        }
    }
}
