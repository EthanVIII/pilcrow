use std::env;
use clap::{arg, Parser};
use std::fs::File;
use std::io::Read;
use std::path::{Display, Path};
use std::str::{Lines, SplitWhitespace};
use log::{debug, error, info, warn};

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

    EOL,
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
}

fn tokenise(txt: String) -> Vec<Token> {
    let lines_txt: Lines = txt.lines();
    let mut token_builder: Vec<Token> = Vec::new();
    let mut cursor: usize = 0;
    let mut lookahead: usize = 0;
    for line in lines_txt.map( |x| x.chars().collect::<Vec<char>>()) {
        while cursor < line.len() {
            if line[cursor] == ' ' {
                println!("Space");
            } else {
                // match_string(cursor)
            }
            cursor += 1; 
        }
        
        token_builder.push(Token::EOL);
    }
    return token_builder;
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
