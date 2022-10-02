use clap::{Parser, Subcommand};

mod crypto;
mod utils;
mod ux;

#[derive(Parser)]
#[command(version, author, about)]
#[command(next_line_help = true)]
struct Cli {
    /// Input file
    // #[arg(long, short)]
    // input: String,

    /// Output file, defaults to appending/replacing .enc/.unenc to input file
    #[arg(long, short)]
    output: Option<String>,

    /// Optional password to use
    #[arg(long, short)]
    password: Option<String>,

    /// Display logging information
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Encrypt {
        /// The file to encrypt
        input: String,
    },
    Decrypt {
        /// The file to decrypt
        input: String,
    },
}

fn main() -> Result<(), std::io::Error> {
    let cli = Cli::parse();
    let (encrypt, input_file) = match cli.commands {
        Commands::Encrypt { input } => (true, input),
        Commands::Decrypt { input } => (false, input),
    };

    let operation = if encrypt == true {
        utils::encrypt
    } else {
        utils::decrypt
    };

    let password = match cli.password {
        Some(x) => String::from(x),
        None => utils::getpass(),
    };

    let output = if let Some(output_file) = cli.output {
        output_file
    } else {
        let (x, y) = match encrypt {
            true => ("unenc", "enc"),
            false => ("enc", "unenc"),
        };
        let input_path = input_file.clone();
        let mut input_path = input_path.split('/').collect::<Vec<&str>>();
        let input_file_name = input_path.remove(input_path.len() - 1);

        let output_file_name = if input_file_name.contains(x) {
            input_file_name.replace(x, y).to_string()
        } else {
            format!("{}.{}", input_file_name, y)
        };
        
        input_path.push(&output_file_name);
        input_path.join("/")
    };

    println!(
        "Input: {}\nOutput: {}\nOperation: {}",
        input_file,
        output,
        match encrypt {
            true => "Encrypt",
            false => "Decrypt",
        }
    );
    operation(input_file, output.to_string(), &password)?;
    Ok(())
}
