use num::Integer;
use std::ops::Neg;

pub struct Parser<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn parse_slice(&mut self, parser: impl Fn(&'a [u8]) -> Option<(&'a [u8], usize)>) -> Option<&'a [u8]> {
        if let Some((res, len)) = parser(&self.data[self.pos..]) {
            self.pos += len;
            Some(res)
        } else {
            None
        }
    }

    pub fn peek_slice(&mut self, parser: impl Fn(&'a [u8]) -> Option<(&'a [u8], usize)>) -> Option<&'a [u8]> {
        if let Some((res, _)) = parser(&self.data[self.pos..]) {
            Some(res)
        } else {
            None
        }
    }

    pub fn parse<T>(&mut self, parser: impl Fn(&[u8]) -> Option<(T, usize)>) -> Option<T> {
        if let Some((res, len)) = parser(&self.data[self.pos..]) {
            self.pos += len;
            Some(res)
        } else {
            None
        }
    }

    pub fn peek<T>(&mut self, parser: impl Fn(&[u8]) -> Option<(T, usize)>) -> Option<T> {
        if let Some((res, _)) = parser(&self.data[self.pos..]) {
            Some(res)
        } else {
            None
        }
    }

    pub fn helper(self) -> ParserHelper<'a> {
        ParserHelper { parser: Parser { data: self.data, pos: self.pos } }
    }

    pub fn new_at(data: &'a [u8], pos: usize) -> Parser<'a> {
        Parser { data, pos }
    }

    pub fn new(data: &'a [u8]) -> Parser<'a> {
        Self::new_at(data, 0)
    }
}

pub struct ParserHelper<'a> {
    parser: Parser<'a>,
}

impl<'a> ParserHelper<'a> {
    pub fn parser(&'a mut self) -> &'a mut Parser {
        &mut self.parser
    }

    pub fn int<T: Integer + Copy + From<u8> + Neg<Output=T>>(&mut self, greedy: bool) -> Option<T> {
        if greedy {
            self.parser.parse(eat_non_numeric_signed);
        }

        self.parser.parse(parse_int)
    }

    pub fn uint<T: Integer + Copy + From<u8>>(&mut self, greedy: bool) -> Option<T> {
        if greedy {
            self.parser.parse(eat_non_numeric_unsigned);
        }

        self.parser.parse(parse_uint)
    }

    pub fn hex_byte(&mut self, allow_half: bool) -> Option<u8> {
        if let Some(res) = self.parser.parse(parse_hex_byte) {
            return Some(res);
        } else if allow_half {
            if let Some(res) = self.parser.parse(parse_hex_number) {
                return Some(res << 4);
            }
        }

        None
    }

    pub fn hex_number(&mut self) -> Option<u8> {
        self.parser.parse(parse_hex_number)
    }

    pub fn line(&mut self) -> Option<&'a [u8]> {
        self.parser.parse_slice(parse_line)
    }

    pub fn skip_nonint(&mut self) {
        self.parser.parse(eat_non_numeric_signed);
    }

    pub fn skip_nonint_unsigned(&mut self) {
        self.parser.parse(eat_non_numeric_unsigned);
    }

    pub fn skip_spaces(&mut self) {
        self.parser.parse(eat_spaces);
    }

    pub fn skip_rest_of_line(&mut self) {
        self.parser.parse(eat_rest_of_line);
    }
}

pub fn expect_byte(pred: u8) -> impl Fn(&[u8]) -> Option<((), usize)> {
    return move |data| {
        if let Some(_) = data.get(0).filter(|v2| **v2 == pred) {
            Some(((), 1))
        } else {
            None
        }
    }
}

pub fn eat_rest_of_line(data: &[u8]) -> Option<((), usize)> {
    if data.is_empty() {
        None
    } else {
        Some(((), data.iter().take_while(|v| **v != b'\n').count() + 1))
    }
}

pub fn eat_spaces(data: &[u8]) -> Option<((), usize)> {
    Some(((), data.iter().take_while(|v| match **v {
        b' ' | b'\n' | b'\r' | b'\t' => true,
        _ => false,
    }).count()))
}

pub fn eat_non_numeric_unsigned(data: &[u8]) -> Option<((), usize)> {
    Some(((), data.iter()
        .take_while(|v| **v < b'0' || **v > b'9')
        .count()))
}

pub fn eat_non_numeric_signed(data: &[u8]) -> Option<((), usize)> {
    Some(((), data.iter()
        .take_while(|v| **v != b'-' && (**v < b'0' || **v > b'9'))
        .count()))
}

