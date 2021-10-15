
pub fn encode(input: &str) -> String {
    base64::encode(input)
}

pub fn decode(input: &str) -> String {
    let byte = base64::decode(input).unwrap();
    byte.iter().map(|&b| b as char).collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_encode() {
        let test_word = "rust";
        let res = encode(&test_word);

        // TODO: HashMapとかでお題と答えを持っておきたい
        assert_eq!(res, "cnVzdA==");
    }

    #[test]
    fn test_decode() {
        let test_word = "cnVzdA==";
        let res = decode(&test_word);

        assert_eq!(res, "rust");
    }
}