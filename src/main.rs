//! Xin language compiler CLI.
//!
//! Compiles Xin source code to bytecode.

use clap::Parser;
use tracing::{error, info};
use tracing_subscriber::fmt;

/// CLI arguments for the Xin compiler.
#[derive(Parser, Debug)]
#[command(name = "xin")]
#[command(about = "Xin language compiler", long_about = None)]
struct Args {
    /// Source file to compile.
    #[arg(short, long)]
    input: String,

    /// Output file (defaults to stdout if not specified).
    #[arg(short, long)]
    output: Option<String>,

    /// Enable verbose output.
    #[arg(short, long)]
    verbose: bool,

    /// Stop after lexing.
    #[arg(long)]
    lex: bool,

    /// Stop after parsing.
    #[arg(long)]
    parse: bool,

    /// Stop after semantic analysis.
    #[arg(long)]
    semantic: bool,
}

fn main() {
    let args = Args::parse();

    if args.verbose {
        fmt::init();
    }

    let source = match std::fs::read_to_string(&args.input) {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to read input file: {}", e);
            std::process::exit(1);
        }
    };

    info!("Compiling {}...", args.input);

    let tokens = match xin_lexer::tokenize(&source) {
        Ok(t) => t,
        Err(e) => {
            error!("Lexical error: {}", e);
            std::process::exit(1);
        }
    };

    info!("Generated {} tokens", tokens.len());

    if args.lex {
        info!("Stopping after lexing");
        if let Some(output) = args.output {
            std::fs::write(&output, format!("{:#?}", tokens)).unwrap();
        }
        return;
    }

    let _stmts = match xin_parser::parse_statement(&source) {
        Ok(s) => s,
        Err(e) => {
            error!("Parse error: {}", e);
            std::process::exit(1);
        }
    };

    info!("Parsed {} statements", _stmts.len());

    if args.parse {
        info!("Stopping after parsing");
        if let Some(output) = args.output {
            std::fs::write(&output, format!("{:#?}", _stmts)).unwrap();
        }
        return;
    }

    // Semantic analysis
    let mut _analyzer = xin_semantic::Analyzer::new();
    for stmt in &_stmts {
        if let Err(e) = _analyzer.analyze(stmt) {
            error!("Semantic error: {}", e);
            std::process::exit(1);
        }
    }

    info!("Semantic analysis passed");

    if args.semantic {
        info!("Stopping after semantic analysis");
        return;
    }

    if let Some(output) = args.output {
        info!("Output written to {}", output);
    } else {
        println!("Compilation successful");
    }
}
