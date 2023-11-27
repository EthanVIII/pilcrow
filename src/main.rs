extern crate core;
use std::env;
use clap::{arg, Parser};
use std::fs::File;
use std::io::Read;
use std::path::{Display, Path};
use std::str::{Lines, SplitWhitespace};
use clap::builder::Str;
use log::{debug, error, info, warn};
use regex::{Error, Regex, RegexSet};

// Argument struct
#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    /// Pilcrow file to run
    filename: String,

    /// Enable verbose mode
    #[arg(long,short)]
    verbose: bool,

    /// Build and run program in compile mode
    #[arg(long, short)]
    compile: bool,
}

#[derive(Debug, Clone)]
enum Token {
    // Language punctuation and operators
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    LeftParen,
    RightParen,
    Semicolon,
    Colon,
    Period,
    RightArrow,
    Comment(String),
    LeqComparator,
    GeqComparator,
    LeComparator,
    GeComparator,
    EqComparator,
    NeqComparator,
    AndOperation,
    OrOperation,
    Equal,
    Asterisk,
    Ampersand,
    Dash,
    Slash,
    QuestionMark,
    Plus,
    Pipe,

    // Language keywords
    IfToken,
    ElseToken,
    WhileToken,
    InToken,
    FnToken,
    ReturnToken,
    LetToken,

    ID(String),
    Literal(String),

    EOL,
}
impl Token {
    fn to_regex_and_literal(&self) -> (String,String) {
        let regex_literal: (&str, &str) = match &self {
            Token::LeftBrace => { (r"^\{", r"{") }
            Token::RightBrace => { (r"^\}", r"}") }
            Token::LeftBracket => { (r"^\[", r"[") }
            Token::RightBracket => { (r"^\]", r"]") }
            Token::LeftParen => { (r"^\(", r"(") }
            Token::RightParen => { (r"^\)", r")") }
            Token::Semicolon => { (r"^;", r";") }
            Token::Colon => { (r"^:", r":") }
            Token::Period => { (r"^\.", r".") }
            Token::RightArrow => { (r"^->", r"->") }
            Token::Comment(_) => { (r"^//", r"//") }
            Token::LeqComparator => { (r"^<=", r"<=") }
            Token::GeqComparator => { (r"^>=", r">=") }
            Token::LeComparator => { (r"^<", r"<") }
            Token::GeComparator => { (r"^>", r">") }
            Token::EqComparator => { (r"^==", r"==") }
            Token::NeqComparator => { (r"^!=", r"!=") }
            Token::AndOperation => { (r"^&&", r"&&") }
            Token::OrOperation => { (r"^\|\|", r"||") }
            Token::Equal => { (r"^=", r"=") }
            Token::Asterisk => { (r"^\*", r"*") }
            Token::Ampersand => { (r"^&", r"&") }
            Token::Dash => { (r"^-", r"-") }
            Token::Slash => { (r"^\/", r"/") }
            Token::QuestionMark => { (r"^\?", r"?") }
            Token::Plus => { (r"^\+", r"+") }
            Token::Pipe => { (r"^\|", r"|") }
            // Keywords can optionally be followed by a non alphanumeric character.
            Token::IfToken => { (r"^if[^a-zA-Z0-9]?", r"if") }
            Token::ElseToken => { (r"^else[^a-zA-Z0-9]?", r"else") }
            Token::WhileToken => { (r"^while[^a-zA-Z0-9]?", r"while") }
            Token::InToken => { (r"^in[^a-zA-Z0-9]?", r"in") }
            Token::FnToken => { (r"^fn[^a-zA-Z0-9]?", r"fn") }
            Token::ReturnToken => { (r"return[^a-zA-Z0-9]?", r"return") }
            Token::LetToken => { (r"let[^a-zA-Z0-9]?", r"let") }
            Token::ID(_) => { (r"^[a-zA-Z0-9_]+", "") }
            Token::Literal(_) => { ("^[0-9]+|^\\\"", "") }
            Token::EOL => { (r"^\r?\n", " ") }
        };
        return (regex_literal.0.to_string(), regex_literal.1.to_string());
    }
}

