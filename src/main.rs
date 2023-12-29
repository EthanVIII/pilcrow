extern crate core;
use std::env;
use clap::{arg, Parser};
use std::fs::File;
use std::io::Read;
use std::path::{Display, Path};
use log::{debug, error, info, warn};
use regex::{Regex, RegexSet};

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

#[derive(Debug, Clone, Eq, PartialEq)]
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
    Space,
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
            Token::Comment(_) => { (r"^\/\/.*\r?\n", r"//") }
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
            Token::ReturnToken => { (r"^return[^a-zA-Z0-9]?", r"return") }
            Token::LetToken => { (r"^let[^a-zA-Z0-9]?", r"let") }
            Token::ID(_) => { (r"^[a-zA-Z0-9_]+", "") }
            Token::Literal(_) => { ("^\"(?:[^\\\\\"]|\\\\.)*\"", "") }
            Token::EOL => { (r"^\r?\n", " ") }
            Token::Space => { ("^ ", " ") }
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

#[derive(Debug, Clone, Eq, PartialEq)]
struct AstNode {
    val: Token,
    children: Vec<AstNode>,
}
impl AstNode {
    fn new_empty(val: Token) -> AstNode {
        return AstNode {
            val,
            children: Vec::new(),
        }
    }
    fn new(val: Token, children: Vec<AstNode>) -> AstNode {
        return AstNode { val, children }
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

    // TODO: Implement better filtering for token spaces. 
    // Sorry, I'm just blanking rn and have no internet.
    let mut tokens_no_space: Vec<Token> = Vec::new();
    for token in tokens {
        match &token {
            Token::Space | Token::EOL | Token::Comment(_) => {}
            _ => {tokens_no_space.push(token)}
        }
    }

    println!("{:?}", tokens_no_space);

    // Parse elements
    let ast: AstNode = parse_to_ast(tokens_no_space);

}

fn parse_to_ast(tokens: Vec<Token>) -> AstNode {
    info!("Parsing tokens to AST.");
    let mut root: AstNode = AstNode::new_empty(Token::EOL);
    let mut token_cursor: usize = 0;
    while token_cursor < tokens.len() {
        let node: AstNode = match tokens[token_cursor] {
            // Structure of let expression declaration is as follows.
            // var_id = variable ID 
            // [] = Optional
            // literal_or_ref = Literal or variable ID
            // type = type of variable
            // let var_id [:type] = literal_or_ref
            //
            // LetToken
            //  |- ID
            //  |- Expression
            //  TODO: Implement the rest of let expression parsing.
            Token::LetToken => {
                // Lookahead to next token.
                let id_token = token_expected_lookahead(&tokens, &token_cursor, 1);
                AstNode::new(Token::LetToken,  vec![AstNode::new_empty(id_token.clone())])
            }
            // TODO: Implement all tokens.
            _ => {
                unimplemented!("Parsing {:?} is unimplemented.", tokens[token_cursor]);
            }
        };
        root.children.push(node);
        token_cursor += 1;
        println!("{:?}",root);
    }

    info!("Successfully parsed tokens to AST.");
    return root;
}

fn token_expected_lookahead<'a>(
    tokens: &'a Vec<Token>,
    position: &'a usize,
    lookahead: usize) -> &'a Token {
    return match token_lookahead(tokens, position, lookahead) {
        Option::Some(return_token) => return_token,
        Option::None => {
            error!(
                // TODO: Implement std::fmt::Display for 'Token'.
                "Invalid syntax, additional token not present at some point after {:?}",
                tokens[*position]);
            panic!("Panicked due to previous error");
        }
    }
}


fn token_lookahead<'a>(
    tokens: &'a Vec<Token>,
    position: &'a usize,
    lookahead: usize) -> Option<&'a Token> {
    if lookahead + position >= tokens.len() {
        return Option::None;
    }
    return Option::Some(&tokens[position + lookahead]);
}


fn tokenise(txt: String) -> Vec<Token> {
    info!("Tokenising from source code.");
    let mut eaten_txt: String = txt; // Text to be consumed.
    let mut tokens: Vec<Token> = vec![Token::EOL]; // Final return vec.

    let token_vals: Vec<Token> = all_tokens(); // Extract all tokens.
    let token_pats: Vec<String> = token_vals.iter()
        .map(|x| x.to_regex_and_literal().0)
        .collect(); // Get matching token string patterns.
    let set: RegexSet = match RegexSet::new(token_pats) {
        Ok(result) => {result}
        Err(error) => {
            error!("Regex parsing error. {}", error);
            panic!("Panicked due to previous error");
        }
    };
    // Continue loop while consuming text into tokens repeatedly.
    while eaten_txt.len() > 0 {
        // Eat multiple whitespace :)
        let first_char: char = eaten_txt.chars().nth(0).unwrap();
        if first_char == ' ' || first_char == '\t' {
            if tokens[tokens.len()-1] != Token::Space {
                tokens.push(Token::Space);
            }
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

        // Backtrack to find the matching pattern in the string. This is important
        // for variable length tokens to be constructed later.
        let matched_regex: Regex = Regex::new(&*longest_matched_token
            .to_regex_and_literal().0)
            .unwrap();
        let regex_match: &str = matched_regex
            .find(&*eaten_txt)
            .unwrap()
            .as_str();
        
        // Prepare the token to be pushed onto return list (tokens).
        // This includes finding the full comment, literal, or ID.
        let token_to_push: (Token, usize) = match longest_matched_token {
            Token::Comment(_) => { (Token::Comment(regex_match.to_string()), regex_match.len()) }
            Token::Literal(_) => { (Token::Literal(regex_match.to_string()), regex_match.len()) }
            Token::ID(_) => { (Token::ID(regex_match.to_string()), regex_match.len()) }
            x => (x.clone(), x.to_regex_and_literal().1.len())
        };

        // Consume the aforementioned token completely.
        eaten_txt = eaten_txt
            .chars()
            .skip(token_to_push.1)
            .collect();

        // Push the token onto return list.
        debug!("Pushing token: {:?}",token_to_push.0);
        debug!("Remaining Text: {:?}",eaten_txt);
        tokens.push(token_to_push.0);
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
