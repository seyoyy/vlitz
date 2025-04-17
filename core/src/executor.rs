use crate::command::{Command, CommandType, CommandArg};
use crate::store::DataStore;
use crate::navigator::Navigator;
use vlitz_shared::{VlitzError, VlitzResult, VzData, Selector};
use vlitz_shared::filter::FilterParser;
use vlitz_shared::utils::format_vzdata;
use std::str::FromStr;

/// 명령어 실행 결과 열거형
pub enum CommandResult {
    /// 성공적으로 실행됨 (표시 메시지 포함)
    Success(String),
    /// 실행 실패 (오류 메시지 포함)
    Error(String),
    /// 반환할 VzData 목록
    DataList(Vec<(usize, VzData)>),
    /// 종료 요청
    Exit,
}

/// 명령어 실행기 구조체
pub struct CommandExecutor {
    /// 데이터 저장소
    store: DataStore,
    /// 내비게이터
    navigator: Navigator,
}

impl CommandExecutor {
    /// 새 명령어 실행기 생성
    pub fn new(items_per_page: usize) -> Self {
        Self {
            store: DataStore::new(items_per_page),
            navigator: Navigator::new(),
        }
    }

    /// 명령어 실행
    pub fn execute(&mut self, command: &Command) -> CommandResult {
        let cmd_type = command.get_type();
        
        match cmd_type {
            // Navigator 명령어
            CommandType::NavSelect => self.execute_nav_select(command),
            CommandType::NavUnselect => self.execute_nav_unselect(command),
            CommandType::NavAdd => self.execute_nav_add(command),
            CommandType::NavSub => self.execute_nav_sub(command),
            CommandType::NavGoto => self.execute_nav_goto(command),
            
            // Log 명령어
            CommandType::LogList => self.execute_log_list(command),
            CommandType::LogNext => self.execute_log_next(command),
            CommandType::LogPrev => self.execute_log_prev(command),
            CommandType::LogSort => self.execute_log_sort(command),
            
            // Library 명령어
            CommandType::LibList => self.execute_lib_list(command),
            CommandType::LibSave => self.execute_lib_save(command),
            CommandType::LibMove => self.execute_lib_move(command),
            CommandType::LibRemove => self.execute_lib_remove(command),
            CommandType::LibClear => self.execute_lib_clear(command),
            
            // Meta 명령어
            CommandType::MetaLabel => self.execute_meta_label(command),
            CommandType::MetaTag => self.execute_meta_tag(command),
            CommandType::MetaUntag => self.execute_meta_untag(command),
            CommandType::MetaTags => self.execute_meta_tags(command),
            
            // 나머지 명령어들은 실제 구현에서 추가
            
            CommandType::Unknown => CommandResult::Error("Unknown command".to_string()),
            _ => CommandResult::Error("Command not implemented yet".to_string()),
        }
    }

    /// 프롬프트 문자열 가져오기
    pub fn get_prompt(&self) -> String {
        self.navigator.get_prompt()
    }

    // Navigator 명령어 실행 메서드
    fn execute_nav_select(&mut self, command: &Command) -> CommandResult {
        if command.args.is_empty() {
            return CommandResult::Error("Selector argument required".to_string());
        }

        match &command.args[0] {
            CommandArg::Selector(selector) => {
                match self.store.select_data(selector) {
                    Ok(data_list) => {
                        if data_list.len() > 1 {
                            return CommandResult::Error("Only one item can be selected".to_string());
                        }
                        
                        let data = data_list[0].clone();
                        self.navigator.select(data.clone());
                        CommandResult::Success(format!("Selected: {}", format_vzdata(0, data)))
                    },
                    Err(e) => CommandResult::Error(format!("Selection error: {}", e)),
                }
            },
            CommandArg::Number(idx) => {
                let selector = Selector::from_str(&idx.to_string()).unwrap();
                match self.store.select_data(&selector) {
                    Ok(data_list) => {
                        let data = data_list[0].clone();
                        self.navigator.select(data.clone());
                        CommandResult::Success(format!("Selected: {}", format_vzdata(*idx as usize, data)))
                    },
                    Err(e) => CommandResult::Error(format!("Selection error: {}", e)),
                }
            },
            _ => CommandResult::Error("Invalid selector argument".to_string()),
        }
    }

