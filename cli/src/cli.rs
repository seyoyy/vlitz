use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "vlitz", about = "Frida CLI Debugger", version)]
pub struct Cli {
    /// 대상 프로세스 이름이나 PID
    #[clap(name = "TARGET")]
    pub target: Option<String>,

    /// USB 디바이스에 연결
    #[clap(short = 'U', long = "usb")]
    pub usb: bool,

    /// 원격 Frida 서버에 연결
    #[clap(short = 'R', long = "remote")]
    pub remote: bool,

    /// 연결할 Frida 서버 호스트
    #[clap(short = 'H', long = "host", requires = "remote")]
    pub host: Option<String>,

    /// 특정 디바이스 ID로 연결
    #[clap(short = 'D', long = "device")]
    pub device_id: Option<String>,

    /// 새 프로세스 실행 (스폰)
    #[clap(short = 'f', long = "file")]
    pub spawn_target: Option<String>,

    /// 이름으로 프로세스에 연결
    #[clap(short = 'n', long = "attach-name")]
    pub attach_name: Option<String>,

    /// ID로 프로세스에 연결
    #[clap(short = 'N', long = "attach-id")]
    pub attach_id: Option<String>,

    /// PID로 프로세스에 연결
    #[clap(short = 'p', long = "attach-pid")]
    pub attach_pid: Option<u32>,

    /// 로드할 VLITZ 스크립트
    #[clap(short = 'l', long = "load")]
    pub load_script: Option<String>,

    /// 서브 명령어
    #[clap(subcommand)]
    pub command: Option<CliCommand>,
}

#[derive(Subcommand)]
pub enum CliCommand {
    /// 프로세스 목록 보기
    Ps {
        /// 애플리케이션만 표시
        #[clap(short = 'a', long = "applications")]
        applications: bool,

        /// 설치된 앱만 표시
        #[clap(short = 'i', long = "installed")]
        installed: bool,
    },

    /// 기기 목록 보기
    Devices,

    /// 프로세스 종료
    Kill {
        /// 종료할 프로세스 이름 또는 PID
        target: String,
    },
}

impl Cli {
    /// 새 CLI 인스턴스 생성
    pub fn new() -> Self {
        Self::parse()
    }

    /// Frida 대상 프로세스 결정
    pub fn get_target(&self) -> Option<String> {
        if let Some(target) = &self.target {
            return Some(target.clone());
        }

        if let Some(name) = &self.attach_name {
            return Some(name.clone());
        }

        if let Some(id) = &self.attach_id {
            return Some(id.clone());
        }

        if let Some(pid) = self.attach_pid {
            return Some(pid.to_string());
        }

        if let Some(file) = &self.spawn_target {
            return Some(file.clone());
        }

        None
    }

    /// 명령행 인자 처리
    pub fn is_command(&self) -> bool {
        self.command.is_some()
    }
} 