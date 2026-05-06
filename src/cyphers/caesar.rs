use crate::Mode;

/// #### Encrypts or decrypts using Caesar cypher depending on the Mode enum.
///  
/// ## Panic
/// 
/// #### this function doesn't panic but does
///  - ignores every non ASCII char and passes it to output without encrypting it.
///  - converts every ASCII char to lowercase.
/// 
/// # Examples
///```
/// let encrypt: String =  caesar("abc", 1, Mode::Encrypt); // -> bcd
/// let decrypt: String =  caesar("bcd", 1, Mode::Decrypt); // -> abc
///```
pub fn caesar(text: &str, key: u8, action: Mode) -> String {
    if text.is_empty() || key == 0 { return String::from(text) }
    
    let mut result = String::new();
    for char in text.to_ascii_uppercase().chars() {
        // checks if the current text char are a valid ASCII characters, if not returns unedited text char.
        if !char.is_ascii_alphabetic() { 
            result.push(char); 
            continue;
        }

        // converts ASCII char into index numbers so a = 0, b = 1, c = 2...
        let char_index = char.to_ascii_lowercase() as u8 - 97;
        // encodes or decodes text depending on the mode. uses caesar cypher.
        match action {
            Mode::Encrypt => { result.push((((char_index + key)      % 26) + 97) as char) }
            Mode::Decrypt => { result.push((((char_index + 26 - key) % 26) + 97) as char) }
        }
    }
    return result;
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