use std::fmt;
use serde::{Serialize, Deserialize};
use strum_macros::{Display, EnumString};
use num_derive::{FromPrimitive, ToPrimitive};

/// 메모리 타입 열거형
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString, Display, FromPrimitive, ToPrimitive)]
pub enum MemoryType {
    Byte,    // Int8
    UByte,   // UInt8
    Short,   // Int16
    UShort,  // UInt16
    Int,     // Int32
    UInt,    // UInt32
    Long,    // Int64
    ULong,   // UInt64
    Float,
    Double,
    Bool,
    Pointer,
    String,  // UTF8 = ASCII
    Bytes,   // ByteArray
}

impl MemoryType {
    /// 메모리 타입에 따른 크기(바이트) 반환
    pub fn size(&self) -> usize {
        match self {
            MemoryType::Byte | MemoryType::UByte | MemoryType::Bool => 1,
            MemoryType::Short | MemoryType::UShort => 2,
            MemoryType::Int | MemoryType::UInt | MemoryType::Float => 4,
            MemoryType::Long | MemoryType::ULong | MemoryType::Double | MemoryType::Pointer => 8,
            MemoryType::String | MemoryType::Bytes => 0, // 가변 크기
        }
    }

    /// 메모리 타입이 정수 타입인지 확인
    pub fn is_integer(&self) -> bool {
        matches!(
            self,
            MemoryType::Byte
                | MemoryType::UByte
                | MemoryType::Short
                | MemoryType::UShort
                | MemoryType::Int
                | MemoryType::UInt
                | MemoryType::Long
                | MemoryType::ULong
        )
    }

    /// 메모리 타입이 부동 소수점 타입인지 확인
    pub fn is_float(&self) -> bool {
        matches!(self, MemoryType::Float | MemoryType::Double)
    }

    /// 메모리 타입이 숫자 타입인지 확인
    pub fn is_numeric(&self) -> bool {
        self.is_integer() || self.is_float()
    }
}

/// 메모리 값을 나타내는 열거형
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryValue {
    Byte(i8),
    UByte(u8),
    Short(i16),
    UShort(u16),
    Int(i32),
    UInt(u32),
    Long(i64),
    ULong(u64),
    Float(f32),
    Double(f64),
    Bool(bool),
    Pointer(u64),
    String(String),
    Bytes(Vec<u8>),
}

impl MemoryValue {
    /// 메모리 값의 타입 반환
    pub fn get_type(&self) -> MemoryType {
        match self {
            MemoryValue::Byte(_) => MemoryType::Byte,
            MemoryValue::UByte(_) => MemoryType::UByte,
            MemoryValue::Short(_) => MemoryType::Short,
            MemoryValue::UShort(_) => MemoryType::UShort,
            MemoryValue::Int(_) => MemoryType::Int,
            MemoryValue::UInt(_) => MemoryType::UInt,
            MemoryValue::Long(_) => MemoryType::Long,
            MemoryValue::ULong(_) => MemoryType::ULong,
            MemoryValue::Float(_) => MemoryType::Float,
            MemoryValue::Double(_) => MemoryType::Double,
            MemoryValue::Bool(_) => MemoryType::Bool,
            MemoryValue::Pointer(_) => MemoryType::Pointer,
            MemoryValue::String(_) => MemoryType::String,
            MemoryValue::Bytes(_) => MemoryType::Bytes,
        }
    }

    /// 다른 타입으로 변환 시도
    pub fn try_convert(&self, target_type: MemoryType) -> Option<MemoryValue> {
        match (self, target_type) {
            // 정수 타입 간 변환
            (MemoryValue::Byte(v), MemoryType::UByte) => Some(MemoryValue::UByte(*v as u8)),
            (MemoryValue::Byte(v), MemoryType::Short) => Some(MemoryValue::Short(*v as i16)),
            (MemoryValue::Byte(v), MemoryType::UShort) => Some(MemoryValue::UShort(*v as u16)),
            (MemoryValue::Byte(v), MemoryType::Int) => Some(MemoryValue::Int(*v as i32)),
            (MemoryValue::Byte(v), MemoryType::UInt) => Some(MemoryValue::UInt(*v as u32)),
            (MemoryValue::Byte(v), MemoryType::Long) => Some(MemoryValue::Long(*v as i64)),
            (MemoryValue::Byte(v), MemoryType::ULong) => Some(MemoryValue::ULong(*v as u64)),
            
            // 정수 -> 부동소수점 변환
            (MemoryValue::Byte(v), MemoryType::Float) => Some(MemoryValue::Float(*v as f32)),
            (MemoryValue::Byte(v), MemoryType::Double) => Some(MemoryValue::Double(*v as f64)),
            
            // 다른 정수 타입의 변환도 유사하게 구현...
            // 축약을 위해 일부만 구현
            
            // 부동소수점 간 변환
            (MemoryValue::Float(v), MemoryType::Double) => Some(MemoryValue::Double(*v as f64)),
            (MemoryValue::Double(v), MemoryType::Float) => Some(MemoryValue::Float(*v as f32)),
            
            // 타입이 이미 같은 경우
            _ if self.get_type() == target_type => Some(self.clone()),
            
            // 그 외 변환 불가능한 경우
            _ => None,
        }
    }