    fn execute_nav_unselect(&mut self, _command: &Command) -> CommandResult {
        self.navigator.unselect();
        CommandResult::Success("Selection cleared".to_string())
    }

    fn execute_nav_add(&mut self, command: &Command) -> CommandResult {
        if command.args.is_empty() {
            return CommandResult::Error("Offset argument required".to_string());
        }

        match &command.args[0] {
            CommandArg::Number(n) if *n >= 0 => {
                match self.navigator.add_offset(*n as u64) {
                    Ok(_) => CommandResult::Success(format!("Address advanced by {}", n)),
                    Err(e) => CommandResult::Error(format!("Failed to add offset: {}", e)),
                }
            },
            CommandArg::Address(addr) => {
                match self.navigator.add_offset(*addr) {
                    Ok(_) => CommandResult::Success(format!("Address advanced by {}", addr)),
                    Err(e) => CommandResult::Error(format!("Failed to add offset: {}", e)),
                }
            },
            _ => CommandResult::Error("Invalid offset argument".to_string()),
        }
    }

    fn execute_nav_sub(&mut self, command: &Command) -> CommandResult {
        if command.args.is_empty() {
            return CommandResult::Error("Offset argument required".to_string());
        }

        match &command.args[0] {
            CommandArg::Number(n) if *n >= 0 => {
                match self.navigator.sub_offset(*n as u64) {
                    Ok(_) => CommandResult::Success(format!("Address decreased by {}", n)),
                    Err(e) => CommandResult::Error(format!("Failed to subtract offset: {}", e)),
                }
            },
            CommandArg::Address(addr) => {
                match self.navigator.sub_offset(*addr) {
                    Ok(_) => CommandResult::Success(format!("Address decreased by {}", addr)),
                    Err(e) => CommandResult::Error(format!("Failed to subtract offset: {}", e)),
                }
            },
            _ => CommandResult::Error("Invalid offset argument".to_string()),
        }
    }

    fn execute_nav_goto(&mut self, command: &Command) -> CommandResult {
        if command.args.is_empty() {
            return CommandResult::Error("Address or selector argument required".to_string());
        }

        match &command.args[0] {
            CommandArg::String(addr_str) => {
                match self.navigator.goto(addr_str) {
                    Ok(_) => CommandResult::Success(format!("Navigated to {}", addr_str)),
                    Err(e) => CommandResult::Error(format!("Failed to navigate: {}", e)),
                }
            },
            CommandArg::Address(addr) => {
                let addr_str = format!("0x{:x}", addr);
                match self.navigator.goto(&addr_str) {
                    Ok(_) => CommandResult::Success(format!("Navigated to {}", addr_str)),
                    Err(e) => CommandResult::Error(format!("Failed to navigate: {}", e)),
                }
            },
            _ => CommandResult::Error("Invalid address argument".to_string()),
        }
    }

    // Log 명령어 실행 메서드
    fn execute_log_list(&mut self, _command: &Command) -> CommandResult {
        let log_items = self.store.get_current_log_page();
        
        if log_items.is_empty() {
            return CommandResult::Success("Log is empty".to_string());
        }
        
        let mut result = String::new();
        
        for (idx, data) in log_items {
            result.push_str(&format!("{}\n", format_vzdata(idx, data)));
        }
        
        CommandResult::Success(result)
    }

