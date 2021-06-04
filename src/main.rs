use clap::{App, Arg};

mod crypto;
mod utils;
mod ux;

fn main() -> Result<(), std::io::Error> {
    let matches = App::new("Rust File Crypto")
                    .version("0.2.1")
                    .author("Jytesh")
                    .about("Used to encrypt, decrypt files.")
                    .arg(
                        Arg::with_name("input")
                        .short("i")
                        .long("input")
                        .help("Input file to encrypt or decrypt, will encrypt or decrypt all files in unen/encrypted if not provided")
                        .takes_value(true)
                        )
                    .arg(
                        Arg::with_name("output")
                        .short("o")
                        .long("output")
                        .help("Output file , will generate from input file if not provided")
                        .takes_value(true)
                    )
                    .arg(
                        Arg::with_name("password")
                        .short("p")
                        .long("password")
                        .help("Your default password for **ALL** files")
                        .takes_value(true)
                    )
                    .arg(
                        Arg::with_name("decrypt")
                        .short("d")
                        .help("Decrypt files, encrypt by default")
                    )
                    .get_matches();

    let encrypt = !matches.is_present("decrypt");
    let password = matches.value_of("password");
    let dir = match encrypt {
        true => "unen",
        false => "encrypted",
    };
    let files: Vec<String> = if let Some(input_file) = matches.value_of("input") {
        vec![String::from(input_file)]
    } else {
        let read_dir = utils::readdir(format!("./{}", dir))?;
        read_dir
    };
    println!("{:?}", files);
    let operation = if encrypt == true {
        utils::encrypt
    } else {
        utils::decrypt
    };

    let pass = match password {
            Some(x) => String::from(x),
            None => utils::getpass(),
        };
    
    for file in files {
        
        let output = if let Some(input_file) = matches.value_of("input") {
            if let Some(output_file) = matches.value_of("output") {
                String::from(output_file)
            } else {
                format!("{}.enc", input_file)
            }
        } else {
            let (x, y) = match encrypt {
                true => ("unen", "encrypted"),
                false => ("encrypted", "unen"),
            };
            String::from(str::replace(&file, x, y))
        };

        operation(file, output.to_string(), &pass)?;
    }
    Ok(())
}
