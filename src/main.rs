use base64_emoji_parser_rs::{Action, AppArg};
use clap::Parser;
mod parser;
use parser::conversion::Conversion;

fn main() {
    let cli = AppArg::parse();
    match cli.action {
        Action::ToEmoji { text } => {
            let result = Conversion::to_emoji(text);
            println!("{}", result);
        }
        Action::ToBase64 { text } => {
            let result = Conversion::to_base64(text);
            println!("{}", result);
        }
    }
}