fn all_tokens() -> Vec<Token> {
    vec![
        Token::LeftBrace,
        Token::RightBrace,
        Token::LeftBracket,
        Token::RightBracket,
        Token::LeftParen,
        Token::RightParen,
        Token::Semicolon,
        Token::Colon,
        Token::Period,
        Token::RightArrow,
        Token::Comment("".to_string()),
        Token::LeqComparator,
        Token::GeqComparator,
        Token::LeComparator,
        Token::GeComparator,
        Token::EqComparator,
        Token::NeqComparator,
        Token::AndOperation,
        Token::OrOperation,
        Token::Equal,
        Token::Asterisk,
        Token::Ampersand,
        Token::Dash,
        Token::Slash,
        Token::QuestionMark,
        Token::Plus,
        Token::Pipe,
        Token::IfToken,
        Token::ElseToken,
        Token::WhileToken,
        Token::InToken,
        Token::FnToken,
        Token::ReturnToken,
        Token::LetToken,
        Token::ID("".to_string()),
        Token::Literal("".to_string()),
        Token::EOL
    ]
}

fn main() {
    let args = Args::parse();

    // TODO: Implement routing to compile mode.
    if args.compile == true {
        unimplemented!("[ERROR] Compile mode not implemented");
    }

    // Enable verbose mode by setting logging level to debug
    if args.verbose == true {
        env::set_var("RUST_LOG", "debug");
    } else {
        env::set_var("RUST_LOG", "error");
    }
    env_logger::init();

    // Read file.
    let file_txt: String = read_file(&args.filename);

    // Tokenise elements
    let tokens: Vec<Token> = tokenise(file_txt);
    println!("{:?}", tokens);
    // TODO: Parse elements in AST.
}

fn tokenise(txt: String) -> Vec<Token> {
    info!("Tokenising from source code.");
    let mut eaten_txt: String = txt; // Text to be consumed.
    let mut tokens: Vec<Token> = Vec::new(); // Final return vec.

    let token_vals: Vec<Token> = all_tokens(); // Extract all tokens.
    let token_pats: Vec<String> = token_vals.iter()
        .map(|x| x.to_regex_and_literal().0)
        .collect(); // Get matching token string patterns.
    let set: RegexSet = match RegexSet::new(token_pats) {
        Ok(result) => {result}
        Err(error) => {
            error!("Regex parsing error. {}", error);
            panic!("Panicked due to previous error.");
        }
    };
    // Continue loop while consuming text into tokens repeatedly.
    while eaten_txt.len() > 0 {
        // Eat whitespace :) It is a token separator only.
        if eaten_txt.chars().nth(0).unwrap() == ' ' {
            eaten_txt.remove(0);
            continue;
        }
        // Match with Regex and pull out the longest matched token.
        // Keywords are always prioritised over string literals and ids.
        let longest_matched_token: &Token = match set
            .matches(&eaten_txt)
            .into_iter()
            .map(|index| &token_vals[index])
            .max_by_key(|x| x.to_regex_and_literal().1.len()) {
            None => {
                error!(
                    "Token not found for character(s) beginning with {}",
                    eaten_txt.chars().nth(0).unwrap()
                );
                panic!("Panicking due to previous error.");
            }
            Some(x) => {x}
        };
        // Prepare the token to be pushed onto return list (tokens).
        // This includes finding the full comment, literal, or ID.
        let token_to_push: (Token, usize) = match longest_matched_token {
            // TODO: Read rest of line as comment.
            Token::Comment(_) => {
                (Token::Comment("This is a comment".to_string()), 1)
            }
            // TODO: Read until other " as Literal. For now.
            // TODO: Implement Escape characters.
            Token::Literal(_) => {
                (Token::Literal("literal".to_string()), 1)
            }
            // TODO: Read until non alphanumeric character.
            Token::ID(_) => {
                (Token::ID("id".to_string()), 1)
            }
            x => (x.clone(), x.to_regex_and_literal().1.len())
        };
        // Consume the aforementioned token completely.
        eaten_txt = eaten_txt
            .chars()
            .skip(token_to_push.1)
            .collect();
        // Push the token onto return list.
        tokens.push(token_to_push.0);
        println!("{:?}",eaten_txt);
        println!("{:?}",longest_matched_token);
    }

    info!("Successfully tokenised lines from source.");

    return tokens;
}

// Read file and return file text if available.
fn read_file(filename: &String) -> String {
    info!("Attempting to open {}", filename);
    let path: &Path = Path::new(filename);
    let display: Display = path.display();
    let mut file: File = match File::open(&path) {
        Err(why) => {
            error!("Could not open {}. {}", display, why);
            panic!("Panicked due to previous error");
        },
        Ok(file) => file,
    };
    let mut file_txt: String = String::new();
    match file.read_to_string(&mut file_txt) {
        Err(why) => {
            error!("Could not read {}. {}", display, why);
            panic!("Panicked due to previous error");
        },
        _ => {}
    };
    info!("{} read successfully", filename);
    return file_txt;
}
