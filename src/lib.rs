const ALPHABET_SIZE: usize = 62;
const ALPHABET: [char; ALPHABET_SIZE] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b',
    'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u',
    'v', 'w', 'x', 'y', 'z',
];

/*
ALPHABET_VERT: [u8; 256] = [0xff: 256];
for (i, &v) in ALPHABET.iter().enumerate() {
    ALPHABET_VERT[v as usize] = i as u8;
}
 */
const ALPHABET_VERT: [u8; 256] = [
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 255, 255, 255,
    255, 255, 255, 255, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28,
    29, 30, 31, 32, 33, 34, 35, 255, 255, 255, 255, 255, 255, 36, 37, 38, 39, 40, 41, 42, 43, 44,
    45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
];

pub fn encode(src: &[u8]) -> String {
    if src.is_empty() {
        return "".to_string();
    }
    let cap = ((256f64.log2() * src.len() as f64) / (ALPHABET_SIZE as f64).log2()).ceil() as usize;
    let mut dst = vec![0u8; cap];
    for &b in src.iter() {
        let mut carry = b as usize;
        for j in (0..cap).rev() {
            if carry == 0 {
                break;
            }
            carry += 256 * dst[j] as usize;
            dst[j] = (carry % ALPHABET_SIZE) as u8;
            carry /= ALPHABET_SIZE;
        }
    }
    let mut skip: usize = 0;
    for &v in dst.iter() {
        if v != 0 {
            break;
        }
        skip += 1;
    }
    dst.iter()
        .skip(skip)
        .map(|&i| ALPHABET[i as usize])
        .collect()
}

#[derive(Debug)]
pub enum Error {
    BadInput { byte: u8 },
}
pub fn decode(src: &[u8]) -> Result<Vec<u8>, Error> {
    if src.is_empty() {
        return Ok(vec![]);
    }
    let cap = (((ALPHABET_SIZE as f64).log2() * src.len() as f64) / 256f64.log2()).ceil() as usize;
    let mut dst = vec![0u8; cap];
    for &b in src.iter() {
        let mut carry: usize = ALPHABET_VERT[b as usize] as usize;
        if carry == 255 {
            return Err(Error::BadInput { byte: b });
        }
        for j in (0..cap).rev() {
            if carry == 0 {
                break;
            }
            carry += ALPHABET_SIZE * (dst[j] as usize);
            dst[j] = (carry % 256) as u8;
            carry /= 256;
        }
    }
    let mut skip: usize = 0;
    for &v in dst.iter() {
        if v != 0 {
            break;
        }
        skip += 1;
    }
    Ok(dst.iter().skip(skip).map(|&i| i).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!("", encode("".as_bytes()));
        assert_eq!("5TP3P3v", encode("Hello".as_bytes()));
        assert_eq!("T8dgcjRVwXIMmiea", encode("Hello, world".as_bytes()));
        assert_eq!("73XpUgzMGAjX6SV", encode("Hello中文".as_bytes()));
        {
            let text = decode("5TP3P3v".as_bytes()).unwrap();
            assert_eq!("Hello", String::from_utf8(text).unwrap());
        }
        {
            let text = decode("73XpUgzMGAjX6SV".as_bytes()).unwrap();
            assert_eq!("Hello中文", String::from_utf8(text).unwrap());
        }
        {
            let text = decode("73XpUgzMGA-jX6SV".as_bytes());
            assert!(text.is_err());
        }
    }
}
