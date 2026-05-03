use crate::Mode;

pub fn caesar(text: &str, key: u64, action: Mode) -> String {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_caesar_encrypt() {
        assert_eq!("bcdefghijklmnopqrstuvwxyza", caesar("abcdefghijklmnopqrstuvwxyz", 1, Mode::Encrypt));
        assert_eq!("gghijklmno1pqjrs7tuvw3ixyzabcdef", caesar("aabcdefghi1jkdlm7nopq3crstuvwxyz", 6, Mode::Encrypt));
    }

    #[test]
    fn test_caesar_decrypt() {
        assert_eq!("abcdefghijklmnopqrstuvwxyz", caesar("bcdefghijklmnopqrstuvwxyza", 1, Mode::Decrypt));
        assert_eq!("aabcdefghi1jkdlm7nopq3crstuvwxyz", caesar("gghijklmno1pqjrs7tuvw3ixyzabcdef", 6, Mode::Decrypt));
    }
}