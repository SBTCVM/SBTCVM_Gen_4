#![feature(or_patterns)]
#![feature(destructuring_assignment)]

use crate::error::MachineError;

mod error;
pub mod ops;

// ALL data values in BalTer are in LITTLE-ENDIAN format
// 1 BalTer will be 1 word long // currently only 1 tryte
// Word size is 24 trits == 1 quadtryte
#[derive(Clone, Copy)]
struct BalTer {
    value: [char; 6],
}

impl BalTer {
    fn new(&self) {}
    fn from_i128(number: i128) -> Result<Vec<BalTer>, MachineError> {
        dec_to_bal(number)
    }
}

impl Default for BalTer {
    fn default() -> Self {
        BalTer {
            value: ['0', '0', '0', '0', '0', '0'],
        }
    }
}

fn dec_to_bal(mut dec: i128) -> Result<Vec<BalTer>, MachineError> {
    let mut out: Vec<BalTer> = vec![];
    let mut idx_vec: usize = 6;
    let mut idx_balter: usize = 0;
    while dec != 0 {
        if idx_vec == 0 {
            idx_vec = 6;
            out.push(BalTer { value: ['0'; 6] });
            idx_balter += 1;
            continue;
        };
        match dec % 3 {
            0 => out[idx_balter].value[idx_vec] = '0',
            1 => out[idx_balter].value[idx_vec] = '+',
            2 => out[idx_balter].value[idx_vec] = '-',
            _ => return Err(MachineError::DecConversion),
        }
        dec = (dec + 1) / 3
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
