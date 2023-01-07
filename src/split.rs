use num_bigint::BigUint;
use num_bigint::RandBigInt;
use num_bigint::ToBigUint;
use rand::rngs::OsRng;

pub fn split(secret: BigUint, n_parts: u64, k_parts: u64) -> Vec<BigUint> {
    let mut rng = OsRng;
    let coefs = (0..k_parts - 1)
        .map(|_| rng.gen_biguint(256))
        .collect::<Vec<BigUint>>();

    (1..=n_parts)
        .map(|i| i.to_biguint().unwrap())
        .map(|i_big| {
            &secret
                + coefs
                    .iter()
                    .enumerate()
                    .map(|(j, coef)| coef * i_big.pow((j + 1).try_into().unwrap()))
                    .sum::<BigUint>()
        })
        .collect::<Vec<BigUint>>()
}
