use std::cmp::min;

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

pub fn parse_u32b(s: &[u8]) -> u32 {
    let mut res = 0;

    for c in s.iter() {
        res *= 10;
        res += (*c - b'0') as u32;
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
        res += if c == b'1' { 1 } else { 0 };
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

pub fn parse_i32s_amount(s: &[u8], dst: &mut [i32], amount: usize) -> usize {
    let mut curr = 0;
    let mut active = false;
    let mut index = 0;
    let mut neg = false;
    let amount = min(dst.len(), amount);

    for (i, b) in s.iter().enumerate() {
        match *b {
            b'0'..=b'9' => {
                active = true;
                curr *= 10;
                curr += (b - b'0') as i32;
            }
            b'-' => {
                neg = true;
            }
            _ => {
                if active {
                    dst[index] = if neg { -curr } else { curr };
                    index += 1;
                    curr = 0;
                    neg = false;

                    if index == amount {
                        return i + 1;
                    }
                }
                active = false;
            }
        }
    }

    if active {
        dst[index] = curr;
    }

    dst.len()
}


pub fn parse_u32_pair(s: &[u8]) -> (u32, u32) {
    let mut curr = 0;
    let mut active = false;
    let mut res = [0u32; 2];
    let mut res_index = 0;

    for b in s.iter() {
        if res_index == 2 {
            break;
        }

        match *b {
            b'0'..=b'9' => {
                active = true;
                curr *= 10;
                curr += (b - b'0') as u32;
            }
            _ => {
                if active {
                    res[res_index] = curr;
                    res_index += 1;
                    curr = 0;
                }
                active = false;
            }
        }
    }

    if active {
        res[res_index] = curr;
    }

    (res[0], res[1])
}

pub fn parse_u32s_until(s: &[u8], vec: &mut Vec<u32>, stop: u8) -> usize {
    let mut curr = 0;
    let mut active = false;

    for (i, b) in s.iter().enumerate() {
        if *b == stop {
            return i + 1;
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

pub fn parse_hex(hex: u8) -> u8 {
    match hex {
        b'0'..=b'9' => hex - b'0',
        b'A'..=b'F' => (hex - b'A') + 10,
        _ => unreachable!(),
    }
}

const BR_MASKS: [u8; 9] = [
    0b00000000,
    0b10000000,
    0b11000000,
    0b11100000,
    0b11110000,
    0b11111000,
    0b11111100,
    0b11111110,
    0b11111111,
];

pub struct BitReader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> BitReader<'a> {
    pub fn set_pos(&mut self, v: usize) {
        self.pos = v;
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn read(&mut self, n: usize) -> u32 {
        let mut remaining = n;
        let mut res = 0;
        while remaining > 0 {
            let i = self.pos / 8;
            let j = self.pos % 8;
            let read_size = min(remaining, 8 - j);
            let mask = BR_MASKS[read_size] >> j;
            let shift = 8 - j - read_size;
            let value = ((self.data[i] & mask) >> shift) as u32;

            #[cfg(test)]
            println!("remaining={} i={} j={} shift={} read_size={} mask={:b} = {}",
                     remaining, i, j, shift, read_size, mask, value);

            res <<= read_size;
            res |= value;

            self.pos += read_size;
            remaining -= read_size;
        }

        res
    }

    pub fn new(data: &'a [u8], pos: usize) -> BitReader<'a> {
        BitReader { data, pos }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reader() {
        let data = vec![0xff, 0xff, 0x00, 0xab, 0xcf, 0x12, 0x34, 0xff, 0xa4, 0xf0];
        let mut reader = BitReader::new(&data, 0);

        assert_eq!(reader.read(4), 0xf);
        assert_eq!(reader.read(2), 0x3);
        assert_eq!(reader.read(3), 0x7);
        assert_eq!(reader.read(3), 0x7);
        assert_eq!(reader.read(4), 0xf);
        assert_eq!(reader.read(4), 0x0);
        assert_eq!(reader.read(4), 0x0);
        assert_eq!(reader.read(4), 0xa);
        assert_eq!(reader.read(4), 0xb);
        assert_eq!(reader.read(8), 0xcf);
        assert_eq!(reader.read(16), 0x1234);
        assert_eq!(reader.read(24), 0xffa4f0);
    }
}
