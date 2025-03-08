use std::env;
use std::fs;

/// Token enum represents different types of tokens that can be identified in the Atomic language.
#[derive(Debug)]
enum Token {
    Print,           // Represents the 'print' keyword
    Add,             // Represents the 'add' keyword
    Multiply,        // Represents the 'multiply' keyword
    Number(i32),     // Represents a number (integer)
    String(String),  // Represents a string (text enclosed in quotes)
    Unknown(String), // Represents an unknown token (not recognized by the lexer)
}

/// ASTNode enum represents different types of parsed expressions in the Abstract Syntax Tree.
#[derive(Debug)]
enum ASTNode {
    Print(String),      // A print statement containing a string
    Add(i32, i32),      // An addition operation with two numbers
    Multiply(i32, i32), // A multiplication operation with two numbers
}

/// Lexer: Converts raw source code into a list of tokens.
fn lexer(code: &str) -> Vec<Token> {
    let mut tokens = Vec::new(); // Vector to store identified tokens
    let mut words = code.split_whitespace().peekable(); // Splits the input text into words

    while let Some(word) = words.next() {
        match word {
            "print" => tokens.push(Token::Print),
            "add" => tokens.push(Token::Add),
            "multiply" => tokens.push(Token::Multiply),
            _ => {
                // Check if the token is a number
                if let Ok(num) = word.parse::<i32>() {
                    tokens.push(Token::Number(num));
                }
                // Check if the token is a string (starts and ends with quotes)
                else if word.starts_with("\"") {
                    let mut full_string = word.to_string();
                    while let Some(next_word) = words.peek() {
                        full_string.push(' ');
                        full_string.push_str(next_word);
                        if next_word.ends_with("\"") {
                            words.next(); // Consume the last word with closing quote
                            break;
                        } else {
                            words.next(); // Keep consuming until we find closing quote
                        }
                    }
                    tokens.push(Token::String(full_string.trim_matches('"').to_string()));
                }
                // If the token is unknown, store it as Token::Unknown
                else {
                    tokens.push(Token::Unknown(word.to_string()));
                }
            }
        }
    }

    tokens
}

/// Parser: Converts the list of tokens into an Abstract Syntax Tree (AST).
fn parser(tokens: Vec<Token>) -> Vec<ASTNode> {
    let mut ast = Vec::new(); // Vector to store AST nodes
    let mut iter = tokens.iter().peekable(); // Allows peeking at upcoming tokens

    while let Some(token) = iter.next() {
        match token {
            Token::Print => {
                if let Some(Token::String(text)) = iter.next() {
                    ast.push(ASTNode::Print(text.clone())); // Store print statement in AST
                } else {
                    eprintln!("Syntax Error: Expected a string after 'print'");
                }
            }
            Token::Add => {
                if let (Some(Token::Number(a)), Some(Token::Number(b))) = (iter.next(), iter.next())
                {
                    ast.push(ASTNode::Add(*a, *b)); // Store addition operation in AST
                } else {
                    eprintln!("Syntax Error: Expected two numbers after 'add'");
                }
            }
            Token::Multiply => {
                if let (Some(Token::Number(a)), Some(Token::Number(b))) = (iter.next(), iter.next())
                {
                    ast.push(ASTNode::Multiply(*a, *b)); // Store multiplication operation in AST
                } else {
                    eprintln!("Syntax Error: Expected two numbers after 'multiply'");
                }
            }
            _ => {}
        }
    }

    ast
}

/// Executor: Processes and executes the AST nodes.
fn execute(ast: &[ASTNode]) {
    for node in ast {
        match node {
            ASTNode::Print(text) => {
                println!("{}", text); // ✅ Prints text to the console
            }
            ASTNode::Add(a, b) => {
                let result = a + b;
                println!("{} + {} = {}", a, b, result); // ✅ Prints addition result
            }
            ASTNode::Multiply(a, b) => {
                let result = a * b;
                println!("{} * {} = {}", a, b, result); // ✅ Prints multiplication result
            }
        }
    }
}

/// Main Function: Reads the file, runs the lexer, parser, and executor.
fn main() {
    // Read command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run <filename.atomic>"); // Print usage error if no file provided
        return;
    }

    // Read the Atomic script file
    let filename = &args[1];
    let code = fs::read_to_string(filename).expect("Failed to read file");

    // Run Lexer: Convert source code to tokens
    let tokens = lexer(&code);
    println!("Tokens:");
    for token in &tokens {
        match token {
            Token::Print => println!("  PRINT"),
            Token::Add => println!("  ADD"),
            Token::Multiply => println!("  MULTIPLY"),
            Token::Number(n) => println!("  NUMBER({})", n),
            Token::String(s) => println!("  STRING(\"{}\")", s),
            Token::Unknown(u) => println!("  UNKNOWN TOKEN(\"{}\")", u), // ✅ Prints unknown tokens
        }
    }

    // Run Parser: Convert tokens to an AST
    let ast = parser(tokens);
    println!("AST: {:?}", ast); // Print the parsed AST

    // Execute AST: Run the Atomic program
    execute(&ast);
}
