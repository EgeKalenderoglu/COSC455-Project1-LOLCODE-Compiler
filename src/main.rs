mod traits;
mod lexer;
mod compiler;

use compiler::LolCompiler;
use traits::Compiler;

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
// Handles command-line input, runs the compiler, saves the HTML output,
// and opens the generated file in the default macOS browser.
fn main() {
    // Read the command-line arguments and require exactly one input file.
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: lolcompiler <input.lol>");
        std::process::exit(1);
    }

    let input_filename = &args[1];
    // Only source files with the .lol extension are accepted.
    if !input_filename.to_lowercase().ends_with(".lol") {
        eprintln!("Usage error: input file must have a .lol extension.");
        std::process::exit(1);
    }
    // Read the entire LOLCODE source file into a string for compilation.
    let source = match fs::read_to_string(input_filename) {
        Ok(contents) => contents,
        Err(error) => {
            eprintln!("Error reading '{}': {}", input_filename, error);
            std::process::exit(1);
        }
    };
    // Create the compiler, load the source, and begin syntax analysis.
    let mut compiler = LolCompiler::new();

    compiler.compile(&source);
    compiler.parse();
    // Keep the original filename and directory but replace .lol with .html.
    let mut output_filename = PathBuf::from(input_filename);
    output_filename.set_extension("html");
    // Save the generated HTML only after compilation finishes successfully.
    compiler.save_html(
        output_filename
            .to_str()
            .expect("Invalid output filename")
    );
    // On macOS, automatically open the generated HTML in the default browser.
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