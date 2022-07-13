pub mod parser;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(
    name = "base64-emoji-parser-rs",
    author = "shouta",
    version = "v1.0.0",
    about = "Rust version of base64-emoji-parser"
)]
pub struct AppArg {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Subcommand)]
pub enum Action {
    ToEmoji { text: String },
    ToBase64 { text: String },
}
