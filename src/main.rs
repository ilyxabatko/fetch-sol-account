use ::clap::{Args, Parser, Subcommand};

mod api;
// mod error;
mod utils;

#[derive(Parser)]
#[command(author, version)]
#[command(
    about = "stringer - a simple CLI to transform and inspect strings",
    long_about = "stringer is a super fancy CLI (kidding)
One can use stringer to modify or inspect strings straight from the terminal"
)]

struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Reverses a string
    Reverse(Reverse),
    /// Inspects a string
    Inspect(Inspect),
}

#[derive(Args)]
struct Reverse {
    /// The string to reverse
    string: Option<String>,
}

#[derive(Args)]
struct Inspect {
    /// The string to inspect
    string: Option<String>,
    /// Use this flag to inspect digits inside a string
    #[arg(short = 'd', long = "digits")]
    only_digits: bool,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Reverse(str)) => match str.string {
            Some(ref _str) => {
                let reverse = api::stringer::reverse_string(_str);
                println!("{}", reverse);
            }
            None => {
                println!("Please, provide a string to reverse.");
            }
        },
        Some(Commands::Inspect(str)) => match str.string {
            Some(ref _str) => {
                let (result, kind) = api::stringer::inspect_string(_str, str.only_digits);

                let mut plural_s = "s";
                if result == 1 {
                    plural_s = "";
                }

                println!("{:?} has {} {}{}.", _str, result, kind, plural_s);
            }, 
            None => {
                println!("Please, provide a string to inspect.");
            }
        },
        None => {}
    }
}
