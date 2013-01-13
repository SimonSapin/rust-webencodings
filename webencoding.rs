use str;
use vec;
use indexes;

const REPLACEMENT_CHARACTER: char = '\uFFFD';


fn decode_windows1252(bytes: &[u8]) -> ~[char] {
    bytes.map(|byte| {
        if *byte <= 0x7F { *byte as char }
        else { indexes::windows1252[*byte - 0x80] }
    })
}


fn encode_windows1252(code_points: &[char]) -> ~[u8] {
    code_points.map(|cp| {
        if *cp <= '\x7F' {
            *cp as u8
        } else {
            (indexes::windows1252.position(|v| {*v == *cp}).get() + 0x80) as u8
        }
    })
}


#[cfg(test)]
mod tests {
    fn test_codec(
            encoder: fn (&[char]) -> ~[u8],
            decoder: fn (&[u8]) -> ~[char],
            string: &str, bytes: &[u8]) {
        let code_points: &[char] = str::chars(string);
        let encoded: &[u8] = encoder(code_points);
        let decoded: &[char] = decoder(bytes);
        assert encoded == bytes;
        assert decoded == code_points;
    }

    #[test]
    fn test_windows1252() {
        test_codec(encode_windows1252, decode_windows1252,
            "H€llö", [72, 128, 108, 108, 246]);
    }

    #[test]
    #[should_fail]
    fn test_invalid_windows1252() {
        encode_windows1252(str::chars("今日は"));
    }
}
