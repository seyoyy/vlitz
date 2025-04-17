use std::str::FromStr;
use vlitz_shared::{VlitzError, VlitzResult, Selector};

/// 명령어 인자 타입
#[derive(Debug, Clone)]
pub enum CommandArg {
    /// 문자열 인자
    String(String),
    /// 선택자 인자
    Selector(Selector),
    /// 필터 표현식 인자
    FilterExpr(String),
    /// 주소 인자
    Address(u64),
    /// 숫자 인자
    Number(i64),
    /// 부동 소수점 인자
    Float(f64),
}

impl CommandArg {
    /// 문자열로 변환
    pub fn as_string(&self) -> VlitzResult<&str> {
        match self {
            CommandArg::String(s) => Ok(s),
            _ => Err(VlitzError::TypeConversion("Expected String argument".to_string())),
        }
    }

    /// 선택자로 변환
    pub fn as_selector(&self) -> VlitzResult<&Selector> {
        match self {
            CommandArg::Selector(s) => Ok(s),
            _ => Err(VlitzError::TypeConversion("Expected Selector argument".to_string())),
        }
    }

    /// 필터 표현식으로 변환
    pub fn as_filter_expr(&self) -> VlitzResult<&str> {
        match self {
            CommandArg::FilterExpr(s) => Ok(s),
            _ => Err(VlitzError::TypeConversion("Expected Filter expression argument".to_string())),
        }
    }

    /// 주소로 변환
    pub fn as_address(&self) -> VlitzResult<u64> {
        match self {
            CommandArg::Address(addr) => Ok(*addr),
            CommandArg::Number(n) if *n >= 0 => Ok(*n as u64),
            _ => Err(VlitzError::TypeConversion("Expected Address argument".to_string())),
        }
    }

    /// 숫자로 변환
    pub fn as_number(&self) -> VlitzResult<i64> {
        match self {
            CommandArg::Number(n) => Ok(*n),
            CommandArg::Address(addr) => Ok(*addr as i64),
            _ => Err(VlitzError::TypeConversion("Expected Number argument".to_string())),
        }
    }

    /// 부동 소수점으로 변환
    pub fn as_float(&self) -> VlitzResult<f64> {
        match self {
            CommandArg::Float(f) => Ok(*f),
            CommandArg::Number(n) => Ok(*n as f64),
            _ => Err(VlitzError::TypeConversion("Expected Float argument".to_string())),
        }
    }
}

/// 명령어 구조체
#[derive(Debug, Clone)]
pub struct Command {
    /// 명령어 이름 (첫 번째 토큰)
    pub name: String,
    /// 하위 명령어 (두 번째 토큰)
    pub subcommand: Option<String>,
    /// 인자 목록
    pub args: Vec<CommandArg>,
}

impl Command {
    /// 문자열에서 명령어 파싱
    pub fn parse(input: &str) -> VlitzResult<Self> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Err(VlitzError::CommandParse("Empty command".to_string()));
        }

        let tokens = split_tokens(trimmed);
        if tokens.is_empty() {
            return Err(VlitzError::CommandParse("Empty command".to_string()));
        }

        let name = tokens[0].to_string();
        
        let subcommand = if tokens.len() > 1 {
            Some(tokens[1].to_string())
        } else {
            None
        };

        let args = if tokens.len() > 2 {
            parse_args(&tokens[2..])
        } else {
            Vec::new()
        };

        Ok(Command {
            name,
            subcommand,
            args,
        })
    }

    /// 인자가 있는지 확인
    pub fn has_args(&self) -> bool {
        !self.args.is_empty()
    }

    /// 인자 갯수 확인
    pub fn arg_count(&self) -> usize {
        self.args.len()
    }

    /// 인자 가져오기
    pub fn get_arg(&self, index: usize) -> Option<&CommandArg> {
        self.args.get(index)
    }
}

/// 토큰 분리
fn split_tokens(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut in_quotes = false;
    let mut escape_next = false;

    for c in input.chars() {
        if escape_next {
            current_token.push(c);
            escape_next = false;
            continue;
        }

        if c == '\\' {
            escape_next = true;
            continue;
        }

        if c == '"' {
            in_quotes = !in_quotes;
            continue;
        }

        if c.is_whitespace() && !in_quotes {
            if !current_token.is_empty() {
                tokens.push(current_token);
                current_token = String::new();
            }
            continue;
        }

        current_token.push(c);
    }

    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    tokens
}

