use sss::*;
use num_bigint::{ ToBigInt};

fn main() {
    let secret = 100.to_bigint().unwrap();
    let res = split(&secret, 10, 4);

    for (index, key_part) in res.clone().into_iter().enumerate() {
        println!("{}  {:x}", index + 1, key_part);
    }

    let res_secret = recover(vec![(0, res[0].clone()), (1, res[1].clone()), (2, res[2].clone()), (3, res[3].clone())]);
    println!("{:?} ", res_secret);
}
