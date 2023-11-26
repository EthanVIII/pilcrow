use std::env;
use clap::{arg, Parser};
use std::fs::File;
use std::io::Read;
use std::path::{Display, Path};
use std::str::{Lines, SplitWhitespace};
use log::{debug, error, info, warn};
use regex::Regex;

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
enum ComparisonType {
    LEQ,
    GEQ,
    LE,
    GE,
    EQ,
    NEQ,
    AND,
    OR,
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
    Comparator(ComparisonType),
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

    // EOL,
}
impl Token {
    // TODO: Implement Regex for these tokens.
    fn token_regex(token: Token) -> Regex {
        return match token {
        // Language punctuation and operators
            Token::LeftBrace => {},
            Token::RightBrace => {},
            Token::LeftBracket => {},
            Token::RightBracket => {},
            Token::LeftParen => {},
            Token::RightParen => {},
            Token::Semicolon=> {},
            Token::Colon=> {},
            Token::Period=> {},
            Token::RightArrow=> {},
            Token::Comment=> {},
            Token::Comparator(ComparisonType)=> {},
            Token::Equal=> {},
            Token::Asterisk=> {},
            Token::Ampersand=> {},
            Token::Dash=> {},
            Token::Slash=> {},
            Token::QuestionMark=> {},
            Token::Plus=> {},
            Token::Pipe=> {},

            // Language keywords
            Token::IfToken=> {},
            Token::ElseToken=> {},
            Token::WhileToken=> {},
            Token::InToken=> {},
            Token::FnToken=> {},
            Token::ReturnToken=> {},
            Token::LetToken=> {},

            Token::ID(String)=> {},
            Token::Literal(String)=> {},
        }
    }
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
    let lines_txt: Lines = txt.lines();
    let mut token_builder: Vec<Token> = Vec::new();
    
    info!("Tokenising lines from source code.");
    // TODO: Do it all in one shot?
    for line in lines_txt{
        let char_vec: Vec<char> = line.chars().collect();
        let token_progress: (Option<Vec<char>>, Option<Vec<char>>)= read_token_and_eat(&char_vec);
        match token_progress.0 {
            Some(x) =>  
        match expr {
                Some(expr) => expr,
                None => expr,
            }
        }
        token_builder.append(token_progress.0)
    }
    info!("Successfully tokenised lines from source.");
    return token_builder;
}

// TODO: Refactor read_token_and_eat to output token and remainder of string.
fn read_token_and_eat(txt: &Vec<char>) -> (Option<Vec<char>>, Option<Vec<char>>) {
    if txt.get(0) == None {
        return (Option::None, Option::None);
    }
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
