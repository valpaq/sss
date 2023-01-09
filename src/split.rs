use num_bigint::{BigInt, RandBigInt, ToBigInt};
use rand::rngs::OsRng;

pub fn split(secret: &BigInt, n_parts: u64, k_parts: u64) -> Vec<BigInt> {
    let mut rng = OsRng;
    let coefs = (0..k_parts - 1)
        .map(|_| rng.gen_bigint(256))
        .collect::<Vec<BigInt>>();

    (1..=n_parts)
        .map(|i| i.to_bigint().unwrap())
        .map(|i_big| {
            secret
                + coefs
                    .iter()
                    .enumerate()
                    .map(|(j, coef)| coef * i_big.pow((j + 1).try_into().unwrap()))
                    .sum::<BigInt>()
        })
        .collect::<Vec<BigInt>>()
}
