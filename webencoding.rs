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
            decoder: fn (&[u8]) -> ~[char],
            encoder: fn (&[char]) -> ~[u8],
            bytes: &[u8], string: &str) {
        let code_points: &[char] = str::chars(string);
        let decoded: &[char] = decoder(bytes);
        let encoded: &[u8] = encoder(code_points);
        assert decoded == code_points;
        assert encoded == bytes;
    }

    #[test]
    fn test_windows1252() {
        test_codec(decode_windows1252, encode_windows1252,
            &[72, 128, 108, 108, 246], &"H€llö");
    }
}
