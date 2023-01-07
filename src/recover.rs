use crate::Fraction;
use num_bigint::{BigInt, ToBigInt};

pub fn recover(points: Vec<(i128, BigInt)>) -> BigInt {
    let (points_x, _): (Vec<_>, Vec<_>) = points.iter().cloned().unzip();
    points
        .iter()
        .map(|(x_point, y_point)| {
            Fraction::new(
                &y_point
                    .checked_mul(
                        &points_x
                            .iter()
                            .filter(|x| *x != x_point)
                            .product::<i128>()
                            .to_bigint()
                            .unwrap(),
                    )
                    .unwrap(),
                &points_x
                    .iter()
                    .filter(|x| *x != x_point)
                    .map(|x| x_point - x)
                    .product::<i128>()
                    .to_bigint()
                    .unwrap(),
            )
        })
        .reduce(|acc, e| acc.fraction_add(&e))
        .unwrap()
        .finalize()
}
