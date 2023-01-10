use num_bigint::ToBigInt;
use sss::*;

fn main() {
    let secret = 123435651345673413445647234u128.to_bigint().unwrap();
    let res = split(&secret, 10, 3);

    for (index, key_part) in res.clone().into_iter().enumerate() {
        println!("{}  {:?}", index + 1, key_part);
    }

    let res_secret = recover(vec![(1, res[0].clone()), (2, res[1].clone()), (3, res[2].clone())]);
    println!("{res_secret:?} ");
}
