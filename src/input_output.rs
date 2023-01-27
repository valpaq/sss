use itertools::free::join;
use itertools::Itertools;
use num_bigint::BigInt;
use num_traits::Num;
use std::fmt::Error;
use std::fs;
use std::io::Write;
use std::ops::AddAssign;
use std::path::PathBuf;

pub fn split_write(out: &mut dyn Write, information: Vec<BigInt>) -> Result<(), Error> {
    for (index, key_part) in information.into_iter().enumerate() {
        out.write(format!("{:}  {key_part:x} \n", index + 1).as_bytes())
            .expect("unable to write");
    }
    Ok(())
}

pub fn parse_split(file: PathBuf) -> (BigInt, u64, u64) {
    let file_contents = fs::read_to_string(file).expect("Unable to read file");
    let mut lines_iter = file_contents.lines();
    let string_to_hide = BigInt::from_str_radix(
        &join(
            lines_iter
                .next()
                .unwrap()
                .chars()
                .map(|x| format!("{:x}", x as u32)),
            &"",
        ),
        16,
    )
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

pub fn parse_recover(file: PathBuf) -> Vec<(i128, BigInt)> {
    let file_contents = fs::read_to_string(file).expect("Unable to read file");
    let mut res: Vec<(i128, BigInt)> = Vec::new();
    for line in file_contents.lines() {
        let args: Vec<&str> = line.split_whitespace().into_iter().collect();
        let x: i128 = args[0].parse().expect("expected number");
        let y = BigInt::parse_bytes(args[1].as_bytes(), 16).expect("Failed to convert");
        res.push((x, y));
    }
    res
}

pub fn recover_parse_result(number: &BigInt) -> String {
    let number_radix = number.to_str_radix(16);
    let mut result_string = String::new();
    for i in (0..number_radix.len()).step_by(2) {
        let letter = number_radix.get(i..=i + 1).unwrap();
        result_string.add_assign(
            &char::from_u32(u32::from_str_radix(letter, 16).unwrap())
                .unwrap()
                .to_string(),
        );
    }
    result_string
}

pub fn recover_write(out: &mut dyn Write, str: &str) -> Result<(), Error> {
    out.write(format!("{}\n", str).as_bytes())
        .expect("unable to write");
    Ok(())
}
