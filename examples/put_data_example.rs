extern crate env_logger;

use std::{fs::File, io::Read, str::FromStr};

use at_records::{at_key::AtKey, at_record::AtValue};
use at_rust::at_client::AtClient;
use at_secrets::AtSecrets;
use at_sign::AtSign;
use clap::Parser;

// e.g. RUST_BACKTRACE=1 RUST_LOG=debug cargo run --example put_data_example -- --file ~/.atsign/keys/@42territorial_key.atKeys --at-sign 42territorial --at-key @virgogigantic64:demo.test@42territorial --data "Hello from rust"

#[derive(Parser, Debug)]
#[command()]
struct Cli {
    /// The file path to process
    #[arg(short, long)]
    file: String,

    /// The string argument to use
    #[arg(short, long)]
    at_sign: String,

    /// String representation of at_key to get the data of
    #[arg(long)]
    at_key: String,

    /// The data to put
    #[arg(long)]
    data: String,
}

fn main() {
    env_logger::init();

    let cli = Cli::parse();

    // Read the contents of the file into a string.
    let mut file = File::open(cli.file).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // Create the secrets object from the file
    let secrets = AtSecrets::from_file(&contents).expect("Failed to create secrets");

    // Create the atSign of the client
    let at_sign = AtSign::new(cli.at_sign);

    let mut at_client = AtClient::init(secrets, at_sign).expect("Failed to create AtClient");
    let at_key = AtKey::from_str(&cli.at_key).expect("Invalid at_key");
    let data = cli.data;
    let result = at_client
        .put_record(&at_key, &AtValue::Text(data))
        .expect("Failed to put record");
    println!("{:?}", result);
}
