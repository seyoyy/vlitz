use anyhow::{Result, anyhow};
use vlitz_cli::{Cli, Repl};

fn main() -> Result<()> {
    // CLI 인자 파싱
    let cli = Cli::new();
    
    // 서브 명령어 처리 (ps, devices, kill 등)
    if cli.is_command() {
        // 나중에 구현
        println!("서브 명령어는 아직 구현되지 않았습니다.");
        return Ok(());
    }
    
    // 대상 프로세스 확인
    let _target = cli.get_target();
    
    // 기본 REPL 실행
    let mut repl = Repl::new(20)?;
    repl.run()?;
    
    Ok(())
} 