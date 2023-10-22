use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use generator::Generator;

pub mod generator;
pub mod models;
fn main() {
    let mut generator = Generator::new();
    let matches = Command::new("password")
        .subcommand(
            Command::new("generate").args(&[
                Arg::new("length")
                    .long("length")
                    .value_parser(value_parser!(u8))
                    .action(clap::ArgAction::Set)
                    .required(true),
                Arg::new("numbers")
                    .long("numbers")
                    .action(clap::ArgAction::SetTrue),
                Arg::new("symbols")
                    .long("symbols")
                    .action(clap::ArgAction::SetTrue),
                Arg::new("capitalized")
                    .long("capitalized")
                    .action(clap::ArgAction::SetTrue),
                Arg::new("type")
                    .long("type")
                    .action(clap::ArgAction::Set)
                    .value_parser(["random", "pin"]),
            ]),
        )
        .get_matches();
    execute_command_from_matches(matches, &mut generator);
}

fn execute_command_from_matches(matches: ArgMatches, generator: &mut Generator) {
    match matches.subcommand() {
        Some(("generate", args)) => {
            if args.args_present() {
                execute_command_from_args(&args, generator)
            }
        }
        _ => unimplemented!(),
    }
}

fn execute_command_from_args(matches: &ArgMatches, generator: &mut Generator) {
    let len: u8 = *matches
        .get_one("length")
        .expect("length should be a positive number between 0 and 255");

    match matches
        .get_one::<String>("type")
        .and_then(|s| Some(s.as_str()))
    {
        Some("random") => {
            println!(
                "Generated random password: {}",
                generator.generate_password(len, true, true, true, true)
            );
            return;
        }
        Some("pin") => {
            println!(
                "Generated pin: {}",
                generator.generate_password(len, false, true, false, false)
            );
            return;
        }
        _ => {}
    }

    let is_num = matches.get_flag("numbers");
    let is_cap = matches.get_flag("capitalized");
    let is_sym = matches.get_flag("symbols");
    println!(
        "Generated Password: {}",
        generator.generate_password(len, is_sym, is_num, is_cap, true)
    );
}
