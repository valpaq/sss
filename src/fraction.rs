use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::{
    identities::{One, Zero},
    CheckedDiv, CheckedMul,
};

#[derive(Clone, Debug, PartialEq, Eq)]
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
    pub fn new(numerator: &BigUint, denominator: &BigUint) -> Self {
        Fraction {
            numerator: numerator.clone(),
            denominator: denominator.clone(),
        }
    }

    pub fn fraction_add(&self, other: &Self) -> Self {
        // x = gcd(b, d)
        // a/b + c/d = [a * (d/x) + c * (b/x)] / (b/x*d)
        let self_denominator = self.denominator.clone();
        let other_denominator = other.denominator.clone();
        let common_denominator = self_denominator.gcd(&other_denominator);
        let other_denominator = other_denominator
            .checked_div(&common_denominator)
            .expect("zero common denominator");
        let self_denominator = self_denominator
            .checked_div(&common_denominator)
            .expect("zero common denominator");
        let new_numerator = self.numerator.checked_mul(&other_denominator).unwrap()
            + other.numerator.checked_mul(&self_denominator).unwrap();
        let new_denominator = self_denominator.checked_mul(&other_denominator).unwrap();
        if new_numerator.is_multiple_of(&common_denominator) {
            Fraction {
                numerator: new_numerator.checked_div(&common_denominator).unwrap(),
                denominator: new_denominator,
            }
        } else {
            Fraction {
                numerator: new_numerator,
                denominator: new_denominator.checked_mul(&common_denominator).unwrap(),
            }
        }
    }

    pub fn finalize(&self) -> BigUint {
        self.numerator.clone() / self.denominator.clone()
    }
}

#[cfg(test)]
mod tests {
    use num_bigint::ToBigUint;

    use super::*;

    #[test]
    fn new_fraction_and_default() {
        let fraction_def = Fraction::default();
        let fraction_init = Fraction::new(&0.to_biguint().unwrap(), &1.to_biguint().unwrap());
        assert_eq!(fraction_def, fraction_init);
    }

    #[test]
    fn sum_fraction_default() {
        let fraction_def1 = Fraction::default();
        let fraction_def2 = Fraction::default();
        let fraction_res = fraction_def1.fraction_add(&fraction_def2);
        assert_eq!(fraction_def1, fraction_res);
    }

    #[test]
    fn sum_fraction_to_1_1() {
        let fraction_def1 = Fraction::new(&1.to_biguint().unwrap(), &2.to_biguint().unwrap());
        let fraction_def2 = Fraction::new(&1.to_biguint().unwrap(), &2.to_biguint().unwrap());
        let fraction_res = fraction_def1.fraction_add(&fraction_def2);
        assert_eq!(
            fraction_res,
            Fraction::new(&1.to_biguint().unwrap(), &1.to_biguint().unwrap())
        );
    }

    #[test]
    fn sum_fraction1() {
        let fraction_def1 = Fraction::new(&1.to_biguint().unwrap(), &3.to_biguint().unwrap());
        let fraction_def2 = Fraction::new(&3.to_biguint().unwrap(), &8.to_biguint().unwrap());
        let fraction_res = fraction_def1.fraction_add(&fraction_def2);
        assert_eq!(
            fraction_res,
            Fraction::new(&17.to_biguint().unwrap(), &24.to_biguint().unwrap())
        );
    }

    #[test]
    fn sum_fraction2() {
        let fraction_def1 = Fraction::new(&3.to_biguint().unwrap(), &9.to_biguint().unwrap());
        let fraction_def2 = Fraction::new(&4.to_biguint().unwrap(), &18.to_biguint().unwrap());
        let fraction_res = fraction_def1.fraction_add(&fraction_def2);
        assert_eq!(
            fraction_res,
            Fraction::new(&10.to_biguint().unwrap(), &18.to_biguint().unwrap())
        );
    }
}
