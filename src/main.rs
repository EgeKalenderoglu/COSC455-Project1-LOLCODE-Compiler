mod traits;
mod lexer;
mod compiler;

use compiler::LolCompiler;
use traits::Compiler;

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: lolcompiler <input.lol>");
        std::process::exit(1);
    }

    let input_filename = &args[1];

    if !input_filename.to_lowercase().ends_with(".lol") {
        eprintln!("Usage error: input file must have a .lol extension.");
        std::process::exit(1);
    }

    let source = match fs::read_to_string(input_filename) {
        Ok(contents) => contents,
        Err(error) => {
            eprintln!("Error reading '{}': {}", input_filename, error);
            std::process::exit(1);
        }
    };

    let mut compiler = LolCompiler::new();

    compiler.compile(&source);
    compiler.parse();

    let mut output_filename = PathBuf::from(input_filename);
    output_filename.set_extension("html");

    compiler.save_html(
        output_filename
            .to_str()
            .expect("Invalid output filename")
    );

    if cfg!(target_os = "macos") {
        let _ = Command::new("open")
            .arg(&output_filename)
            .status();
    }
    
    println!(
        "Compilation successful: {}",
        output_filename.display()
    );
}