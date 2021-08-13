use clap::{App, Arg, ArgMatches};

const PAYLOAD: &str = "payload";

fn main() {
    let arguments = parse_arguments();
    let payload = arguments.value_of(PAYLOAD).unwrap();
    let (hash, nonce) = pow::prove(payload)
        .expect("Error occurred while solving the puzzle")
        .expect("Solution could not be found");
    println!("{}\n{}", hash, nonce);
}

fn parse_arguments() -> ArgMatches<'static> {
    App::new(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::with_name(PAYLOAD)
                .required(true)
                .short("p")
                .long("payload")
                .takes_value(true)
                .help("Payload (hexadecimal string)"),
        )
        .get_matches()
}
