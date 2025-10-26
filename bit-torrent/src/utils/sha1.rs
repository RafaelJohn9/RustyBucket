use std::fmt::Write;

/// Compute SHA-1 digest of input bytes. Returns 20-byte array.
pub fn sha1(data: &[u8]) -> [u8; 20] {
    // Initial SHA-1 state
    let mut h0: u32 = 0x67452301;
    let mut h1: u32 = 0xEFCDAB89;
    let mut h2: u32 = 0x98BADCFE;
    let mut h3: u32 = 0x10325476;
    let mut h4: u32 = 0xC3D2E1F0;

    // Pre-processing: padding
    let bit_len = (data.len() as u64) * 8;
    let mut padded = Vec::with_capacity(((data.len() + 8) / 64 + 2) * 64);
    padded.extend_from_slice(data);
    padded.push(0x80u8);
    while (padded.len() % 64) != 56 {
        padded.push(0x00);
    }
    padded.extend_from_slice(&bit_len.to_be_bytes());

    // Process each 512-bit chunk
    for chunk in padded.chunks(64) {
        let mut w = [0u32; 80];
        // break chunk into sixteen 32-bit BE words
        for (i, word) in chunk.chunks(4).take(16).enumerate() {
            w[i] = u32::from_be_bytes([word[0], word[1], word[2], word[3]]);
        }
        // extend to 80 words
        for t in 16..80 {
            let tmp = w[t - 3] ^ w[t - 8] ^ w[t - 14] ^ w[t - 16];
            w[t] = tmp.rotate_left(1);
        }

        // Initialize working variables
        let mut a = h0;
        let mut b = h1;
        let mut c = h2;
        let mut d = h3;
        let mut e = h4;

        for t in 0..80 {
            let (f, k) = match t {
                0..=19 => ((b & c) | ((!b) & d), 0x5A827999),
                20..=39 => (b ^ c ^ d, 0x6ED9EBA1),
                40..=59 => ((b & c) | (b & d) | (c & d), 0x8F1BBCDC),
                _ => (b ^ c ^ d, 0xCA62C1D6),
            };
            let temp = a
                .rotate_left(5)
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(k)
                .wrapping_add(w[t]);
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = temp;
        }

        h0 = h0.wrapping_add(a);
        h1 = h1.wrapping_add(b);
        h2 = h2.wrapping_add(c);
        h3 = h3.wrapping_add(d);
        h4 = h4.wrapping_add(e);
    }

    let mut digest = [0u8; 20];
    digest[0..4].copy_from_slice(&h0.to_be_bytes());
    digest[4..8].copy_from_slice(&h1.to_be_bytes());
    digest[8..12].copy_from_slice(&h2.to_be_bytes());
    digest[12..16].copy_from_slice(&h3.to_be_bytes());
    digest[16..20].copy_from_slice(&h4.to_be_bytes());
    digest
}

/// Return lowercase hex representation of SHA-1 digest for input bytes.
pub fn sha1_hex(data: &[u8]) -> String {
    bytes_to_hex(&sha1(data))
}

/// Convert 20-byte SHA-1 digest to lowercase hex string.
pub fn bytes_to_hex(digest: &[u8; 20]) -> String {
    let mut s = String::with_capacity(40);
    for b in digest.iter() {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}

/// Parse 40-hex-digit SHA-1 string into 20-byte array. Returns None on parse error.
pub fn hex_to_bytes(s: &str) -> Option<[u8; 20]> {
    let s = s.trim();
    if s.len() != 40 {
        return None;
    }
    let mut out = [0u8; 20];
    for i in 0..20 {
        let hi = hex_val(s.as_bytes()[i * 2])?;
        let lo = hex_val(s.as_bytes()[i * 2 + 1])?;
        out[i] = (hi << 4) | lo;
    }
    Some(out)
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha1_empty() {
        let d = sha1(b"");
        assert_eq!(
            bytes_to_hex(&d),
            "da39a3ee5e6b4b0d3255bfef95601890afd80709".to_string()
        );
    }

    #[test]
    fn test_sha1_abc() {
        let d = sha1(b"abc");
        assert_eq!(
            bytes_to_hex(&d),
            "a9993e364706816aba3e25717850c26c9cd0d89d".to_string()
        );
    }

    #[test]
    fn test_hex_roundtrip() {
        let src = b"the quick brown fox jumps over the lazy dog";
        let hex = sha1_hex(src);
        let parsed = hex_to_bytes(&hex).unwrap();
        let digest = sha1(src);
        assert_eq!(parsed, digest);
    }

    #[test]
    fn test_invalid_hex() {
        assert!(hex_to_bytes("zz").is_none());
        assert!(hex_to_bytes("a".repeat(39).as_str()).is_none());
    }
}