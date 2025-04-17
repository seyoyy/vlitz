use std::str::FromStr;
use regex::Regex;
use crate::vzdata::{VzData, VzDataType};
use crate::memory::MemoryType;
use crate::error::{VlitzError, VlitzResult};

/// 필터 명령에서 사용하는 연산자
#[derive(Debug, Clone, PartialEq)]
pub enum FilterOperator {
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    Contains, // : 연산자, 문자열 포함 여부
}

impl FromStr for FilterOperator {
    type Err = VlitzError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "=" => Ok(FilterOperator::Equal),
            "!=" => Ok(FilterOperator::NotEqual),
            "<" => Ok(FilterOperator::LessThan),
            ">" => Ok(FilterOperator::GreaterThan),
            "<=" => Ok(FilterOperator::LessThanOrEqual),
            ">=" => Ok(FilterOperator::GreaterThanOrEqual),
            ":" => Ok(FilterOperator::Contains),
            _ => Err(VlitzError::FilterExpr(format!("Unknown operator: {}", s))),
        }
    }
}

/// 필터 표현식 조건의 종류
#[derive(Debug, Clone)]
pub enum FilterCondition {
    // 필드 비교
    Field {
        field_name: String,
        operator: FilterOperator,
        value: String,
    },
    // 메모리 데이터 비교 (포인터 전용)
    MemoryData {
        memory_type: MemoryType,
        operator: FilterOperator,
        value: String,
    },
    // 논리 연산
    And(Box<FilterCondition>, Box<FilterCondition>),
    Or(Box<FilterCondition>, Box<FilterCondition>),
}

impl FilterCondition {
    /// VzData에 조건을 적용하여 필터링
    pub fn apply(&self, data: &VzData) -> bool {
        match self {
            FilterCondition::Field { field_name, operator, value } => {
                Self::check_field(data, field_name, operator, value)
            },
            FilterCondition::MemoryData { memory_type, operator, value } => {
                Self::check_memory_data(data, memory_type, operator, value)
            },
            FilterCondition::And(cond1, cond2) => {
                cond1.apply(data) && cond2.apply(data)
            },
            FilterCondition::Or(cond1, cond2) => {
                cond1.apply(data) || cond2.apply(data)
            },
        }
    }

    /// 필드 기반 필터링 검사
    fn check_field(data: &VzData, field_name: &str, operator: &FilterOperator, value: &str) -> bool {
        // 일반적인 필드 검사 규칙
        match field_name {
            "type" => {
                let type_str = format!("{:?}", data.data_type);
                Self::compare_string(&type_str, operator, value)
            },
            "label" => {
                if let Some(label) = &data.label {
                    Self::compare_string(label, operator, value)
                } else {
                    *operator == FilterOperator::NotEqual || (*operator == FilterOperator::Equal && value.is_empty())
                }
            },
            "tags" => {
                // 태그에 대한 필터링 (포함 여부 확인)
                match operator {
                    FilterOperator::Contains => data.tags.iter().any(|tag| tag.contains(value)),
                    FilterOperator::Equal => data.tags.contains(value),
                    FilterOperator::NotEqual => !data.tags.contains(value),
                    _ => false,
                }
            },
            "name" => {
                if let Some(name) = data.get_name() {
                    Self::compare_string(name, operator, value)
                } else {
                    false
                }
            },
            "address" => {
                if let Some(address) = data.get_address() {
                    // 16진수 값은 0x 접두사로 시작하면 16진수로 파싱
                    let value_num = if value.starts_with("0x") || value.starts_with("0X") {
                        u64::from_str_radix(&value[2..], 16).unwrap_or(0)
                    } else {
                        value.parse::<u64>().unwrap_or(0)
                    };
                    Self::compare_number(address, operator, value_num)
                } else {
                    false
                }
            },
            // 타입 별 특수 필드
            "class_name" => {
                if let Some(method) = data.as_method() {
                    Self::compare_string(&method.class_name, operator, value)
                } else {
                    false
                }
            },
            "size" => {
                // Module, Range, Pointer에 대한 size 필드
                let size = if let Some(module) = data.as_module() {
                    Some(module.size as u64)
                } else if let Some(range) = data.as_range() {
                    Some(range.size as u64)
                } else if let Some(pointer) = data.as_pointer() {
                    Some(pointer.size as u64)
                } else {
                    None
                };

                if let Some(size) = size {
                    let value_num = value.parse::<u64>().unwrap_or(0);
                    Self::compare_number(size, operator, value_num)
                } else {
                    false
                }
            },
            "protection" => {
                if let Some(range) = data.as_range() {
                    Self::compare_string(&range.protection, operator, value)
                } else {
                    false
                }
            },
            _ => false,
        }
    }

