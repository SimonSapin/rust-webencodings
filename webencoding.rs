use vec;
use indexes;

const REPLACEMENT_CHARACTER: u32 = 0xFFFD;


fn decode_windows1252(bytes: &[u8]) -> ~[u32] {
    let mut code_points = ~[];
    for vec::each(bytes) |byte| {
        code_points.push(match *byte {
            byte if byte <= 0x7F => byte as u32,
            byte => match indexes::windows1252[byte - 0x80] {
                code_point if code_point >= 0 => code_point as u32,
                _ => REPLACEMENT_CHARACTER, // Not in the index.
            }
        })
    }
    code_points
}


fn encode_windows1252(code_points: &[u32]) -> ~[u8] {
    let mut bytes = ~[];
    for vec::each(code_points) |code_point| {
        bytes.push(match *code_point as i32 {
            code_point if code_point <= 0x7F => code_point as u8,
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
            decoder: fn (&[u8]) -> ~[u32],
            encoder: fn (&[u32]) -> ~[u8],
            bytes: &[u8], code_points: &[u32]) {
        let decoded: &[u32] = decoder(bytes);
        let encoded: &[u8] = encoder(code_points);
        assert decoded == code_points;
        assert encoded == bytes;
    }

    #[test]
    fn test_windows1252() {
        test_codec(decode_windows1252, encode_windows1252,
            &[72, 128, 108, 108, 246], &[72, 8364, 108, 108, 246]);
    }
}