/// 인자 파싱
fn parse_args(tokens: &[String]) -> Vec<CommandArg> {
    let mut args = Vec::new();

    for token in tokens {
        // 16진수 주소 인식
        if token.starts_with("0x") || token.starts_with("0X") {
            if let Ok(addr) = u64::from_str_radix(&token[2..], 16) {
                args.push(CommandArg::Address(addr));
                continue;
            }
        }

        // 10진수 정수 인식
        if let Ok(num) = token.parse::<i64>() {
            args.push(CommandArg::Number(num));
            continue;
        }

        // 부동 소수점 인식
        if let Ok(float) = token.parse::<f64>() {
            args.push(CommandArg::Float(float));
            continue;
        }

        // 선택자 인식
        if let Ok(selector) = Selector::from_str(token) {
            args.push(CommandArg::Selector(selector));
            continue;
        }

        // 필터 표현식 인식 (간단한 구현, 실제로는 더 복잡할 수 있음)
        if token.contains('=') || token.contains('<') || token.contains('>') || token.contains(':') {
            args.push(CommandArg::FilterExpr(token.clone()));
            continue;
        }

        // 그 외는 문자열로 처리
        args.push(CommandArg::String(token.clone()));
    }

    args
}

/// 명령어 목록 열거형
#[derive(Debug, Clone, PartialEq)]
pub enum CommandType {
    // Navigator 명령어
    NavSelect,
    NavUnselect,
    NavAdd,
    NavSub,
    NavGoto,
    
    // Log 명령어
    LogList,
    LogNext,
    LogPrev,
    LogSort,
    
    // Library 명령어
    LibList,
    LibSave,
    LibMove,
    LibRemove,
    LibClear,
    
    // Meta 명령어
    MetaLabel,
    MetaTag,
    MetaUntag,
    MetaTags,
    
    // List 명령어
    ListClass,
    ListMethod,
    ListModule,
    ListExports,
    ListRange,
    
    // Memory 명령어
    MemDump,
    MemRead,
    MemWrite,
    MemList,
    MemWatch,
    MemLock,
    MemTrace,
    MemUnwatch,
    MemUnlock,
    MemUntrace,
    MemType,
    MemDisas,
    
    // Attacher 명령어
    AttachHook,
    AttachUnhook,
    AttachCall,
    AttachList,
    
    // Scanner 명령어
    ScanSearch,
    ScanExact,
    ScanMin,
    ScanMax,
    ScanInc,
    ScanDec,
    ScanCh,
    ScanUnch,
    
    // Utilities
    Fields,
    Help,
    Run,
    
    // 기타
    Unknown,
}

