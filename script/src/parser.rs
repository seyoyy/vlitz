/// 스크립트 파일을 파싱하는 함수
pub fn parse_script(content: &str) -> Result<Vec<String>, String> {
    // 기본 구현 - 줄 단위로 명령어 반환
    let commands = content
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| line.to_string())
        .collect();
    
    Ok(commands)
}

/// 스크립트 파서 구조체
pub struct ScriptParser;

impl ScriptParser {
    /// 새 파서 인스턴스 생성
    pub fn new() -> Self {
        ScriptParser
    }
    
    /// 스크립트 내용 파싱
    pub fn parse(&self, content: &str) -> Result<Vec<String>, String> {
        parse_script(content)
    }
} 