pub fn parse_hex_byte(data: &[u8]) -> Option<(u8, usize)> {
    if let Some((a, _)) = parse_hex_number(data) {
        if let Some((b, _)) = parse_hex_number(&data[1..]) {
            return Some(((a << 4) + b, 2));
        }
    }

    None
}

pub fn parse_hex_number(data: &[u8]) -> Option<(u8, usize)> {
    if data.is_empty() {
        return None;
    }

    let hex = data[0];
    match hex {
        b'0'..=b'9' => Some((hex - b'0', 1)),
        b'A'..=b'F' => Some(((hex - b'A') + 10, 1)),
        _ => None,
    }
}

pub fn parse_int<T: Integer + Copy + From<u8> + Neg<Output=T>>(data: &[u8]) -> Option<(T, usize)> {
    if data.is_empty() {
        return None;
    }

    let mut sum = T::zero();
    let mut neg = false;
    let ten = T::from(10u8);

    for (i, b) in data.iter().enumerate() {
        match *b {
            b'0'..=b'9' => {
                sum = sum.mul(ten);
                sum = sum.add(T::from(*b - b'0'));
            }
            b'-' if !neg => {
                neg = true;
            }
            _ => {
                return if i > 0 {
                    if neg {
                        sum = sum.neg()
                    }
                    Some((sum, i))
                } else {
                    None
                };
            }
        }
    }

    if neg {
        sum = sum.neg()
    }
    Some((sum, data.len()))
}

pub fn parse_uint<T: Integer + Copy + From<u8>>(data: &[u8]) -> Option<(T, usize)> {
    if data.is_empty() {
        return None;
    }

    let mut sum = T::zero();
    let ten = T::from(10u8);

    for (i, b) in data.iter().enumerate() {
        match *b {
            b'0'..=b'9' => {
                sum = sum.mul(ten);
                sum = sum.add(T::from(*b - b'0'));
            }
            _ => {
                return if i > 0 {
                    Some((sum, i))
                } else {
                    None
                };
            }
        }
    }

    Some((sum, data.len()))
}

pub fn parse_line(data: &[u8]) -> Option<(&[u8], usize)> {
    let len = data.iter().take_while(|b| **b != b'\n').count();
    if len > 0 {
        Some((&data[..len], len + 1))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ints() {
        let mut parser = Parser::new_at(b"16,-32,48,123,443,12345678912345,75557863761098695507968", 0);
        parser.parse(eat_non_numeric_signed);
        assert_eq!(parser.parse(parse_int::<i64>), Some(16i64));
        parser.parse(eat_non_numeric_signed);
        assert_eq!(parser.parse(parse_int::<i32>), Some(-32i32));
        parser.parse(eat_non_numeric_signed);
        assert_eq!(parser.parse(parse_int::<i16>), Some(48i16));
        parser.parse(eat_non_numeric_unsigned);
        assert_eq!(parser.parse(parse_uint::<u16>), Some(123u16));
        parser.parse(eat_non_numeric_signed);
        assert_eq!(parser.parse(parse_int::<i128>), Some(443i128));
        parser.parse(eat_non_numeric_unsigned);
        assert_eq!(parser.parse(parse_uint::<u64>), Some(12345678912345u64));
        parser.parse(eat_non_numeric_unsigned);
        assert_eq!(parser.parse(parse_uint::<u128>), Some(75557863761098695507968u128));
    }

    #[test]
    fn test_parse_hex() {
        let mut parser = Parser::new_at(b"DEADC0FEE", 0);
        assert_eq!(parser.parse(parse_hex_byte), Some(0xDE));
        assert_eq!(parser.parse(parse_hex_byte), Some(0xAD));
        assert_eq!(parser.parse(parse_hex_byte), Some(0xC0));
        assert_eq!(parser.parse(parse_hex_byte), Some(0xFE));
        assert_eq!(parser.parse(parse_hex_byte), None);
        assert_eq!(parser.parse(parse_hex_number), Some(0xE));
    }

    #[test]
    fn test_parse_line() {
        let mut parser = Parser::new_at(b"123\ntest\nline\n\n", 0);
        assert_eq!(parser.parse_slice(parse_line), Some(b"123".as_slice()));
        assert_eq!(parser.parse_slice(parse_line), Some(b"test".as_slice()));
        assert_eq!(parser.parse_slice(parse_line), Some(b"line".as_slice()));
        assert_eq!(parser.parse_slice(parse_line), None);
        assert_eq!(parser.parse_slice(parse_line), None);
        assert_eq!(parser.parse_slice(parse_line), None);
    }
}