    /// 메모리 데이터 필터링 (포인터 전용)
    fn check_memory_data(data: &VzData, _memory_type: &MemoryType, _operator: &FilterOperator, _value: &str) -> bool {
        // 실제 구현에서는 Frida API를 사용하여 메모리의 값을 읽고 비교
        // 이 예제에서는 간단한 구현만 제공
        // (실제 구현은 runtime 모듈에서 메모리 접근 후 필터링)
        
        // 1. 해당 데이터가 포인터인지 확인
        if data.as_pointer().is_none() {
            return false;
        }

        // 2. 메모리에서 해당 타입의 값을 읽어옴 (이 예제에서는 생략)
        // 3. 읽은 값을 주어진 값과 비교
        
        // 단순화 버전: 항상 false 반환 (실제 구현에서는 메모리 값 비교)
        false
    }

    /// 문자열 비교
    fn compare_string(a: &str, operator: &FilterOperator, b: &str) -> bool {
        match operator {
            FilterOperator::Equal => a == b,
            FilterOperator::NotEqual => a != b,
            FilterOperator::Contains => a.contains(b),
            FilterOperator::LessThan => a < b,
            FilterOperator::GreaterThan => a > b,
            FilterOperator::LessThanOrEqual => a <= b,
            FilterOperator::GreaterThanOrEqual => a >= b,
        }
    }

    /// 숫자 비교
    fn compare_number<T: PartialEq + PartialOrd>(a: T, operator: &FilterOperator, b: T) -> bool {
        match operator {
            FilterOperator::Equal => a == b,
            FilterOperator::NotEqual => a != b,
            FilterOperator::LessThan => a < b,
            FilterOperator::GreaterThan => a > b,
            FilterOperator::LessThanOrEqual => a <= b,
            FilterOperator::GreaterThanOrEqual => a >= b,
            FilterOperator::Contains => false, // 숫자에는 Contains 연산자 적용 불가
        }
    }
}

/// 필터 표현식 파서
pub struct FilterParser;

