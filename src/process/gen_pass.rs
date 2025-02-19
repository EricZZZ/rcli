use rand::seq::{IndexedRandom, SliceRandom};
use zxcvbn::zxcvbn;

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

pub fn process_genpass(
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<()> {
    let mut rng = rand::rng();
    let mut password = Vec::new();
    let mut chars = Vec::with_capacity(128);
    if upper {
        chars.extend_from_slice(UPPER);
        password.push(*chars.choose(&mut rng).expect("UPPER is would not be empty"));
    }
    if lower {
        chars.extend_from_slice(LOWER);
        password.push(*chars.choose(&mut rng).expect("LOWER is would not be empty"))
    }
    if number {
        chars.extend_from_slice(NUMBER);
        password.push(
            *chars
                .choose(&mut rng)
                .expect("NUMBER is would not be empty"),
        )
    }
    if symbol {
        chars.extend_from_slice(SYMBOL);
        password.push(
            *chars
                .choose(&mut rng)
                .expect("SYMBOL is would not be empty"),
        )
    }
    for _ in 0..(length - password.len() as u8) {
        let c = chars.choose(&mut rng).expect("chars is would not be empty");
        password.push(*c);
    }
    password.shuffle(&mut rng);

    let password = String::from_utf8(password)?;
    println!("{}", password);

    let estimate = zxcvbn(&password, &[]);
    println!("Password strength: {}", estimate.score());

    Ok(())
}
