use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use error::PasswordGeneratorError;
use generator::Generator;

pub mod error;
pub mod generator;
pub mod models;

fn main() -> Result<(), PasswordGeneratorError> {
    let mut generator = Generator::new();
    let matches = Command::new("password")
        .subcommand(
            Command::new("password-generate").args(&[
                Arg::new("length")
                    .long("length")
                    .value_parser(value_parser!(u8))
                    .action(ArgAction::Set)
                    .default_value("10"),
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

    println!("Welcome to the password generator 5000");
    execute_command_from_matches(matches, &mut generator)
}

fn execute_command_from_matches(
    matches: ArgMatches,
    generator: &mut Generator,
) -> Result<(), PasswordGeneratorError> {
    match matches.subcommand() {
        Some(("password-generate", args)) => {
            if args.args_present() {
                execute_command_from_args(&args, generator)?;
                return Ok(());
            }
            Err(PasswordGeneratorError::ArgumentsNotFound)
        }
        _ => {
            eprintln!("Error: Please choose a valid subcommand: (password-generate)");
            Err(PasswordGeneratorError::CommandNotFound)
        }
    }
}

fn execute_command_from_args(
    matches: &ArgMatches,
    generator: &mut Generator,
) -> Result<(), PasswordGeneratorError> {
    let len: u8 = match matches.get_one("length") {
        Some(len) => *len,
        None => {
            eprintln!("length should be a positive number between 0 and 255");
            return Err(PasswordGeneratorError::InvalidArgument);
        }
    };

    match matches
        .get_one::<String>("type")
        .and_then(|s| Some(s.as_str()))
    {
        Some("random") => {
            let generated_password = generator.generate_password(len, true, true, true, true);
            println!("Generated fully random password: {}", generated_password);
            eprintln!(
                "Fully random password's strength: {:.0}%",
                generator.get_password_strength(len, true, true, true, true)
            );
            return Ok(());
        }
        Some("pin") => {
            let generated_password = generator.generate_password(len, false, true, false, false);
            println!("Generated pin: {}", generated_password);
            eprintln!(
                "Generated pin's strength: {:.0}%",
                generator.get_password_strength(len, false, true, false, false)
            );
            return Ok(());
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
        "Password's strength: {:.0}%",
        generator.get_password_strength(len, is_sym, is_num, is_cap, true)
    );
    Ok(())
}
