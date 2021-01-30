use crate::error::ArithmeticError;
use crate::BalTer;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

/// Simple 1-trit addition arithmetic
///
/// This functions serves as a universal trit addition function
/// It takes in the 2 trits and returns the output and the carry
///
/// ```
/// use sbtcvm_balter::arithmetic::trit_addition;
///
/// assert_eq!(('-', '+'), trit_adder('+','+'))
/// ```
fn trit_adder(c1: char, c2: char, carry: char) -> Result<(char, char), ArithmeticError> {
    // The order of intermediate is (-1, 0, 1)
    let mut intermediate: (i8, i8, i8) = (0, 0, 0);
    [&c1, &c2, &carry].iter().for_each(|i| match i {
        '-' => intermediate.0 += 1,
        '0' => intermediate.1 += 1,
        '+' => intermediate.2 += 1,
        _ => {}
    });
    match intermediate {
        (1, 1, 1) | (0, 3, 0) => Ok(('0', '0')),
        (2, 0, 1) | (1, 2, 0) => Ok(('-', '0')),
        (0, 2, 1) | (1, 0, 2) => Ok(('+', '0')),
        (0, 1, 2) => Ok(('-', '+')),
        (0, 0, 3) => Ok(('0', '+')),
        (3, 0, 0) => Ok(('0', '-')),
        (2, 1, 0) => Ok(('+', '-')),
        (_, _, _) => Err(ArithmeticError::InvalidInput),
    }
}

impl Add for BalTer {
    type Output = Result<(BalTer, char), ArithmeticError>;
    // TODO: Implement global overflow flag as with operator overloading we cannot check.
    fn add(mut self, rhs: Self) -> Self::Output {
        let mut carry: char = '0'; // '+', '0', '-'
        for i in (0..self.value.len()).rev() {
            let (res, c) = trit_adder(self.value[i], rhs.value[i], carry)?;
            carry = c;
            self.value[i] = res
        }
        Ok((self, carry))
    }
}

impl AddAssign for BalTer {
    fn add_assign(&mut self, rhs: Self) {
        unimplemented!()
    }
}

impl Div for BalTer {
    type Output = BalTer;

    fn div(self, rhs: Self) -> Self::Output {
        unimplemented!()
    }
}

impl DivAssign for BalTer {
    fn div_assign(&mut self, rhs: Self) {
        unimplemented!()
    }
}

impl Mul for BalTer {
    type Output = BalTer;

    fn mul(self, rhs: Self) -> Self::Output {
        unimplemented!()
    }
}

impl MulAssign for BalTer {
    fn mul_assign(&mut self, rhs: Self) {
        unimplemented!()
    }
}

impl Rem for BalTer {
    type Output = BalTer;

    fn rem(self, rhs: Self) -> Self::Output {
        unimplemented!()
    }
}

impl RemAssign for BalTer {
    fn rem_assign(&mut self, rhs: Self) {
        unimplemented!()
    }
}

impl Sub for BalTer {
    type Output = Result<(BalTer, char), ArithmeticError>;

    fn sub(self, rhs: Self) -> Self::Output {
        Ok(self.add((!rhs)?)?)
    }
}

impl SubAssign for BalTer {
    fn sub_assign(&mut self, rhs: Self) {
        unimplemented!()
    }
}
