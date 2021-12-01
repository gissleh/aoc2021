pub fn parse_u64(s: &str) -> u64 {
    let mut res = 0;

    for c in s.bytes() {
        res *= 10;
        res += (c - b'0') as u64;
    }

    res
}

pub fn parse_u32(s: &str) -> u32 {
    let mut res = 0;

    for c in s.bytes() {
        res *= 10;
        res += (c - b'0') as u32;
    }

    res
}

pub fn parse_u8(s: &str) -> u8 {
    let mut res = 0;

    for c in s.bytes() {
        res *= 10;
        res += (c - b'0') as u8;
    }

    res
}

pub fn parse_usize(s: &str) -> usize {
    let mut res = 0;

    for c in s.bytes() {
        res *= 10;
        res += (c - b'0') as usize;
    }

    res
}