    /// 문자열에서 MemoryValue로 파싱
    pub fn parse(value_str: &str, memory_type: MemoryType) -> Result<Self, String> {
        match memory_type {
            MemoryType::Byte => Ok(MemoryValue::Byte(value_str.parse().map_err(|e| format!("Failed to parse i8: {}", e))?)),
            MemoryType::UByte => Ok(MemoryValue::UByte(value_str.parse().map_err(|e| format!("Failed to parse u8: {}", e))?)),
            MemoryType::Short => Ok(MemoryValue::Short(value_str.parse().map_err(|e| format!("Failed to parse i16: {}", e))?)),
            MemoryType::UShort => Ok(MemoryValue::UShort(value_str.parse().map_err(|e| format!("Failed to parse u16: {}", e))?)),
            MemoryType::Int => Ok(MemoryValue::Int(value_str.parse().map_err(|e| format!("Failed to parse i32: {}", e))?)),
            MemoryType::UInt => Ok(MemoryValue::UInt(value_str.parse().map_err(|e| format!("Failed to parse u32: {}", e))?)),
            MemoryType::Long => Ok(MemoryValue::Long(value_str.parse().map_err(|e| format!("Failed to parse i64: {}", e))?)),
            MemoryType::ULong => Ok(MemoryValue::ULong(value_str.parse().map_err(|e| format!("Failed to parse u64: {}", e))?)),
            MemoryType::Float => Ok(MemoryValue::Float(value_str.parse().map_err(|e| format!("Failed to parse f32: {}", e))?)),
            MemoryType::Double => Ok(MemoryValue::Double(value_str.parse().map_err(|e| format!("Failed to parse f64: {}", e))?)),
            MemoryType::Bool => {
                match value_str.to_lowercase().as_str() {
                    "true" | "1" => Ok(MemoryValue::Bool(true)),
                    "false" | "0" => Ok(MemoryValue::Bool(false)),
                    _ => Err(format!("Failed to parse bool from '{}'", value_str)),
                }
            },
            MemoryType::Pointer => {
                // 16진수로 시작하면 16진수로 파싱, 아니면 10진수로 파싱
                if value_str.starts_with("0x") || value_str.starts_with("0X") {
                    let hex_str = &value_str[2..];
                    Ok(MemoryValue::Pointer(u64::from_str_radix(hex_str, 16).map_err(|e| format!("Failed to parse hex pointer: {}", e))?))
                } else {
                    Ok(MemoryValue::Pointer(value_str.parse().map_err(|e| format!("Failed to parse pointer: {}", e))?))
                }
            },
            MemoryType::String => Ok(MemoryValue::String(value_str.to_string())),
            MemoryType::Bytes => {
                // 16진수 스트링을 바이트 배열로 변환
                if value_str.starts_with("0x") || value_str.starts_with("0X") {
                    let hex_str = &value_str[2..];
                    let mut bytes = Vec::new();
                    for i in 0..(hex_str.len() / 2) {
                        let byte_str = &hex_str[i*2..i*2+2];
                        let byte = u8::from_str_radix(byte_str, 16).map_err(|e| format!("Failed to parse hex byte: {}", e))?;
                        bytes.push(byte);
                    }
                    Ok(MemoryValue::Bytes(bytes))
                } else {
                    // 바이트 배열로 변환할 수 없는 경우
                    Err(format!("Cannot parse {} as bytes, use 0x prefix for hex", value_str))
                }
            },
        }
    }
}

impl fmt::Display for MemoryValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MemoryValue::Byte(v) => write!(f, "{}", v),
            MemoryValue::UByte(v) => write!(f, "{}", v),
            MemoryValue::Short(v) => write!(f, "{}", v),
            MemoryValue::UShort(v) => write!(f, "{}", v),
            MemoryValue::Int(v) => write!(f, "{}", v),
            MemoryValue::UInt(v) => write!(f, "{}", v),
            MemoryValue::Long(v) => write!(f, "{}", v),
            MemoryValue::ULong(v) => write!(f, "{}", v),
            MemoryValue::Float(v) => write!(f, "{}", v),
            MemoryValue::Double(v) => write!(f, "{}", v),
            MemoryValue::Bool(v) => write!(f, "{}", v),
            MemoryValue::Pointer(v) => write!(f, "0x{:x}", v),
            MemoryValue::String(v) => write!(f, "{:?}", v),
            MemoryValue::Bytes(v) => {
                if v.len() > 16 {
                    write!(f, "0x{} ... ({} bytes)", 
                        v.iter().take(8).map(|b| format!("{:02x}", b)).collect::<Vec<String>>().join(""),
                        v.len()
                    )
                } else {
                    write!(f, "0x{}", 
                        v.iter().map(|b| format!("{:02x}", b)).collect::<Vec<String>>().join("")
                    )
                }
            }
        }
    }
} 