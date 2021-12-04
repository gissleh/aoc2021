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
    let mut res = Vec::new();
    parse_u32_list_into(s, &mut res);

    res
}

pub fn parse_u32_list_into(s: &[u8], vec: &mut Vec<u32>) {
    let mut curr = 0;
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
                    vec.push(curr);
                    curr = 0;
                }
                active = false;
            }
        }
    }

    if active {
        vec.push(curr);
    }
}

pub fn parse_u32s_amount(s: &[u8], vec: &mut Vec<u32>, amount: usize) -> usize {
    let mut curr = 0;
    let mut active = false;

    for (i, b) in s.iter().enumerate() {
        match *b {
            b'0'..=b'9' => {
                active = true;
                curr *= 10;
                curr += (b - b'0') as u32;
            }
            _ => {
                if active {
                    vec.push(curr);
                    curr = 0;

                    if vec.len() == amount {
                        return i + 1;
                    }
                }
                active = false;
            }
        }
    }

    if active {
        vec.push(curr);
    }

    vec.len()
}

pub fn parse_u32s_until(s: &[u8], vec: &mut Vec<u32>, stop: u8) -> usize {
    let mut curr = 0;
    let mut active = false;

    for (i, b) in s.iter().enumerate() {
        if *b == stop {
            return i+1;
        }

        match *b {
            b'0'..=b'9' => {
                active = true;
                curr *= 10;
                curr += (b - b'0') as u32;
            }
            _ => {
                if active {
                    vec.push(curr);
                    curr = 0;
                }
                active = false;
            }
        }
    }

    if active {
        vec.push(curr);
    }

    s.len()
}