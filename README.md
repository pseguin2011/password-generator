# Password Generation CLI

## Build instructions
- use command `cargo build --release`

## Run instructions
- use command `cargo run --release password-generate`
- use command `./target/release/password-generator`

### Arguments
The following arguments can be passed to the running command
- `--length <Length>` the length of the generated password (required)
- `--type <[random, pin, memorable]>` override flags below for a defined set of rules
    - `random` includes numbers, symbols, capitalized and lowerase letters
    - `pin` includes only numbers
    - `memorable` includes numbers, symbols, capitalized and lowerase words
- `--numbers` if the password should include numbers
- `--symbols` if the password should include symbols
- `--capitalized` if the password should include capitalized letters

## Note
By default, lowercase letters will be included in the password rules and the password length is 10

To provide a secure generation of passwords, a CSprng was used to randomly select characters in the password, and the entropy of the host device is used to seed the rng.

View docs here:
https://rust-random.github.io/rand/rand/rngs/struct.StdRng.html
https://docs.rs/rand_core/latest/rand_core/trait.SeedableRng.html#method.from_entropy

## Help
run command `cargo run --release password-generate --help`

## Test
run command `cargo test`