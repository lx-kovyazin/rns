use core::slice;
use static_assertions::const_assert;

use gcd::euclid_u8;

const fn gcd_mul_fold(acc: u8, head: u8, others: &[u8]) -> u8 {
    match others.split_first() {
        None => acc,
        Some((&next, others)) => gcd_mul_fold(acc * euclid_u8(head, next), head, others),
    }
}

const fn is_pairwise_coprime(numbers: &[u8]) -> bool {
    match numbers.split_first() {
        None => false,
        Some((&head, others)) => gcd_mul_fold(1, head, others) == 1,
    }
}

const fn max(modules: &[u8]) -> usize {
    match modules.split_first() {
        None => 1,
        Some((&head, others)) => head as usize * max(others),
    }
}

#[allow(dead_code)]
fn check() {
    const_assert!(is_pairwise_coprime(&[3, 5, 7]));
}

pub struct Rns<'a> {
    mod_system: &'a [u8],
}


#[cfg(test)]
mod tests {
    use static_assertions::const_assert_eq;

    use super::*;

    #[test]
    fn it_works() {
        const_assert!(is_pairwise_coprime(&[3, 5, 7]));
        const_assert!(is_pairwise_coprime(&[193, 197, 199]));
        const_assert_eq!(max(&[3, 5, 7]), 105);
        const_assert_eq!(max(&[193, 197, 199]), 7_566_179);
    }
}
