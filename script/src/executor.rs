use crate::parser::ScriptParser;

/// 스크립트 실행 결과
#[derive(Debug)]
pub enum ExecutionResult {
    Success,
    Error(String),
}

/// 스크립트 실행자 구조체
pub struct ScriptExecutor {
    parser: ScriptParser,
}

impl ScriptExecutor {
    /// 새 실행자 인스턴스 생성
    pub fn new() -> Self {
        ScriptExecutor {
            parser: ScriptParser::new(),
        }
    }
    
    /// 스크립트 실행
    pub fn execute(&self, script_content: &str) -> ExecutionResult {
        // 스크립트 파싱
        let commands = match self.parser.parse(script_content) {
            Ok(cmds) => cmds,
            Err(err) => return ExecutionResult::Error(format!("파싱 오류: {}", err)),
        };
        
        // 명령어 실행 (현재는 로깅만 함)
        for cmd in commands {
            println!("실행: {}", cmd);
        }
        
        ExecutionResult::Success
    }
} 