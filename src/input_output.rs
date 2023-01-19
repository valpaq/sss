use itertools::free::join;
use itertools::Itertools;
use num_bigint::BigInt;
use std::fmt::Error;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

pub fn split_write(out: &mut dyn Write, information: Vec<BigInt>) -> Result<(), Error> {
    for (index, key_part) in information.into_iter().enumerate() {
        out.write(format!("{}  {key_part:} \n", index + 1).as_bytes())
            .expect("unable to write");
    }
    Ok(())
}

pub fn parse_split(file: PathBuf) -> (BigInt, u64, u64) {
    let file_contents = fs::read_to_string(file).expect("Unable to read file");
    let mut lines_iter = file_contents.lines();
    let string_to_hide = BigInt::from_str(&join(
        lines_iter
            .next()
            .unwrap()
            .as_bytes()
            .into_iter()
            .map(|s| format!("{:03}", s)),
        &"",
    ))
    .unwrap();
    let (n_parts, k_parts): (u64, u64) = lines_iter
        .next()
        .unwrap()
        .split(' ')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .next_tuple()
        .unwrap();

    (string_to_hide, n_parts, k_parts)
}

// pub fn parse_recover() -> (usize, Vec<(i128, BigInt)>) {
//     let mut t = String::new();
//     io::stdin().read_line(&mut t).expect("Failed to read T");
//     let t: usize = t.trim().parse().expect("Non number value T");

//     let mut key_parts: Vec<BigInt> = Vec::new();
//     let mut x: Vec<i128> = Vec::new();

//     for _ in 0..t {
//         let mut s = String::new();
//         io::stdin().read_line(&mut s).expect("Failed to read key");

//         let args: Vec<&str> = s.split_whitespace().into_iter().collect();

//         x.push(args[0].parse().expect("Number expected"));

//         let key_part = args[1].trim().trim_start_matches("0x");
//         let key_part = BigInt::parse_bytes(key_part.as_bytes(), 16).expect("Failed to convert");

//         key_parts.push(key_part);
//     }

//     let v: Vec<(i128, BigInt)> = x.into_iter().zip(key_parts.into_iter()).collect();

//     (t, v)
// }
