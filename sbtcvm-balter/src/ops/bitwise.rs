use crate::error::ArithmeticError;
use crate::BalTer;
use std::ops::Not;

impl Not for BalTer {
    type Output = Result<BalTer, ArithmeticError>;

    fn not(self) -> Self::Output {
        let mut out = BalTer::default();
        for (i, val) in self.value.iter().enumerate() {
            match val {
                '+' => out.value[i] = '-',
                '0' => out.value[i] = '0',
                '-' => out.value[i] = '+',
                _ => return Err(ArithmeticError::InvalidInput),
            }
        }
        Ok(out)
    }
}
