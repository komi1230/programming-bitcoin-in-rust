use primitive_types::U256;

use std::cmp::{Eq, PartialEq};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone)]
pub struct FieldElement {
    pub num: U256,
    pub prime: U256,
}

impl FieldElement {
    pub fn new(num: U256, prime: U256) -> Self {
        if num >= prime {
            panic!("Num {:?} not in field range 0 to {:?}", num, prime)
        }
        Self { num, prime }
    }
}

impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        self.prime == other.prime && self.num == other.num
    }
}

impl Eq for FieldElement {}

impl Add for FieldElement {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        if self.prime != other.prime {
            panic!("Prime number should be same")
        }
        if self.num + other.num >= self.prime {
            Self {
                num: self.num + other.num - self.prime,
                prime: self.prime,
            }
        } else {
            Self {
                num: self.num + other.num,
                prime: self.prime,
            }
        }
    }
}

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        if self.prime != other.prime {
            panic!("Cannot subtract two numbers in different Fields.");
        }
        if self.num < other.num {
            Self::new(self.prime - self.num + other.num, self.prime)
        } else {
            Self::new(self.num - other.num, self.prime)
        }
    }
}

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot multiply two numbers in different Fields.");
        }
        let mut ret = FieldElement::new(U256::from(0), self.prime);
        let mut counter = other.num;
        loop {
            if counter < U256::from(u128::MAX) {
                for _ in 0..counter.as_u128() {
                    ret = ret + self.clone();
                }
                break;
            }

            if counter >= U256::from(u128::MAX) {
                for _ in 0..u128::MAX {
                    ret = ret + self.clone();
                }
            }
            counter -= U256::from(u128::MAX);
        }
        ret
    }
}

impl Div for FieldElement {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let p = self.prime;
        self * other.pow(p - 2)
    }
}

trait Pow<T>
where
    T: Add<Output = T> + Mul<Output = T>,
{
    fn pow(self, exponent: T) -> Self;
}

impl Pow<U256> for FieldElement {
    fn pow(self, exponent: U256) -> Self {
        let mut ret = FieldElement::new(U256::from(1), self.prime);
        let mut counter = exponent % (self.prime - 1);

        loop {
            if counter < U256::from(u128::MAX) {
                for _ in 0..counter.as_u128() {
                    ret = ret * self.clone();
                }
                break;
            }
            if counter >= U256::from(u128::MAX) {
                for _ in 0..u128::MAX {
                    ret = ret * self.clone();
                }
            }
            counter -= U256::from(u128::MAX);
        }
        ret
    }
}

impl Pow<i32> for FieldElement {
    fn pow(self, exponent: i32) -> Self {
        let n = if exponent < 0 {
            self.prime - 1 - U256::from(-exponent)
        } else {
            U256::from(exponent)
        };
        self.pow(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        FieldElement::new(U256::from(2), U256::from(3));
    }

    #[test]
    fn eq() {
        let a = FieldElement::new(U256::from(2), U256::from(3));
        let b = FieldElement::new(U256::from(2), U256::from(3));
        let c = FieldElement::new(U256::from(1), U256::from(3));

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn add() {
        let a = FieldElement::new(U256::from(2), U256::from(7));
        let b = FieldElement::new(U256::from(1), U256::from(7));
        let c = FieldElement::new(U256::from(3), U256::from(7));

        assert_eq!(a + b, c);
    }

    #[test]
    fn sub() {
        let a = FieldElement::new(U256::from(6), U256::from(7));
        let b = FieldElement::new(U256::from(4), U256::from(7));
        let c = FieldElement::new(U256::from(2), U256::from(7));

        assert_eq!(a - b, c);
    }

    #[test]
    fn mul() {
        let a = FieldElement::new(U256::from(3), U256::from(13));
        let b = FieldElement::new(U256::from(12), U256::from(13));
        let c = FieldElement::new(U256::from(10), U256::from(13));

        assert_eq!(a * b, c);
    }

    #[test]
    fn pow() {
        let a = FieldElement::new(U256::from(3), U256::from(13));
        let b = FieldElement::new(U256::from(1), U256::from(13));

        assert_eq!(a.pow(U256::from(3)), b);

        let c = FieldElement::new(U256::from(7), U256::from(13));
        let d = FieldElement::new(U256::from(8), U256::from(13));

        assert_eq!(c.pow(-3), d);
    }

    #[test]
    fn div() {
        let a = FieldElement::new(U256::from(7), U256::from(19));
        let b = FieldElement::new(U256::from(5), U256::from(19));
        let c = FieldElement::new(U256::from(9), U256::from(19));

        assert_eq!(a / b, c);
    }
}
