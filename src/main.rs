use std::env;
use clap::{arg, Parser};
use std::fs::File;
use std::io::Read;
use std::path::{Display, Path};
use log::{ debug, error, info, warn};

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

    println!("{}", file_txt);
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