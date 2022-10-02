# file-crypto-cli
A simple CLI utility to encrypt/decrypt your files with Orion-rs


Uses [Orion](https://github.com/orion-rs) to decrypt and encrypt with Argon2i stretched passwords.

# CLI

**__Usage:__** file-crypto [OPTIONS] <COMMAND>

**__Commands:__**
  **encrypt**
          
  **decrypt**
          
  **help**
          Print this message or the help of the given subcommand(s)

**__Options:__**
  **-o**, **--output** <OUTPUT>
          Input file Output file, defaults to appending/replacing .enc/.unenc to input file
  **-p**, **--password** <PASSWORD>
          Optional password to use
  **-v**, **--verbose**...
          Display logging information
  **-h**, **--help**
          Print help information
  **-V**, **--version**
          Print version information


```
NOTE: Verbose option is not supported at the moment
```

# Examples
- `file-crypto -p "any password" encrypt ./Cargo.toml`
- `file-crypto -p "any password"decrypt ./Cargo.toml.enc`


