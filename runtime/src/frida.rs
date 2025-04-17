use std::collections::HashMap;
use frida_rust::{DeviceManager, Device, DeviceType, Process, Session, Script};
use crate::session::VlitzSession;
use vlitz_shared::{VlitzError, VlitzResult};

lazy_static::lazy_static! {
    static ref DEVICE_MANAGER: DeviceManager = DeviceManager::new();
}

/// Frida 디바이스 관리 인터페이스
pub struct FridaManager;

impl FridaManager {
    /// 사용 가능한 디바이스 목록 반환
    pub fn get_devices() -> VlitzResult<Vec<Device>> {
        DEVICE_MANAGER.enumerate_devices()
            .map_err(|e| VlitzError::Frida(format!("Failed to enumerate devices: {}", e)))
    }

    /// ID로 디바이스 가져오기
    pub fn get_device_by_id(id: &str) -> VlitzResult<Device> {
        DEVICE_MANAGER.get_device(id)
            .map_err(|e| VlitzError::Frida(format!("Failed to get device: {}", e)))
    }

    /// USB 디바이스 가져오기
    pub fn get_usb_device() -> VlitzResult<Device> {
        DEVICE_MANAGER.get_usb_device()
            .map_err(|e| VlitzError::Frida(format!("Failed to get USB device: {}", e)))
    }

    /// 로컬 디바이스 가져오기
    pub fn get_local_device() -> VlitzResult<Device> {
        DEVICE_MANAGER.get_local_device()
            .map_err(|e| VlitzError::Frida(format!("Failed to get local device: {}", e)))
    }

    /// 원격 디바이스 가져오기
    pub fn get_remote_device(host: Option<&str>) -> VlitzResult<Device> {
        let addr = host.unwrap_or("127.0.0.1");
        DEVICE_MANAGER.add_remote_device(addr)
            .map_err(|e| VlitzError::Frida(format!("Failed to add remote device: {}", e)))
    }

    /// 디바이스 타입 문자열 변환
    pub fn device_type_to_string(device_type: DeviceType) -> &'static str {
        match device_type {
            DeviceType::Local => "Local",
            DeviceType::Remote => "Remote",
            DeviceType::Usb => "USB",
            _ => "Unknown",
        }
    }

    /// 프로세스 목록 가져오기
    pub fn get_processes(device: &Device) -> VlitzResult<Vec<Process>> {
        device.enumerate_processes()
            .map_err(|e| VlitzError::Frida(format!("Failed to enumerate processes: {}", e)))
    }

    /// 이름으로 프로세스 찾기
    pub fn find_process_by_name(device: &Device, name: &str) -> VlitzResult<Option<Process>> {
        let processes = Self::get_processes(device)?;
        Ok(processes.into_iter().find(|p| p.name().contains(name)))
    }

    /// PID로 프로세스 찾기
    pub fn get_process_by_pid(device: &Device, pid: u32) -> VlitzResult<Process> {
        device.get_process_by_pid(pid)
            .map_err(|e| VlitzError::Frida(format!("Failed to get process by PID: {}", e)))
    }

    /// 프로세스에 Attach
    pub fn attach(device: &Device, pid: u32) -> VlitzResult<VlitzSession> {
        let session = device.attach(pid)
            .map_err(|e| VlitzError::Frida(format!("Failed to attach to process: {}", e)))?;
        
        Ok(VlitzSession::new(session))
    }

    /// 프로세스 실행 (Spawn)
    pub fn spawn(device: &Device, program: &str, args: Option<Vec<&str>>) -> VlitzResult<u32> {
        let pid = match args {
            Some(args) => device.spawn(program, Some(&args))
                .map_err(|e| VlitzError::Frida(format!("Failed to spawn process: {}", e)))?,
            None => device.spawn(program, None::<Vec<&str>>)
                .map_err(|e| VlitzError::Frida(format!("Failed to spawn process: {}", e)))?,
        };
        
        Ok(pid)
    }

    /// 프로세스 실행 후 Attach
    pub fn spawn_and_attach(device: &Device, program: &str, args: Option<Vec<&str>>) -> VlitzResult<(u32, VlitzSession)> {
        let pid = Self::spawn(device, program, args)?;
        let session = Self::attach(device, pid)?;
        
        Ok((pid, session))
    }

    /// 프로세스 Resume
    pub fn resume(device: &Device, pid: u32) -> VlitzResult<()> {
        device.resume(pid)
            .map_err(|e| VlitzError::Frida(format!("Failed to resume process: {}", e)))
    }

    /// 프로세스 Kill
    pub fn kill(device: &Device, pid: u32) -> VlitzResult<()> {
        device.kill(pid)
            .map_err(|e| VlitzError::Frida(format!("Failed to kill process: {}", e)))
    }
} 