use std::collections::HashMap;
use std::env;
use std::fs;

/// Token enum represents different types of tokens in Atomic.
#[derive(Debug)]
enum Token {
    Print,
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    Let,
    Identifier(String),
    Number(i32),
    String(String),
}

/// Value enum represents a number or a variable name.
#[derive(Debug)]
enum Value {
    Number(i32),
    Variable(String),
}

/// ASTNode represents parsed expressions in the Abstract Syntax Tree.
#[derive(Debug)]
enum ASTNode {
    Print(String),
    Add(Value, Value),
    Subtract(Value, Value),
    Multiply(Value, Value),
    Divide(Value, Value),
    Modulus(Value, Value),
    Let(String, i32),
}

/// Lexer: Converts source code into a list of tokens.
fn lexer(code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut words = code.split_whitespace().peekable();

    while let Some(word) = words.next() {
        match word {
            "print" => tokens.push(Token::Print),
            "add" => tokens.push(Token::Add),
            "subtract" => tokens.push(Token::Subtract),
            "multiply" => tokens.push(Token::Multiply),
            "divide" => tokens.push(Token::Divide),
            "mod" => tokens.push(Token::Modulus),
            "let" => {
                tokens.push(Token::Let);
                if let Some(var) = words.next() {
                    if words.peek() == Some(&"=") {
                        words.next(); // Consume '='
                        if let Some(value) = words.next() {
                            if let Ok(num) = value.parse::<i32>() {
                                tokens.push(Token::Identifier(var.to_string()));
                                tokens.push(Token::Number(num));
                                continue;
                            }
                        }
                    }
                }
                eprintln!("Syntax Error: Invalid variable assignment syntax");
            }
            _ => {
                if let Ok(num) = word.parse::<i32>() {
                    tokens.push(Token::Number(num));
                } else if word.starts_with("\"") {
                    let mut full_string = word.to_string();
                    while let Some(next_word) = words.peek() {
                        full_string.push(' ');
                        full_string.push_str(next_word);
                        if next_word.ends_with("\"") {
                            words.next();
                            break;
                        } else {
                            words.next();
                        }
                    }
                    tokens.push(Token::String(full_string.trim_matches('"').to_string()));
                } else if word.chars().all(|c| c.is_alphabetic()) {
                    tokens.push(Token::Identifier(word.to_string()));
                } else {
                    eprintln!("Unknown token: {}", word);
                }
            }
        }
    }

    tokens
}

/// Parser: Converts tokens into an AST.
fn parser(tokens: Vec<Token>) -> Vec<ASTNode> {
    let mut ast = Vec::new();
    let mut iter = tokens.iter().peekable();

    while let Some(token) = iter.next() {
        match token {
            Token::Print => {
                if let Some(Token::String(text)) = iter.next() {
                    ast.push(ASTNode::Print(text.clone()));
                } else {
                    eprintln!("Syntax Error: Expected a string after 'print'");
                }
            }
            Token::Let => {
                if let (Some(Token::Identifier(var)), Some(Token::Number(value))) =
                    (iter.next(), iter.next())
                {
                    ast.push(ASTNode::Let(var.clone(), *value));
                } else {
                    eprintln!("Syntax Error: Expected variable name and value after 'let'");
                }
            }
            Token::Add | Token::Subtract | Token::Multiply | Token::Divide | Token::Modulus => {
                let first = iter.next();
                let second = iter.next();

                let left_value = match first {
                    Some(Token::Number(n)) => Value::Number(*n),
                    Some(Token::Identifier(var)) => Value::Variable(var.clone()),
                    _ => {
                        eprintln!("Syntax Error: Expected a number or variable after operation");
                        continue;
                    }
                };

                let right_value = match second {
                    Some(Token::Number(n)) => Value::Number(*n),
                    Some(Token::Identifier(var)) => Value::Variable(var.clone()),
                    _ => {
                        eprintln!("Syntax Error: Expected a number or variable after operation");
                        continue;
                    }
                };

                let operation = match token {
                    Token::Add => ASTNode::Add(left_value, right_value),
                    Token::Subtract => ASTNode::Subtract(left_value, right_value),
                    Token::Multiply => ASTNode::Multiply(left_value, right_value),
                    Token::Divide => ASTNode::Divide(left_value, right_value),
                    Token::Modulus => ASTNode::Modulus(left_value, right_value),
                    _ => unreachable!(),
                };

                ast.push(operation);
            }
            _ => {}
        }
    }

    ast
}

/// Executor: Processes and executes the AST nodes.
fn execute(ast: &[ASTNode]) {
    let mut variables: HashMap<String, i32> = HashMap::new();

    for node in ast {
        match node {
            ASTNode::Print(text) => {
                println!("{}", text);
            }
            ASTNode::Let(var, value) => {
                variables.insert(var.clone(), *value);
                println!("Variable {} set to {}", var, value);
            }
            ASTNode::Add(a, b) => {
                let left = resolve_value(a, &variables);
                let right = resolve_value(b, &variables);
                println!("{} + {} = {}", left, right, left + right);
            }
            ASTNode::Subtract(a, b) => {
                let left = resolve_value(a, &variables);
                let right = resolve_value(b, &variables);
                println!("{} - {} = {}", left, right, left - right);
            }
            ASTNode::Multiply(a, b) => {
                let left = resolve_value(a, &variables);
                let right = resolve_value(b, &variables);
                println!("{} * {} = {}", left, right, left * right);
            }
            ASTNode::Divide(a, b) => {
                let left = resolve_value(a, &variables);
                let right = resolve_value(b, &variables);
                if right != 0 {
                    println!("{} / {} = {}", left, right, left / right);
                } else {
                    eprintln!("Error: Division by zero");
                }
            }
            ASTNode::Modulus(a, b) => {
                let left = resolve_value(a, &variables);
                let right = resolve_value(b, &variables);
                if right != 0 {
                    println!("{} % {} = {}", left, right, left % right);
                } else {
                    eprintln!("Error: Modulus by zero");
                }
            }
        }
    }
}

/// Helper Function: Resolves a value (number or variable).
fn resolve_value(value: &Value, variables: &HashMap<String, i32>) -> i32 {
    match value {
        Value::Number(n) => *n,
        Value::Variable(name) => *variables.get(name).unwrap_or(&0), // Default to 0 if variable not found
    }
}

/// Main Function: Reads the file, runs the lexer, parser, and executor.
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run <filename.atomic>");
        return;
    }

    let filename = &args[1];
    let code = fs::read_to_string(filename).expect("Failed to read file");

    let tokens = lexer(&code);
    println!("Tokens: {:?}", tokens);

    let ast = parser(tokens);
    println!("AST: {:?}", ast);

    execute(&ast);
}