    fn execute_log_next(&mut self, command: &Command) -> CommandResult {
        let count = if command.args.is_empty() {
            1
        } else {
            match &command.args[0] {
                CommandArg::Number(n) if *n > 0 => *n as usize,
                _ => 1,
            }
        };
        
        match self.store.next_log_page(count) {
            Ok(page) => CommandResult::Success(format!("Moved to log page {}", page + 1)),
            Err(e) => CommandResult::Error(format!("Failed to move to next page: {}", e)),
        }
    }

    fn execute_log_prev(&mut self, command: &Command) -> CommandResult {
        let count = if command.args.is_empty() {
            1
        } else {
            match &command.args[0] {
                CommandArg::Number(n) if *n > 0 => *n as usize,
                _ => 1,
            }
        };
        
        match self.store.prev_log_page(count) {
            Ok(page) => CommandResult::Success(format!("Moved to log page {}", page + 1)),
            Err(e) => CommandResult::Error(format!("Failed to move to previous page: {}", e)),
        }
    }

    fn execute_log_sort(&mut self, command: &Command) -> CommandResult {
        let field = if command.args.is_empty() {
            "name"
        } else {
            match &command.args[0] {
                CommandArg::String(field) => field,
                _ => "name",
            }
        };
        
        match self.store.sort_log(field) {
            Ok(_) => CommandResult::Success(format!("Sorted log by {}", field)),
            Err(e) => CommandResult::Error(format!("Failed to sort log: {}", e)),
        }
    }

    // Library 명령어 실행 메서드
    fn execute_lib_list(&mut self, _command: &Command) -> CommandResult {
        let lib_items = self.store.get_current_lib_page();
        
        if lib_items.is_empty() {
            return CommandResult::Success("Library is empty".to_string());
        }
        
        let mut result = String::new();
        
        for (idx, data) in lib_items {
            result.push_str(&format!("{}\n", format_vzdata(idx, data)));
        }
        
        CommandResult::Success(result)
    }

    fn execute_lib_save(&mut self, command: &Command) -> CommandResult {
        if command.args.is_empty() {
            return CommandResult::Error("Selector argument required".to_string());
        }

        match &command.args[0] {
            CommandArg::Selector(selector) => {
                match self.store.save_to_lib(selector) {
                    Ok(count) => CommandResult::Success(format!("Saved {} items to library", count)),
                    Err(e) => CommandResult::Error(format!("Failed to save to library: {}", e)),
                }
            },
            _ => CommandResult::Error("Invalid selector argument".to_string()),
        }
    }

    fn execute_lib_move(&mut self, command: &Command) -> CommandResult {
        if command.args.len() < 2 {
            return CommandResult::Error("Two index arguments required".to_string());
        }

        let from_idx = match &command.args[0] {
            CommandArg::Number(n) if *n >= 0 => *n as usize,
            _ => return CommandResult::Error("Invalid from index".to_string()),
        };

        let to_idx = match &command.args[1] {
            CommandArg::Number(n) if *n >= 0 => *n as usize,
            _ => return CommandResult::Error("Invalid to index".to_string()),
        };

        match self.store.move_in_lib(from_idx, to_idx) {
            Ok(_) => CommandResult::Success(format!("Moved item from {} to {}", from_idx, to_idx)),
            Err(e) => CommandResult::Error(format!("Failed to move item: {}", e)),
        }
    }

    fn execute_lib_remove(&mut self, command: &Command) -> CommandResult {
        if command.args.is_empty() {
            return CommandResult::Error("Selector argument required".to_string());
        }

        match &command.args[0] {
            CommandArg::Selector(selector) => {
                match self.store.remove_from_lib(selector) {
                    Ok(count) => CommandResult::Success(format!("Removed {} items from library", count)),
                    Err(e) => CommandResult::Error(format!("Failed to remove from library: {}", e)),
                }
            },
            _ => CommandResult::Error("Invalid selector argument".to_string()),
        }
    }

