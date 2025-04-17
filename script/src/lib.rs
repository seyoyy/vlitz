pub mod parser;
pub mod executor;

pub use parser::*;
pub use executor::*;

// VLITZ 스크립트 파서 및 실행기 모듈
// 이 모듈은 .vzs 스크립트 파일을 파싱하고 실행하는 기능을 제공합니다.

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
} 