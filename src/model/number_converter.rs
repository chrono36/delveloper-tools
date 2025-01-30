use base64::Engine as _;
use std::{fmt, num::ParseIntError, slice::Iter};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NumberBaseConverter {
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
    Base64,
}

impl NumberBaseConverter {
    pub fn iter() -> Iter<'static, NumberBaseConverter> {
        static BASELIST: [NumberBaseConverter; 5] = [
            NumberBaseConverter::Binary,
            NumberBaseConverter::Decimal,
            NumberBaseConverter::Hexadecimal,
            NumberBaseConverter::Octal,
            NumberBaseConverter::Base64,
        ];
        BASELIST.iter()
    }

    pub fn to_decimal(&self, src: &str) -> Result<i64, ParseIntError> {
        match self {
            NumberBaseConverter::Binary => i64::from_str_radix(src, 2),
            NumberBaseConverter::Octal => i64::from_str_radix(src, 8),
            NumberBaseConverter::Decimal => src.trim().parse::<i64>(),
            NumberBaseConverter::Hexadecimal => i64::from_str_radix(src, 16),
            NumberBaseConverter::Base64 => {
                match base64::engine::general_purpose::STANDARD.decode(src.trim()) {
                    Ok(bytes) => {
                        if bytes.len() <= 8 {
                            let mut value: i64 = 0;
                            for byte in bytes {
                                value = (value << 8) | (byte as i64);
                            }
                            Ok(value)
                        } else {
                            Err(i64::from_str_radix("invalid", 10).unwrap_err())
                        }
                    }
                    Err(_) => Err(i64::from_str_radix("invalid", 10).unwrap_err()),
                }
            }
        }
    }

    pub fn convert(&self, num: i64) -> Result<String, &'static str> {
        match self {
            NumberBaseConverter::Binary => Ok(format!("{:b}", num)),
            NumberBaseConverter::Octal => Ok(format!("{:o}", num)),
            NumberBaseConverter::Decimal => Ok(num.to_string()),
            NumberBaseConverter::Hexadecimal => Ok(format!("{:x}", num)),
            NumberBaseConverter::Base64 => {
                // 移除前导零以避免不必要的填充
                let bytes: Vec<u8> = num
                    .to_be_bytes()
                    .into_iter()
                    .skip_while(|&x| x == 0)
                    .collect();
                let bytes = if bytes.is_empty() { vec![0] } else { bytes };
                Ok(base64::engine::general_purpose::STANDARD.encode(bytes))
            }
        }
    }
}

impl fmt::Display for NumberBaseConverter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NumberBaseConverter::Binary => write!(f, "{:?}(2)", self),
            NumberBaseConverter::Octal => write!(f, "{:?}(8)", self),
            NumberBaseConverter::Decimal => write!(f, "{:?}(10)", self),
            NumberBaseConverter::Hexadecimal => write!(f, "{:?}(16)", self),
            NumberBaseConverter::Base64 => write!(f, "{:?}(64)", self),
        }

        // write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_decimal() {
        let converter = NumberBaseConverter::Decimal;
        println!("{:?}", converter.to_decimal("36"));
    }
}
