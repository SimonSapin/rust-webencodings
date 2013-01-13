use str;
use vec;
use indexes;

const REPLACEMENT_CHARACTER: char = '\uFFFD';


fn decode_windows1252(bytes: &[u8]) -> ~[char] {
    let mut code_points = ~[];
    for vec::each(bytes) |byte| {
        code_points.push(
            if *byte <= 0x7F { *byte as char }
            else { indexes::windows1252[*byte - 0x80] }
        )
    }
    code_points
}


fn encode_windows1252(code_points: &[char]) -> ~[u8] {
    let mut bytes = ~[];
    for vec::each(code_points) |code_point| {
        bytes.push(match *code_point {
            code_point if code_point <= '\x7F' => code_point as u8,
            code_point =>
                match indexes::windows1252.position(|v| {*v == code_point}) {
                    Some(index) => (index + 0x80) as u8,
                    _ => fail
                }
        })
    }
    bytes
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
