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

#[derive(Debug)]
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
    Comment,
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
    fn to_regex_pattern(&self) -> String {
        return match &self {
            Token::LeftBrace => { r"^\{" }
            Token::RightBrace => { r"^\}" }
            Token::LeftBracket => { r"^\[" }
            Token::RightBracket => { r"^\]" }
            Token::LeftParen => { r"^\(" }
            Token::RightParen => { r"^\)" }
            Token::Semicolon => { r"^;" }
            Token::Colon => { r"^:" }
            Token::Period => { r"^\." }
            Token::RightArrow => { r"^->" }
            Token::Comment => { r"^//" }
            Token::LeqComparator => { r"^<=" }
            Token::GeqComparator => { r"^>=" }
            Token::LeComparator => { r"^<" }
            Token::GeComparator => { r"^>" }
            Token::EqComparator => { r"^==" }
            Token::NeqComparator => { r"^!=" }
            Token::AndOperation => { r"^&&" }
            Token::OrOperation => { r"^\|\|" }
            Token::Equal => { r"^=" }
            Token::Asterisk => { r"^\*" }
            Token::Ampersand => { r"^&" }
            Token::Dash => { r"^-" }
            Token::Slash => { r"^\/" }
            Token::QuestionMark => { r"^\?" }
            Token::Plus => { r"^\+" }
            Token::Pipe => { r"^\|" }
            Token::IfToken => { r"^if" }
            Token::ElseToken => { r"^else" }
            Token::WhileToken => { r"^while" }
            Token::InToken => { r"^in" }
            Token::FnToken => { r"^fn" }
            Token::ReturnToken => { r"return" }
            Token::LetToken => { r"let" }
            Token::ID(_) => { r"^[a-zA-Z_]+" }
            Token::Literal(_) => { "^[0-9]+|^\\\"" }
            Token::EOL => {r"^\r?\n"}
        }.to_string()
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
        Token::Comment,
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

    let token_vals: Vec<Token> = all_tokens();
    let token_pats: Vec<String> = token_vals.iter()
        .map(|x| x.to_regex_pattern())
        .collect();
    let set: RegexSet = match RegexSet::new(token_pats) {
        Ok(result) => {result}
        Err(error) => {
            error!("Regex parsing error. {}", error);
            panic!("Panicked due to previous error.");
        }
    };

    let matches: Vec<&Token> = set
        .matches(&txt)
        .into_iter()
        .map(|index| &token_vals[index])
        .collect();

    println!("{:?}",matches);

    //let token_progress: (Option<Vec<char>>, Option<Vec<char>>)= read_token_and_eat(&char_vec);

    info!("Successfully tokenised lines from source.");
    let regexes: Vec<_> = set
        .patterns()
        .iter()
        .map(|pat| Regex::new(pat).unwrap())
        .collect();
    return vec![];
}




// TODO: Refactor read_token_and_eat to output token and remainder of string.
fn read_token_and_eat(txt: &Vec<char>) -> (Option<Vec<char>>, Option<Vec<char>>) {
    if txt.get(0) == None {
        return (Option::None, Option::None);
    }





    ////
    // Special cases that should be looked ahead.
    // Full alphabet chars, not just ascii.
    // TODO: Rewrite with regex.
    if txt[0].is_alphabetic() || txt[0] == '_' {
        // TODO: Parse Identifier for a-z, A-Z, _
        let mut cursor: usize = 1;
        while txt.get(cursor) != Option::None {
            if txt[cursor].is_alphabetic() || txt[cursor] == '_' {
                cursor += 1;
            }
            else {
                return (Option::Some(txt[..cursor].to_vec()),
                    Option::Some(txt[cursor..].to_vec()));
            }
        }
        return (Option::Some(txt.to_vec()), Option::None); 
    }
    if txt[0].is_numeric() {
        // TODO: Parse full number for 0-9
        return (Option::None, Option::Some(txt[1..].to_vec())); 
    }
    // If string literal, parse until the end of the line.
    // TODO: Support for escaping strings using regex.
    if txt[0] == '"' {
        // TODO: Parse full string.

    }
    // Eat whitespace
    // TODO: Preserve whitespace eating behaviour.
    if txt[0].is_whitespace() {
        return (Option::None, Option::Some(txt[1..].to_vec()));
    }
    // TODO: Tokenising using regex.
    return (Option::Some(vec![txt[0]]), Option::Some(txt[1..].to_vec()));
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
