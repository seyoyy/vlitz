use thiserror::Error;

#[derive(Error, Debug)]
pub enum VlitzError {
    #[error("일반 오류: {0}")]
    General(String),

    #[error("명령어 파싱 오류: {0}")]
    CommandParse(String),

    #[error("필터 표현식 오류: {0}")]
    FilterExpr(String),

    #[error("메모리 접근 오류: {0}")]
    MemoryAccess(String),

    #[error("Frida 오류: {0}")]
    Frida(String),

    #[error("스크립트 실행 오류: {0}")]
    ScriptExec(String),

    #[error("셀렉터 오류: {0}")]
    Selector(String),

    #[error("타입 변환 오류: {0}")]
    TypeConversion(String),

    #[error("I/O 오류: {0}")]
    Io(#[from] std::io::Error),
}

pub type VlitzResult<T> = Result<T, VlitzError>; 