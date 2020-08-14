/// Calculate the Josephus Number J(n) for any n > 0.
///
/// We use a simple formula here: express the number `n` as
/// 2^m + l for some m >= 0, 0 <= l < 2^m, then calculate the
/// value of 2 * l + 1, which is the result.
///
/// * `n` - The number to calculate J for
pub fn josephus(n: u32) -> u32 {
    let base: u32 = 2;

    let mut index: u32 = 0;
    let mut hbits: u32 = n;

    loop {
        hbits = hbits >> 1;
        if hbits == 0 {
            break;
        }
        index += 1;
    }

    let l = n - base.pow(index);

    2 * l + 1
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(josephus(1), 1);
    }

    #[test]
    fn test_three() {
        assert_eq!(josephus(3), 3);
    }

    #[test]
    fn test_twelve() {
        assert_eq!(josephus(12), 9);
    }

    #[test]
    fn test_fifty_one() {
        assert_eq!(josephus(51), 39);
    }

    #[test]
    fn test_sixty_four() {
        assert_eq!(josephus(64), 1);
    }
}
