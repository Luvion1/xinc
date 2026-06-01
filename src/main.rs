//! Xin language compiler CLI.
//!
//! Compiles Xin source code via the driver pipeline.

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

    if args.lex {
        match xin_driver::tokenize(&source) {
            Ok(tokens) => {
                info!("Generated {} tokens", tokens.len());
                if let Some(output) = args.output
                    && let Err(e) = std::fs::write(&output, format!("{:#?}", tokens))
                {
                    error!("Failed to write output file: {}", e);
                    std::process::exit(1);
                }
            }
            Err(e) => {
                error!("Lexical error: {}", e);
                std::process::exit(1);
            }
        }
        return;
    }

    let stmts = match xin_driver::parse(&source) {
        Ok(s) => {
            info!("Parsed {} statements", s.len());
            s
        }
        Err(e) => {
            error!("Parse error: {}", e);
            std::process::exit(1);
        }
    };

if args.parse
    && let Some(ref output) = args.output
    && let Err(e) = std::fs::write(output, format!("{:#?}", stmts))
{
        error!("Failed to write output file: {}", e);
        std::process::exit(1);
    }

    if args.parse {
        return;
    }

    // Full compilation via driver
    match xin_driver::compile(&source) {
        Ok(result) => {
            if args.semantic {
                info!("Semantic analysis passed");
                return;
            }
            if let Some(output) = args.output {
                if let Err(e) = std::fs::write(&output, result) {
                    error!("Failed to write output file: {}", e);
                    std::process::exit(1);
                }
                info!("Output written to {}", output);
            } else {
                println!("{}", result);
            }
        }
        Err(e) => {
            error!("Compilation failed: {}", e);
            std::process::exit(1);
        }
    }
}
