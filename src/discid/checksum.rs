use crate::core::digit::Digits;

const EAN_13_WEIGHTS: [u8; 12] = [1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3];
const UPC_A_WEIGHTS: [u8; 11] = [3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3];

/// Checksum with alternating weights used in EAN and UPC-A barcodes
#[inline]
fn calc_checksum<const L: usize>(digits: &Digits<L>, weights: &[u8; L]) -> u8 {
  let mut sum: u32 = 0;

  for (digit, weight) in digits.as_bytes().iter().zip(weights) {
    sum += (*digit as u32) * (*weight as u32);
  }

  let modulus = sum % 10;

  if modulus == 0 {
    0
  } else {
    (10 - modulus) as u8
  }
}

pub fn calc_ean_13_checksum(digits: &Digits<12>) -> u8 {
  calc_checksum(digits, &EAN_13_WEIGHTS)
}

pub fn calc_upc_a_checksum(digits: &Digits<11>) -> u8 {
  calc_checksum(digits, &UPC_A_WEIGHTS)
}