impl FilterParser {
    /// 문자열에서 필터 조건 파싱
    pub fn parse(expr: &str) -> VlitzResult<FilterCondition> {
        // 가장 바깥쪽 AND 연산자 (최소 우선순위)
        if let Some(pos) = Self::find_operator(expr, "&") {
            let left = &expr[0..pos].trim();
            let right = &expr[pos+1..].trim();
            return Ok(FilterCondition::And(
                Box::new(Self::parse(left)?),
                Box::new(Self::parse(right)?),
            ));
        }

        // 다음으로 OR 연산자
        if let Some(pos) = Self::find_operator(expr, "|") {
            let left = &expr[0..pos].trim();
            let right = &expr[pos+1..].trim();
            return Ok(FilterCondition::Or(
                Box::new(Self::parse(left)?),
                Box::new(Self::parse(right)?),
            ));
        }

        // 기본 조건 파싱
        // 1. 필드 조건 (field=value, field!=value, field<value, field>value, field:value)
        let field_regex = Regex::new(r"^([a-zA-Z_][a-zA-Z0-9_]*)\s*([=!<>:][=]?)\s*(.+)$").unwrap();
        if let Some(caps) = field_regex.captures(expr) {
            let field_name = caps.get(1).unwrap().as_str().to_string();
            let op_str = caps.get(2).unwrap().as_str();
            let value = caps.get(3).unwrap().as_str().trim();
            
            // 값에 따옴표가 있으면 제거
            let value = if (value.starts_with('"') && value.ends_with('"')) || 
                          (value.starts_with('\'') && value.ends_with('\'')) {
                &value[1..value.len()-1]
            } else {
                value
            };

            let operator = FilterOperator::from_str(op_str)?;
            
            return Ok(FilterCondition::Field {
                field_name,
                operator,
                value: value.to_string(),
            });
        }

        // 2. 메모리 데이터 조건 (float<30.5, uint=42, string:"hello")
        let memory_regex = Regex::new(r"^([a-zA-Z]+)\s*([=!<>:][=]?)\s*(.+)$").unwrap();
        if let Some(caps) = memory_regex.captures(expr) {
            let type_name = caps.get(1).unwrap().as_str();
            let op_str = caps.get(2).unwrap().as_str();
            let value = caps.get(3).unwrap().as_str().trim();
            
            // 값에 따옴표가 있으면 제거
            let value = if (value.starts_with('"') && value.ends_with('"')) || 
                          (value.starts_with('\'') && value.ends_with('\'')) {
                &value[1..value.len()-1]
            } else {
                value
            };

            // 메모리 타입 파싱
            let memory_type = match type_name.to_lowercase().as_str() {
                "byte" | "int8" => MemoryType::Byte,
                "ubyte" | "uint8" => MemoryType::UByte,
                "short" | "int16" => MemoryType::Short,
                "ushort" | "uint16" => MemoryType::UShort,
                "int" | "int32" => MemoryType::Int,
                "uint" | "uint32" => MemoryType::UInt,
                "long" | "int64" => MemoryType::Long,
                "ulong" | "uint64" => MemoryType::ULong,
                "float" => MemoryType::Float,
                "double" => MemoryType::Double,
                "bool" => MemoryType::Bool,
                "pointer" => MemoryType::Pointer,
                "string" | "utf8" | "ascii" => MemoryType::String,
                "bytes" | "bytearray" => MemoryType::Bytes,
                _ => return Err(VlitzError::FilterExpr(format!("Unknown memory type: {}", type_name))),
            };

            let operator = FilterOperator::from_str(op_str)?;
            
            return Ok(FilterCondition::MemoryData {
                memory_type,
                operator,
                value: value.to_string(),
            });
        }

        Err(VlitzError::FilterExpr(format!("Failed to parse filter expression: {}", expr)))
    }

    /// 연산자 위치 찾기 (괄호 내부는 무시)
    fn find_operator(expr: &str, op: &str) -> Option<usize> {
        let mut depth = 0;
        let mut in_quote = false;
        let mut escape_next = false;
        
        let chars: Vec<char> = expr.chars().collect();
        for i in 0..chars.len() {
            let c = chars[i];
            
            if escape_next {
                escape_next = false;
                continue;
            }
            
            if c == '\\' {
                escape_next = true;
                continue;
            }
            
            if c == '"' && !in_quote {
                in_quote = true;
                continue;
            }
            
            if c == '"' && in_quote {
                in_quote = false;
                continue;
            }
            
            if in_quote {
                continue;
            }
            
            if c == '(' {
                depth += 1;
                continue;
            }
            
            if c == ')' {
                depth -= 1;
                continue;
            }
            
            if depth == 0 && i + op.len() <= chars.len() {
                let sub = chars[i..i+op.len()].iter().collect::<String>();
                if sub == op {
                    return Some(i);
                }
            }
        }
        
        None
    }
}
