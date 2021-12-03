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

pub fn parse_binary_u32(s: &str) -> u32 {
    let mut res = 0;

    for c in s.bytes() {
        res *= 2;
        res += if c == b'1' {1} else {0};
    }

    res
}

pub fn parse_u32_list(s: &[u8]) -> Vec<u32> {
    let mut curr = 0;
    let mut res = Vec::new();
    let mut active = false;

    for b in s.iter() {
        match *b {
            b'0'..=b'9' => {
                active = true;
                curr *= 10;
                curr += (b - b'0') as u32;
            }
            _ => {
                if active {
                    res.push(curr);
                    curr = 0;
                }
                active = false;
            }
        }
    }

    if active {
        res.push(curr);
    }

    res
}