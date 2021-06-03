use clap::{App, Arg, SubCommand};

mod keygen;
mod utils;

fn main() -> Result < (), std::io::Error> {
    let matches = App::new("Rust File Crypto")
                    .version("1.0")
                    .author("Jytesh")
                    .about("Used to encrypt, decrypt files. Also provides RSA Key generation")
                    .arg(
                        Arg::with_name("input")
                        .short("i")
                        .help("Input file to encrypt or decrypt, will encrypt or decrypt all files in unen/encrypted if not provided")
                        .takes_value(true)
                        )
                    .arg(
                        Arg::with_name("output")
                        .short("o")
                        .help("Output file , will generate from input file if not provided")
                        .takes_value(true)
                    )
                    .arg(
                        Arg::with_name("decrypt")
                        .short("d")
                        .help("Decrypt files, encrypt by default")
                    )
                    .subcommand(
                        SubCommand::with_name("generate-key")
                        .about("Generate RSA Private and Public Keys")
                        .version("1.0")
                        .author("Jytesh")
                    )
                    .get_matches();

    // Key generation
    if let Some(_) = matches.subcommand_matches("generate-key") {
        keygen::keygen();
    }
    let encrypt = !matches.is_present("decrypt");
    let dir = match encrypt {
                true => "unen",
                false => "encrypted",
    };
    let files: Vec<String> = if let Some(input_file) =  matches.value_of("input") {
            vec![String::from(input_file)]
    } else {
        let read_dir = utils::readdir(format!("./{}", dir))?;
        read_dir
    };
    
    println!("{:?}", files);
    Ok(())
}

