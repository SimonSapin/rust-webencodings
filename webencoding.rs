use str;
use vec;
use indexes;

const REPLACEMENT_CHARACTER: char = '\uFFFD';


fn decode_utf8(bytes: &[u8]) -> ~[char] {
    let mut code_point: u32 = 0;
    let mut lower_boundary: u8 = 0x80;
    let mut upper_boundary: u8 = 0xBF;
    let mut bytes_needed: uint = 0;

    let mut code_points = ~[];
    for vec::each(bytes) |byte_p| {
        let byte = *byte_p;
        if bytes_needed != 0 && !(
                lower_boundary <= byte && byte <= upper_boundary) {
            code_points.push(REPLACEMENT_CHARACTER);
            code_point = 0;
            lower_boundary = 0x80;
            upper_boundary = 0xBF;
            bytes_needed = 0;
        }
        if bytes_needed == 0 {
            match byte {
                0x00 .. 0x7F => code_points.push(byte as char),
                0xC2 .. 0xDF => {
                    bytes_needed = 1;
                    code_point = (byte - 0xC0) as u32 << 6;
                },
                0xE0 .. 0xEF => {
                    if byte == 0xE0 {
                        lower_boundary = 0xA0;
                    }
                    if byte == 0xED {
                        upper_boundary = 0x9F;
                    }
                    bytes_needed = 2;
                    code_point = (byte - 0xE0) as u32 << 12;
                },
                0xF0 .. 0xF4 => {
                    if byte == 0xF0 {
                        lower_boundary = 0x90;
                    }
                    if byte == 0xF4 {
                        upper_boundary = 0x8F;
                    }
                    bytes_needed = 3;
                    code_point = (byte - 0xF0) as u32 << 18;
                },
                _ => code_points.push(REPLACEMENT_CHARACTER)
            }
        } else {  // bytes_needed != 0
            bytes_needed -= 1;
            code_point += (byte - 0x80) as u32 << (6 * bytes_needed);
            if bytes_needed == 0 {
                code_points.push(code_point as char);
                code_point = 0;
                lower_boundary = 0x80;
                upper_boundary = 0xBF;
            }
        }
    }
    if bytes_needed != 0 {
        code_points.push(REPLACEMENT_CHARACTER)
    }
    code_points
}


fn encode_utf8(code_points: &[char]) -> ~[u8] {
    do code_points.flat_map |code_point| {
        let cp = *code_point as u32;
        match cp {
            0x0000 .. 0x007F => ~[cp as u8],
            0xD800 .. 0xDFFF => fail,  // Surrogate pairs
            0x0080 .. 0x07FF => ~[
                (0xC0 + (cp >> 6        )) as u8,
                (0x80 + (cp       & 0x3f)) as u8],
            0x0800 .. 0xFFFF => ~[
                (0xE0 + (cp >> 12       )) as u8,
                (0x80 + (cp >> 6  & 0x3f)) as u8,
                (0x80 + (cp       & 0x3f)) as u8],
            0x10000 .. 0x10FFFF => ~[
                (0xF0 + (cp >> 18       )) as u8,
                (0x80 + (cp >> 12 & 0x3f)) as u8,
                (0x80 + (cp >> 6  & 0x3f)) as u8,
                (0x80 + (cp       & 0x3f)) as u8],
            _ => fail
        }
    }
}


fn decode_windows1252(bytes: &[u8]) -> ~[char] {
    do bytes.map |byte| {
        if *byte <= 0x7F { *byte as char }
        else { indexes::windows1252[*byte - 0x80] }
    }
}


fn encode_windows1252(code_points: &[char]) -> ~[u8] {
    do code_points.map |cp| {
        if *cp <= '\x7F' {
            *cp as u8
        } else {
            (indexes::windows1252.position(|v| {*v == *cp}).get() + 0x80) as u8
        }
    }
}

type Encoding = {
    name: &str,
    encoder: fn (&[char]) -> ~[u8],
    decoder: fn (&[u8]) -> ~[char],
};

const UTF8: Encoding = {
    name: "utf-8",
    encoder: encode_utf8,
    decoder: decode_utf8,
};


const Windows1252: Encoding = {
    name: "windows-1252",
    encoder: encode_windows1252,
    decoder: decode_windows1252,
};


#[cfg(test)]
mod tests {
    use cmp::Eq;

    fn assert_equals<T: Eq>(message: &str, a: &T, b: &T) {
        if a != b {
            fail fmt!("%s: %? != %?", message, a, b);
        }
    }

    fn test_codec(encoding: Encoding, code_points: &[char], bytes: &[u8]) {
        let encoded: &[u8] = (encoding.encoder)(code_points);
        let decoded: &[char] = (encoding.decoder)(bytes);
        assert_equals("Encoding", &encoded, &bytes);
        assert_equals("Decoding", &decoded, &code_points);
    }

    #[test]
    fn test_windows1252() {
        test_codec(Windows1252, ['H', '€', 'l', 'l', 'ö'],
                   [72, 128, 108, 108, 246]);
    }

    #[test]
    #[should_fail]
    fn test_invalid_windows1252() {
        (Windows1252.encoder)(['今', '日', 'は']);
    }

    #[test]
    fn test_utf8() {
        test_codec(UTF8, ['H', '€', 'l', 'l', 'ö'],
                   [72, 226, 130, 172, 108, 108, 195, 182]);
        test_codec(UTF8, ['今', '日', 'は'],
                   [228, 187, 138, 230, 151, 165, 227, 129, 175]);
        let decoded: &[char] = (UTF8.decoder)(
            [72, 226, 130, 255, 108, 108, 195, 182]);
        assert_equals("Decoding errors", &decoded,
                      & &['H', '�', '�', 'l', 'l', 'ö'])
    }
}