    fn execute_lib_clear(&mut self, command: &Command) -> CommandResult {
        let filter_expr = if command.args.is_empty() {
            None
        } else {
            match &command.args[0] {
                CommandArg::FilterExpr(expr) => Some(expr.as_str()),
                _ => None,
            }
        };

        match self.store.clear_lib(filter_expr) {
            Ok(count) => CommandResult::Success(format!("Cleared {} items from library", count)),
            Err(e) => CommandResult::Error(format!("Failed to clear library: {}", e)),
        }
    }

    // Meta 명령어 실행 메서드
    fn execute_meta_label(&mut self, command: &Command) -> CommandResult {
        if command.args.len() < 2 {
            return CommandResult::Error("Selector and label arguments required".to_string());
        }

        let selector = match &command.args[0] {
            CommandArg::Selector(selector) => selector,
            _ => return CommandResult::Error("Invalid selector argument".to_string()),
        };

        let label = match &command.args[1] {
            CommandArg::String(label) => label,
            _ => return CommandResult::Error("Invalid label argument".to_string()),
        };

        match self.store.get_data_mut(selector) {
            Ok(data_list) => {
                for data in data_list {
                    data.set_label(label.clone());
                }
                CommandResult::Success(format!("Applied label '{}' to {} items", label, data_list.len()))
            },
            Err(e) => CommandResult::Error(format!("Failed to label items: {}", e)),
        }
    }

    fn execute_meta_tag(&mut self, command: &Command) -> CommandResult {
        if command.args.len() < 2 {
            return CommandResult::Error("Selector and tag arguments required".to_string());
        }

        let selector = match &command.args[0] {
            CommandArg::Selector(selector) => selector,
            _ => return CommandResult::Error("Invalid selector argument".to_string()),
        };

        let tag = match &command.args[1] {
            CommandArg::String(tag) => tag,
            _ => return CommandResult::Error("Invalid tag argument".to_string()),
        };

        match self.store.get_data_mut(selector) {
            Ok(data_list) => {
                for data in data_list {
                    data.add_tag(tag.clone());
                }
                CommandResult::Success(format!("Added tag '{}' to {} items", tag, data_list.len()))
            },
            Err(e) => CommandResult::Error(format!("Failed to tag items: {}", e)),
        }
    }

    fn execute_meta_untag(&mut self, command: &Command) -> CommandResult {
        if command.args.len() < 2 {
            return CommandResult::Error("Selector and tag arguments required".to_string());
        }

        let selector = match &command.args[0] {
            CommandArg::Selector(selector) => selector,
            _ => return CommandResult::Error("Invalid selector argument".to_string()),
        };

        let tag = match &command.args[1] {
            CommandArg::String(tag) => tag,
            _ => return CommandResult::Error("Invalid tag argument".to_string()),
        };

        match self.store.get_data_mut(selector) {
            Ok(data_list) => {
                let mut removed = 0;
                for data in data_list {
                    if data.remove_tag(tag) {
                        removed += 1;
                    }
                }
                CommandResult::Success(format!("Removed tag '{}' from {} items", tag, removed))
            },
            Err(e) => CommandResult::Error(format!("Failed to untag items: {}", e)),
        }
    }

    fn execute_meta_tags(&mut self, command: &Command) -> CommandResult {
        if command.args.is_empty() {
            return CommandResult::Error("Selector argument required".to_string());
        }

        let selector = match &command.args[0] {
            CommandArg::Selector(selector) => selector,
            _ => return CommandResult::Error("Invalid selector argument".to_string()),
        };

        match self.store.select_data(selector) {
            Ok(data_list) => {
                let mut result = String::new();
                
                for (i, data) in data_list.iter().enumerate() {
                    let tags: Vec<_> = data.tags.iter().collect();
                    result.push_str(&format!("Item {}: [{}]\n", i, tags.join(", ")));
                }
                
                if result.is_empty() {
                    result = "No tags found".to_string();
                }
                
                CommandResult::Success(result)
            },
            Err(e) => CommandResult::Error(format!("Failed to get tags: {}", e)),
        }
    }
} 