use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::{
    identities::{One, Zero},
    CheckedDiv, CheckedMul,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Fraction {
    pub numerator: BigUint,
    pub denominator: BigUint,
}

impl Default for Fraction {
    fn default() -> Self {
        Fraction {
            numerator: BigUint::zero(),
            denominator: BigUint::one(),
        }
    }
}

impl Fraction {
    fn new(numerator: BigUint, denominator: BigUint) -> Self {
        Fraction {
            numerator,
            denominator,
        }
    }

    fn fraction_add(self, other: Self) -> Self {
        let self_denominator = self.denominator.clone();
        let other_denominator = other.denominator.clone();
        let common_denominator = self_denominator.gcd(&other_denominator);
        let other_denominator = other_denominator
            .checked_div(&common_denominator)
            .expect("zero common denominator");
        let self_denominator = self_denominator
            .checked_div(&common_denominator)
            .expect("zero common div");
        let new_numerator = self.numerator.checked_mul(&other_denominator).unwrap()
            + other.numerator.checked_mul(&self_denominator).unwrap();
        let new_denominator = self_denominator.checked_mul(&other_denominator).unwrap();
        Fraction {
            numerator: new_numerator,
            denominator: new_denominator,
        }
    }

    fn finalize(self) -> BigUint {
        self.numerator / self.denominator
    }
}
