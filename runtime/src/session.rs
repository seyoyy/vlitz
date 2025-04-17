use std::collections::HashMap;
use frida_rust::{Session, Script, ScriptOption};
use vlitz_shared::{VlitzError, VlitzResult};

/// VLITZ 세션 관리 구조체
pub struct VlitzSession {
    /// Frida 세션
    session: Session,
    /// 현재 로드된 스크립트 맵 (이름 -> 스크립트)
    scripts: HashMap<String, Script>,
}

impl VlitzSession {
    /// 새 VLITZ 세션 생성
    pub fn new(session: Session) -> Self {
        Self {
            session,
            scripts: HashMap::new(),
        }
    }

    /// 스크립트 생성 및 로드
    pub fn create_script(&mut self, name: &str, source: &str) -> VlitzResult<&Script> {
        if self.scripts.contains_key(name) {
            return Err(VlitzError::ScriptExec(format!("Script '{}' already exists", name)));
        }

        let script = self.session.create_script(source, ScriptOption::new())
            .map_err(|e| VlitzError::ScriptExec(format!("Failed to create script: {}", e)))?;

        script.load()
            .map_err(|e| VlitzError::ScriptExec(format!("Failed to load script: {}", e)))?;

        self.scripts.insert(name.to_string(), script);
        
        Ok(self.scripts.get(name).unwrap())
    }

    /// 스크립트 언로드
    pub fn unload_script(&mut self, name: &str) -> VlitzResult<()> {
        if let Some(script) = self.scripts.remove(name) {
            script.unload()
                .map_err(|e| VlitzError::ScriptExec(format!("Failed to unload script: {}", e)))?;
            Ok(())
        } else {
            Err(VlitzError::ScriptExec(format!("Script '{}' not found", name)))
        }
    }

    /// 스크립트 가져오기
    pub fn get_script(&self, name: &str) -> Option<&Script> {
        self.scripts.get(name)
    }

    /// 스크립트 목록 가져오기
    pub fn get_scripts(&self) -> Vec<&str> {
        self.scripts.keys().map(|k| k.as_str()).collect()
    }

    /// 스크립트에서 RPC 호출
    pub fn call_rpc<T: serde::de::DeserializeOwned>(&self, script_name: &str, export_name: &str, args: &[serde_json::Value]) -> VlitzResult<T> {
        if let Some(script) = self.scripts.get(script_name) {
            script.call(export_name, args)
                .map_err(|e| VlitzError::ScriptExec(format!("Failed to call RPC '{}': {}", export_name, e)))
        } else {
            Err(VlitzError::ScriptExec(format!("Script '{}' not found", script_name)))
        }
    }
    
    /// 세션 분리 (Detach)
    pub fn detach(&self) -> VlitzResult<()> {
        // 모든 스크립트 언로드 (실제 구현에서는 이 로직이 추가될 수 있음)
        
        self.session.detach()
            .map_err(|e| VlitzError::Frida(format!("Failed to detach session: {}", e)))
    }
} 