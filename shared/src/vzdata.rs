use std::collections::HashSet;
use serde::{Serialize, Deserialize};
use strum_macros::{Display, EnumString};
use crate::memory::MemoryType;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, EnumString, Display)]
pub enum VzDataType {
    Pointer,
    Function,
    Method,
    Module,
    Class,
    Range,
    Variable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VzData {
    // 공통 필드
    pub label: Option<String>,
    pub tags: HashSet<String>,
    pub data_type: VzDataType,
    // 구체적인 데이터를 포함하는 필드
    pub content: VzDataContent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VzDataContent {
    Pointer(VzPointer),
    Function(VzFunction),
    Method(VzMethod),
    Module(VzModule),
    Class(VzClass),
    Range(VzRange),
    Variable(VzVariable),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VzPointer {
    pub address: u64,
    pub memory_type: MemoryType,
    pub size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VzFunction {
    pub name: String,
    pub address: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VzMethod {
    pub class_name: String,
    pub name: String,
    pub args: Vec<String>,
    pub ret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VzClass {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VzModule {
    pub name: String,
    pub address: u64,
    pub size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VzRange {
    pub address: u64,
    pub size: usize,
    pub protection: String,
    pub file: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VzVariable {
    pub name: String,
    pub address: u64,
}

impl VzData {
    // VzData 생성 유틸리티 함수들
    pub fn new_pointer(address: u64, memory_type: MemoryType, size: usize) -> Self {
        Self {
            label: None,
            tags: HashSet::new(),
            data_type: VzDataType::Pointer,
            content: VzDataContent::Pointer(VzPointer {
                address,
                memory_type,
                size,
            }),
        }
    }

    pub fn new_function(name: String, address: u64) -> Self {
        Self {
            label: None,
            tags: HashSet::new(),
            data_type: VzDataType::Function,
            content: VzDataContent::Function(VzFunction {
                name,
                address,
            }),
        }
    }

    pub fn new_method(class_name: String, name: String, args: Vec<String>, ret: String) -> Self {
        Self {
            label: None,
            tags: HashSet::new(),
            data_type: VzDataType::Method,
            content: VzDataContent::Method(VzMethod {
                class_name,
                name,
                args,
                ret,
            }),
        }
    }

    pub fn new_class(name: String) -> Self {
        Self {
            label: None,
            tags: HashSet::new(),
            data_type: VzDataType::Class,
            content: VzDataContent::Class(VzClass {
                name,
            }),
        }
    }

    pub fn new_module(name: String, address: u64, size: usize) -> Self {
        Self {
            label: None,
            tags: HashSet::new(),
            data_type: VzDataType::Module,
            content: VzDataContent::Module(VzModule {
                name,
                address,
                size,
            }),
        }
    }

    pub fn new_range(address: u64, size: usize, protection: String, file: Option<String>) -> Self {
        Self {
            label: None,
            tags: HashSet::new(),
            data_type: VzDataType::Range,
            content: VzDataContent::Range(VzRange {
                address,
                size,
                protection,
                file,
            }),
        }
    }

    pub fn new_variable(name: String, address: u64) -> Self {
        Self {
            label: None,
            tags: HashSet::new(),
            data_type: VzDataType::Variable,
            content: VzDataContent::Variable(VzVariable {
                name,
                address,
            }),
        }
    }

    // 유틸리티 메서드
    pub fn set_label(&mut self, label: String) {
        self.label = Some(label);
    }

    pub fn add_tag(&mut self, tag: String) {
        self.tags.insert(tag);
    }

    pub fn remove_tag(&mut self, tag: &str) -> bool {
        self.tags.remove(tag)
    }

    // 컨텐츠 접근 도우미 메서드들
    pub fn as_pointer(&self) -> Option<&VzPointer> {
        match &self.content {
            VzDataContent::Pointer(p) => Some(p),
            _ => None,
        }
    }

    pub fn as_function(&self) -> Option<&VzFunction> {
        match &self.content {
            VzDataContent::Function(f) => Some(f),
            _ => None,
        }
    }

    pub fn as_method(&self) -> Option<&VzMethod> {
        match &self.content {
            VzDataContent::Method(m) => Some(m),
            _ => None,
        }
    }

    pub fn as_class(&self) -> Option<&VzClass> {
        match &self.content {
            VzDataContent::Class(c) => Some(c),
            _ => None,
        }
    }

    pub fn as_module(&self) -> Option<&VzModule> {
        match &self.content {
            VzDataContent::Module(m) => Some(m),
            _ => None,
        }
    }

    pub fn as_range(&self) -> Option<&VzRange> {
        match &self.content {
            VzDataContent::Range(r) => Some(r),
            _ => None,
        }
    }

    pub fn as_variable(&self) -> Option<&VzVariable> {
        match &self.content {
            VzDataContent::Variable(v) => Some(v),
            _ => None,
        }
    }

    // 주소 필드를 가진 타입에서 주소 가져오기
    pub fn get_address(&self) -> Option<u64> {
        match &self.content {
            VzDataContent::Pointer(p) => Some(p.address),
            VzDataContent::Function(f) => Some(f.address),
            VzDataContent::Module(m) => Some(m.address),
            VzDataContent::Range(r) => Some(r.address),
            VzDataContent::Variable(v) => Some(v.address),
            _ => None,
        }
    }

    // 이름 필드를 가진 타입에서 이름 가져오기
    pub fn get_name(&self) -> Option<&str> {
        match &self.content {
            VzDataContent::Function(f) => Some(&f.name),
            VzDataContent::Method(m) => Some(&m.name),
            VzDataContent::Class(c) => Some(&c.name),
            VzDataContent::Module(m) => Some(&m.name),
            VzDataContent::Variable(v) => Some(&v.name),
            _ => None,
        }
    }
    
    // 표시 이름 가져오기
    pub fn get_display_name(&self) -> String {
        match &self.content {
            VzDataContent::Pointer(p) => format!("0x{:x}", p.address),
            VzDataContent::Function(f) => format!("{} @ 0x{:x}", f.name, f.address),
            VzDataContent::Method(m) => {
                let args = m.args.join(", ");
                format!("{}::{}({}) -> {}", m.class_name, m.name, args, m.ret)
            },
            VzDataContent::Class(c) => c.name.clone(),
            VzDataContent::Module(m) => format!("{} @ 0x{:x}", m.name, m.address),
            VzDataContent::Range(r) => format!("0x{:x} ({} bytes) [{}]", r.address, r.size, r.protection),
            VzDataContent::Variable(v) => format!("{} @ 0x{:x}", v.name, v.address),
        }
    }
} 