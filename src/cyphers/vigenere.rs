use crate::Mode;

/// #### Encrypts or decrypts using Vigenere cypher depending on the Mode enum.
///  
/// ## Panic
/// 
/// #### this function doesn't panic but does
///  - ignores every non ASCII char and passes it to output without encrypting it.
///  - converts every ASCII char to lowercase.
/// 
/// # Examples
///```
/// let encrypt: String = vigenere("test", "rust", Mode::Encrypt); // -> kykm
/// let decrypt: String = vigenere("kykm", "rust", Mode::Decrypt); // -> test
///```
pub fn vigenere(text: &str, key: &str, action: Mode) -> String {
    if text.is_empty() || key.is_empty() { return String::from(text) }
    
    let mut result = String::new();
    // used for getting the right index of key. 
    let mut count = 0;

    for char in text.to_ascii_uppercase().chars() {
        // checks if both the current text char and key char are a valid asci characters, if not returns unedited text char.
        if !char.is_ascii_alphabetic() || !key.chars().nth(count% key.len()).unwrap().is_ascii_alphabetic() {
            result.push(char);
            continue;
        }

        // converts ASCII char's into index numbers so a = 0, b = 1, c = 2...
        let key_char_index = key.chars().nth(count % key.len()).unwrap().to_ascii_lowercase() as u8 - 97;
        let char_index = char.to_ascii_lowercase() as u8 - 97;
        // encodes or decodes text depending on the mode. uses vigenere cypher.
        match action {
            Mode::Encrypt => { result.push( (((char_index + key_char_index)      % 26) + 97) as char) }
            Mode::Decrypt => { result.push( (((char_index + 26 - key_char_index) % 26) + 97) as char) }
        }
        count += 1;
    }
    return  result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vigenere_encrypt() {
        assert_eq!(String::from("a"), vigenere("a", "a", Mode::Encrypt));
        assert_eq!(String::from("y"), vigenere("z", "z", Mode::Encrypt));
        assert_eq!(String::from("vyc fnqkm spdpv nqo hjfxa qmcg 13 eiha umvl."), vigenere("The quick brown fox jumps over 13 lazy dogs.", "cryptii", Mode::Encrypt));
    }

    #[test]
    fn test_vigenere_decode() {
        assert_eq!(String::from("a"), vigenere("a", "a", Mode::Decrypt));
        assert_eq!(String::from("z"), vigenere("y", "z", Mode::Decrypt));
        assert_eq!(String::from("the quick brown fox jumps over 13 lazy dogs."), vigenere("vyc fnqkm spdpv nqo hjfxa qmcg 13 eiha umvl.", "cryptii", Mode::Decrypt));
    }
}