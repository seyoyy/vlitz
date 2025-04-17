use vlitz_shared::{VzData, VlitzError, VlitzResult, VzDataContent, VzDataType};
use vlitz_shared::utils::hex_to_addr;
use vlitz_shared::memory::MemoryType;

/// 내비게이터 구조체 (선택된 VzData 관리)
pub struct Navigator {
    /// 현재 선택된 데이터
    selected: Option<VzData>,
}

impl Navigator {
    /// 새 내비게이터 생성
    pub fn new() -> Self {
        Self {
            selected: None,
        }
    }

    /// 데이터 선택
    pub fn select(&mut self, data: VzData) {
        self.selected = Some(data);
    }

    /// 선택 해제
    pub fn unselect(&mut self) {
        self.selected = None;
    }

    /// 주소 오프셋 추가
    pub fn add_offset(&mut self, offset: u64) -> VlitzResult<()> {
        if let Some(data) = &self.selected {
            if let Some(address) = data.get_address() {
                let new_address = address.checked_add(offset)
                    .ok_or_else(|| VlitzError::General("Address overflow".to_string()))?;
                
                // 주소 포인터로 변환
                let pointer = VzData::new_pointer(new_address, MemoryType::UInt, 4);
                self.selected = Some(pointer);
                return Ok(());
            }
            
            return Err(VlitzError::General("Selected data has no address field".to_string()));
        }
        
        Err(VlitzError::General("No data selected".to_string()))
    }

    /// 주소 오프셋 빼기
    pub fn sub_offset(&mut self, offset: u64) -> VlitzResult<()> {
        if let Some(data) = &self.selected {
            if let Some(address) = data.get_address() {
                let new_address = address.checked_sub(offset)
                    .ok_or_else(|| VlitzError::General("Address underflow".to_string()))?;
                
                // 주소 포인터로 변환
                let pointer = VzData::new_pointer(new_address, MemoryType::UInt, 4);
                self.selected = Some(pointer);
                return Ok(());
            }
            
            return Err(VlitzError::General("Selected data has no address field".to_string()));
        }
        
        Err(VlitzError::General("No data selected".to_string()))
    }

    /// 지정된 주소로 이동
    pub fn goto(&mut self, address_or_selector: &str) -> VlitzResult<()> {
        // 셀렉터가 오면 store에서 VzData를 가져온 후 그 주소로 이동해야 함
        // 이 함수에서는 간단하게 주소 문자열만 처리
        
        let address = hex_to_addr(address_or_selector)?;
        let pointer = VzData::new_pointer(address, MemoryType::UInt, 4);
        self.selected = Some(pointer);
        Ok(())
    }

    /// 선택된 데이터 가져오기
    pub fn get_selected(&self) -> Option<&VzData> {
        self.selected.as_ref()
    }

    /// 프롬프트 문자열 생성
    pub fn get_prompt(&self) -> String {
        if let Some(data) = &self.selected {
            match &data.content {
                VzDataContent::Pointer(p) => format!("vlitz:Pointer:0x{:x}>", p.address),
                VzDataContent::Function(f) => format!("vlitz:Function:{}>", f.name),
                VzDataContent::Method(m) => format!("vlitz:Method:{}::{}>", m.class_name, m.name),
                VzDataContent::Class(c) => format!("vlitz:Class:{}>", c.name),
                VzDataContent::Module(m) => format!("vlitz:Module:{}>", m.name),
                VzDataContent::Range(r) => format!("vlitz:Range:0x{:x}>", r.address),
                VzDataContent::Variable(v) => format!("vlitz:Variable:{}>", v.name),
            }
        } else {
            "vlitz>".to_string()
        }
    }
} 