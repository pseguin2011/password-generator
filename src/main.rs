use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use generator::Generator;

pub mod generator;
pub mod models;
fn main() {
    let mut generator = Generator::new();
    let matches = Command::new("password")
        .subcommand(
            Command::new("password-generate").args(&[
                Arg::new("length")
                    .long("length")
                    .value_parser(value_parser!(u8))
                    .action(ArgAction::Set)
                    .required(true),
                Arg::new("numbers")
                    .long("numbers")
                    .action(ArgAction::SetTrue),
                Arg::new("symbols")
                    .long("symbols")
                    .action(ArgAction::SetTrue),
                Arg::new("capitalized")
                    .long("capitalized")
                    .action(ArgAction::SetTrue),
                Arg::new("type")
                    .long("type")
                    .action(ArgAction::Set)
                    .value_parser(["random", "pin", "memorable"]),
            ]),
        )
        .get_matches();
    execute_command_from_matches(matches, &mut generator);
}

fn execute_command_from_matches(matches: ArgMatches, generator: &mut Generator) {
    match matches.subcommand() {
        Some(("password-generate", args)) => {
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
            let generated_password = generator.generate_password(len, true, true, true, true);
            println!("Generated fully random password: {}", generated_password);
            eprintln!(
                "Fully random password's strength: {}",
                generator.get_password_strength(len, true, true, true, true)
            );
            return;
        }
        Some("pin") => {
            let generated_password = generator.generate_password(len, false, true, false, false);
            println!("Generated pin: {}", generated_password);
            eprintln!(
                "Generated pin's strength: {}",
                generator.get_password_strength(len, false, true, false, false)
            );
            return;
        }
        Some("memorable") => unimplemented!(),
        _ => {}
    }

    let is_num = matches.get_flag("numbers");
    let is_cap = matches.get_flag("capitalized");
    let is_sym = matches.get_flag("symbols");
    let generated_password = generator.generate_password(len, is_sym, is_num, is_cap, true);
    println!("Generated password: {}", generated_password);
    eprintln!(
        "Fully random password's strength: {}",
        generator.get_password_strength(len, is_sym, is_num, is_cap, true)
    );
}
