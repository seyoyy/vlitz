use std::str::FromStr;
use crate::error::{VlitzError, VlitzResult};
use crate::VzData;

/// 셀렉터 구조체
#[derive(Debug, Clone)]
pub enum Selector {
    /// 단일 인덱스
    Single(usize),
    /// 여러 인덱스 (6,3,5)
    Multiple(Vec<usize>),
    /// 범위 선택 (3-6)
    Range(usize, usize),
    /// 전체 선택
    All,
    /// 라이브러리 선택 (lib:5)
    Lib(Box<Selector>),
    /// 로그 선택 (log:5)
    Log(Box<Selector>),
}

impl Selector {
    /// 셀렉터에 해당하는 인덱스 목록 반환
    pub fn get_indices(&self, lib_len: usize, log_len: usize) -> Vec<usize> {
        match self {
            Selector::Single(idx) => vec![*idx],
            Selector::Multiple(indices) => indices.clone(),
            Selector::Range(start, end) => {
                if start <= end {
                    (start.clone()..=end.clone()).collect()
                } else {
                    (end.clone()..=start.clone()).rev().collect()
                }
            },
            Selector::All => {
                // 일반적으로 lib + log 순서대로 반환
                let mut indices = Vec::with_capacity(lib_len + log_len);
                for i in 0..lib_len {
                    indices.push(i);
                }
                for i in 0..log_len {
                    indices.push(i);
                }
                indices
            },
            Selector::Lib(inner) => {
                let inner_indices = inner.get_indices(lib_len, log_len);
                inner_indices.into_iter()
                    .filter(|&idx| idx < lib_len)
                    .collect()
            },
            Selector::Log(inner) => {
                let inner_indices = inner.get_indices(lib_len, log_len);
                inner_indices.into_iter()
                    .filter(|&idx| idx < log_len)
                    .collect()
            },
        }
    }
}

impl FromStr for Selector {
    type Err = VlitzError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 문자열이 비어있으면 오류
        if s.is_empty() {
            return Err(VlitzError::Selector("Empty selector".to_string()));
        }

        // 라이브러리 또는 로그 접두사 확인
        if s.starts_with("lib:") {
            let inner = &s[4..];
            let inner_selector = Selector::from_str(inner)?;
            return Ok(Selector::Lib(Box::new(inner_selector)));
        }

        if s.starts_with("log:") {
            let inner = &s[4..];
            let inner_selector = Selector::from_str(inner)?;
            return Ok(Selector::Log(Box::new(inner_selector)));
        }

        // "all" 키워드인 경우
        if s.to_lowercase() == "all" {
            return Ok(Selector::All);
        }

        // 쉼표로 구분된 복수 선택자인 경우
        if s.contains(',') {
            let parts: Vec<&str> = s.split(',').collect();
            let mut indices = Vec::with_capacity(parts.len());

            for part in parts {
                match Selector::from_str(part)? {
                    Selector::Single(idx) => indices.push(idx),
                    Selector::Range(start, end) => {
                        for i in start..=end {
                            indices.push(i);
                        }
                    },
                    Selector::Multiple(mut sub_indices) => {
                        indices.append(&mut sub_indices);
                    },
                    _ => return Err(VlitzError::Selector(format!("Invalid selector part: {}", part))),
                }
            }

            return Ok(Selector::Multiple(indices));
        }

        // 범위 선택자인 경우 (3-6)
        if s.contains('-') {
            let parts: Vec<&str> = s.split('-').collect();
            if parts.len() != 2 {
                return Err(VlitzError::Selector(format!("Invalid range selector: {}", s)));
            }

            let start = parts[0].parse::<usize>()
                .map_err(|_| VlitzError::Selector(format!("Invalid range start: {}", parts[0])))?;
            let end = parts[1].parse::<usize>()
                .map_err(|_| VlitzError::Selector(format!("Invalid range end: {}", parts[1])))?;

            return Ok(Selector::Range(start, end));
        }

        // 단일 숫자 선택자인 경우
        match s.parse::<usize>() {
            Ok(idx) => Ok(Selector::Single(idx)),
            Err(_) => Err(VlitzError::Selector(format!("Invalid selector: {}", s))),
        }
    }
}

/// 셀렉터를 사용하여 VzData 배열에서 데이터 추출
pub fn select_data<'a>(selector: &Selector, lib: &'a [VzData], log: &'a [VzData]) -> VlitzResult<Vec<&'a VzData>> {
    let lib_len = lib.len();
    let log_len = log.len();
    
    let indices = selector.get_indices(lib_len, log_len);
    let mut selected_data = Vec::with_capacity(indices.len());
    
    for idx in indices {
        if let Some(data) = lib.get(idx) {
            selected_data.push(data);
        } else if let Some(data) = log.get(idx) {
            selected_data.push(data);
        }
    }
    
    if selected_data.is_empty() {
        return Err(VlitzError::Selector("No data found for selector".to_string()));
    }
    
    Ok(selected_data)
}

/// 콘솔 출력에서 사용할 formatted 문자열 생성
pub fn format_vzdata(index: usize, data: &VzData) -> String {
    let type_str = format!("[{}]", data.data_type);
    let display_name = data.get_display_name();
    
    let mut result = format!("[{}] {} {}", index, type_str, display_name);
    
    // 라벨이 있는 경우 추가
    if let Some(label) = &data.label {
        result.push_str(&format!(" ({})", label));
    }
    
    // 태그가 있는 경우 추가
    if !data.tags.is_empty() {
        let tags_str = data.tags.iter()
            .map(|t| t.as_str())
            .collect::<Vec<&str>>()
            .join(", ");
        result.push_str(&format!(" [{}]", tags_str));
    }
    
    result
}

/// 주소를 16진수 문자열로 변환
pub fn addr_to_hex(addr: u64) -> String {
    format!("0x{:x}", addr)
}

/// 16진수 문자열을 주소로 변환
pub fn hex_to_addr(hex: &str) -> VlitzResult<u64> {
    if hex.starts_with("0x") || hex.starts_with("0X") {
        let hex_str = &hex[2..];
        u64::from_str_radix(hex_str, 16)
            .map_err(|_| VlitzError::TypeConversion(format!("Invalid hex address: {}", hex)))
    } else {
        hex.parse::<u64>()
            .map_err(|_| VlitzError::TypeConversion(format!("Invalid address: {}", hex)))
    }
}

/// 크기를 사람이 읽기 쉬운 형태로 변환 (KB, MB 등)
pub fn format_size(size: usize) -> String {
    const KB: usize = 1024;
    const MB: usize = KB * 1024;
    const GB: usize = MB * 1024;
    
    if size >= GB {
        format!("{:.1}GB", size as f64 / GB as f64)
    } else if size >= MB {
        format!("{:.1}MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.1}KB", size as f64 / KB as f64)
    } else {
        format!("{}B", size)
    }
} 