impl Command {
    /// 명령어 타입 해석
    pub fn get_type(&self) -> CommandType {
        match self.name.as_str() {
            "nav" | "navigator" => {
                match self.subcommand.as_deref() {
                    Some("select") | Some("sel") => CommandType::NavSelect,
                    Some("unselect") | Some("unsel") => CommandType::NavUnselect,
                    Some("add") | Some("+") => CommandType::NavAdd,
                    Some("sub") | Some("-") => CommandType::NavSub,
                    Some("goto") | Some(":") => CommandType::NavGoto,
                    _ => CommandType::Unknown,
                }
            },
            "sel" => CommandType::NavSelect,
            "unsel" => CommandType::NavUnselect,
            "+" => CommandType::NavAdd,
            "-" => CommandType::NavSub,
            ":" => CommandType::NavGoto,
            
            "log" => {
                match self.subcommand.as_deref() {
                    Some("list") | Some("lg") => CommandType::LogList,
                    Some("next") | Some("nxt") => CommandType::LogNext,
                    Some("prev") | Some("prv") => CommandType::LogPrev,
                    Some("sort") => CommandType::LogSort,
                    _ => CommandType::Unknown,
                }
            },
            "lg" => CommandType::LogList,
            "nxt" => CommandType::LogNext,
            "prv" => CommandType::LogPrev,
            
            "lib" => {
                match self.subcommand.as_deref() {
                    Some("list") | Some("ls") => CommandType::LibList,
                    Some("save") | Some("sav") => CommandType::LibSave,
                    Some("move") | Some("mv") => CommandType::LibMove,
                    Some("remove") | Some("rm") => CommandType::LibRemove,
                    Some("clear") | Some("clr") => CommandType::LibClear,
                    _ => CommandType::Unknown,
                }
            },
            "ls" => CommandType::LibList,
            "sav" => CommandType::LibSave,
            "mv" => CommandType::LibMove,
            "rm" => CommandType::LibRemove,
            "clr" => CommandType::LibClear,
            
            "meta" => {
                match self.subcommand.as_deref() {
                    Some("label") => CommandType::MetaLabel,
                    Some("tag") => CommandType::MetaTag,
                    Some("untag") => CommandType::MetaUntag,
                    Some("tags") => CommandType::MetaTags,
                    _ => CommandType::Unknown,
                }
            },
            
            "list" => {
                match self.subcommand.as_deref() {
                    Some("class") => CommandType::ListClass,
                    Some("method") => CommandType::ListMethod,
                    Some("module") => CommandType::ListModule,
                    Some("exports") => CommandType::ListExports,
                    Some("range") => CommandType::ListRange,
                    _ => CommandType::Unknown,
                }
            },
            "class" => CommandType::ListClass,
            "method" => CommandType::ListMethod,
            "module" => CommandType::ListModule,
            "exports" => CommandType::ListExports,
            "range" => CommandType::ListRange,
            
            "mem" => {
                match self.subcommand.as_deref() {
                    Some("dump") | Some("d") => CommandType::MemDump,
                    Some("read") | Some("r") => CommandType::MemRead,
                    Some("write") | Some("w") => CommandType::MemWrite,
                    Some("list") | Some("lm") => CommandType::MemList,
                    Some("watch") => CommandType::MemWatch,
                    Some("lock") => CommandType::MemLock,
                    Some("trace") => CommandType::MemTrace,
                    Some("unwatch") => CommandType::MemUnwatch,
                    Some("unlock") => CommandType::MemUnlock,
                    Some("untrace") => CommandType::MemUntrace,
                    Some("type") => CommandType::MemType,
                    Some("disas") => CommandType::MemDisas,
                    _ => CommandType::Unknown,
                }
            },
            "d" => CommandType::MemDump,
            "r" => CommandType::MemRead,
            "w" => CommandType::MemWrite,
            "lm" => CommandType::MemList,
            
            "attach" => {
                match self.subcommand.as_deref() {
                    Some("hook") => CommandType::AttachHook,
                    Some("unhook") => CommandType::AttachUnhook,
                    Some("call") => CommandType::AttachCall,
                    Some("list") | Some("la") => CommandType::AttachList,
                    _ => CommandType::Unknown,
                }
            },
            "la" => CommandType::AttachList,
            
            "scan" => {
                match self.subcommand.as_deref() {
                    Some("search") => CommandType::ScanSearch,
                    Some("exact") => CommandType::ScanExact,
                    Some("min") => CommandType::ScanMin,
                    Some("max") => CommandType::ScanMax,
                    Some("inc") => CommandType::ScanInc,
                    Some("dec") => CommandType::ScanDec,
                    Some("ch") => CommandType::ScanCh,
                    Some("unch") => CommandType::ScanUnch,
                    _ => CommandType::Unknown,
                }
            },
            "search" => CommandType::ScanSearch,
            "exact" => CommandType::ScanExact,
            "min" => CommandType::ScanMin,
            "max" => CommandType::ScanMax,
            "inc" => CommandType::ScanInc,
            "dec" => CommandType::ScanDec,
            "ch" => CommandType::ScanCh,
            "unch" => CommandType::ScanUnch,
            
            "fields" => CommandType::Fields,
            "help" => CommandType::Help,
            "run" => CommandType::Run,
            
            _ => CommandType::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command() {
        let cmd = Command::parse("list class MainActivity").unwrap();
        assert_eq!(cmd.name, "list");
        assert_eq!(cmd.subcommand, Some("class".to_string()));
        assert_eq!(cmd.args.len(), 1);
        
        let cmd = Command::parse("sel 1").unwrap();
        assert_eq!(cmd.name, "sel");
        assert_eq!(cmd.subcommand, None);
        assert_eq!(cmd.args.len(), 1);
        
        let cmd = Command::parse("mem read 0x7ff8392000 float").unwrap();
        assert_eq!(cmd.name, "mem");
        assert_eq!(cmd.subcommand, Some("read".to_string()));
        assert_eq!(cmd.args.len(), 2);
    }

    #[test]
    fn test_parse_args() {
        let cmd = Command::parse("scan search 47.3 float").unwrap();
        assert_eq!(cmd.args.len(), 2);
        
        if let CommandArg::Float(val) = &cmd.args[0] {
            assert_eq!(*val, 47.3);
        } else {
            panic!("Expected Float argument");
        }
        
        if let CommandArg::String(val) = &cmd.args[1] {
            assert_eq!(val, "float");
        } else {
            panic!("Expected String argument");
        }
    }

    #[test]
    fn test_get_command_type() {
        let cmd = Command::parse("nav select 1").unwrap();
        assert_eq!(cmd.get_type(), CommandType::NavSelect);
        
        let cmd = Command::parse("sel 1").unwrap();
        assert_eq!(cmd.get_type(), CommandType::NavSelect);
        
        let cmd = Command::parse("search 47.3 float").unwrap();
        assert_eq!(cmd.get_type(), CommandType::ScanSearch);
    }
} 