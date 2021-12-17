use std::cmp::min;
use num::Integer;
use std::ops::Neg;

pub fn byte(data: &[u8]) -> Option<(u8, &[u8])> {
    if !data.is_empty() {
        Some((data[0], &data[1..]))
    } else {
        None
    }
}


pub fn expect_byte(data: &[u8], pred: u8) -> Option<((), &[u8])> {
    if let Some(_) = data.get(0).filter(|v2| **v2 == pred) {
        Some(((), &data[1..]))
    } else {
        None
    }
}

pub fn expect_bytes<'a, 'b>(data: &'a [u8], pred: &'b [u8]) -> Option<((), &'a [u8])> {
    if data.starts_with(pred) {
        Some(((), &data[pred.len()..]))
    } else {
        None
    }
}

pub fn expect_string<'a, 'b>(data: &'a [u8], pred: &'b str) -> Option<((), &'a [u8])> {
    if data.starts_with(pred.as_bytes()) {
        Some(((), &data[pred.len()..]))
    } else {
        None
    }
}

pub fn rest_of_line(data: &[u8]) -> Option<(&[u8], &[u8])> {
    if data.is_empty() {
        None
    } else {
        let len = data.iter().take_while(|v| **v != b'\n').count();
        Some((&data[..len], &data[len+1..]))
    }
}

pub fn non_numeric(data: &[u8], signed: bool) -> Option<(&[u8], &[u8])> {
    let len = if signed {
        data.iter()
            .take_while(|v| **v != b'-' && (**v < b'0' || **v > b'9'))
            .count()
    } else {
        data.iter()
            .take_while(|v| **v < b'0' || **v > b'9')
            .count()
    };

    if len > 0 {
        Some((&data[..len], &data[len..]))
    } else {
        Some((&data[..0], &data))
    }
}

pub fn hex_byte(data: &[u8]) -> Option<(u8, &[u8])> {
    if let Some((a, _)) = hex_number(data) {
        if let Some((b, _)) = hex_number(&data[1..]) {
            return Some(((a << 4) + b, &data[2..]));
        } else {
            return Some(((a << 4), &data[1..]));
        }
    }

    None
}

pub fn hex_number(data: &[u8]) -> Option<(u8, &[u8])> {
    if data.is_empty() {
        return None;
    }

    let hex = data[0];
    match hex {
        b'0'..=b'9' => Some((hex - b'0', &data[1..])),
        b'A'..=b'F' => Some(((hex - b'A') + 10, &data[1..])),
        _ => None,
    }
}

pub fn int<T: Integer + Copy + From<u8> + Neg<Output=T>>(data: &[u8]) -> Option<(T, &[u8])> {
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
                    Some((sum, &data[i..]))
                } else {
                    None
                };
            }
        }
    }

    if neg {
        sum = sum.neg()
    }
    Some((sum, &data[data.len()..]))
}

pub fn uint<T: Integer + Copy + From<u8>>(data: &[u8]) -> Option<(T, &[u8])> {
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
                    Some((sum, &data[i..]))
                } else {
                    None
                };
            }
        }
    }

    Some((sum, &data[data.len()..]))
}

pub fn line(data: &[u8]) -> Option<(&[u8], &[u8])> {
    let len = data.iter().take_while(|b| **b != b'\n').count();
    if len > 0 {
        Some((&data[..len], &data[min(len, data.len())..]))
    } else {
        None
    }
}
