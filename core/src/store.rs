use vlitz_shared::{VzData, VlitzError, VlitzResult, Selector, FilterCondition};
use vlitz_shared::filter::FilterParser;

/// 데이터 스토어 구조체 (log, lib 데이터 관리)
pub struct DataStore {
    /// 라이브러리 데이터
    lib: Vec<VzData>,
    /// 로그 데이터
    log: Vec<VzData>,
    /// 현재 로그 페이지
    current_page: usize,
    /// 페이지당 항목 수
    items_per_page: usize,
}

impl DataStore {
    /// 새로운 데이터 스토어 생성
    pub fn new(items_per_page: usize) -> Self {
        Self {
            lib: Vec::new(),
            log: Vec::new(),
            current_page: 0,
            items_per_page,
        }
    }

    /// 로그에 데이터 추가
    pub fn add_to_log(&mut self, data: VzData) -> usize {
        let index = self.log.len();
        self.log.push(data);
        index
    }

    /// 로그에 여러 데이터 추가
    pub fn add_multiple_to_log(&mut self, data: Vec<VzData>) -> usize {
        let start_index = self.log.len();
        self.log.extend(data);
        start_index
    }

    /// 로그에서 라이브러리로 데이터 이동
    pub fn save_to_lib(&mut self, selector: &Selector) -> VlitzResult<usize> {
        let data = self.select_data(selector)?;
        let start_index = self.lib.len();
        
        // 데이터 복제
        for item in data {
            self.lib.push(item.clone());
        }
        
        Ok(start_index)
    }

    /// 라이브러리 내에서 데이터 이동
    pub fn move_in_lib(&mut self, from_idx: usize, to_idx: usize) -> VlitzResult<()> {
        if from_idx >= self.lib.len() {
            return Err(VlitzError::Selector(format!("Source index {} out of bounds", from_idx)));
        }
        
        let mut to_idx = to_idx;
        if to_idx > self.lib.len() {
            to_idx = self.lib.len();
        }
        
        let item = self.lib.remove(from_idx);
        self.lib.insert(to_idx, item);
        
        Ok(())
    }

    /// 라이브러리에서 데이터 제거
    pub fn remove_from_lib(&mut self, selector: &Selector) -> VlitzResult<usize> {
        let indices = selector.get_indices(self.lib.len(), self.log.len());
        
        // 내림차순으로 정렬 (제거 시 인덱스 변화 방지)
        let mut indices: Vec<_> = indices.into_iter()
            .filter(|&idx| idx < self.lib.len())
            .collect();
        indices.sort_unstable_by(|a, b| b.cmp(a));
        
        let mut removed = 0;
        for idx in indices {
            self.lib.remove(idx);
            removed += 1;
        }
        
        if removed == 0 {
            return Err(VlitzError::Selector("No data found for selector".to_string()));
        }
        
        Ok(removed)
    }

    /// 라이브러리 초기화
    pub fn clear_lib(&mut self, filter_expr: Option<&str>) -> VlitzResult<usize> {
        let removed = self.lib.len();
        
        if let Some(expr) = filter_expr {
            let filter = FilterParser::parse(expr)?;
            self.lib.retain(|data| !filter.apply(data));
            return Ok(removed - self.lib.len());
        }
        
        self.lib.clear();
        Ok(removed)
    }

    /// 셀렉터로 데이터 선택
    pub fn select_data<'a>(&'a self, selector: &Selector) -> VlitzResult<Vec<&'a VzData>> {
        let indices = selector.get_indices(self.lib.len(), self.log.len());
        let mut result = Vec::new();
        
        for idx in indices {
            if idx < self.lib.len() {
                result.push(&self.lib[idx]);
            } else if idx < self.log.len() {
                result.push(&self.log[idx]);
            }
        }
        
        if result.is_empty() {
            return Err(VlitzError::Selector("No data found for selector".to_string()));
        }
        
        Ok(result)
    }

    /// 필터로 데이터 필터링
    pub fn filter_data<'a>(&'a self, filter: &FilterCondition) -> Vec<(usize, &'a VzData)> {
        let mut result = Vec::new();
        
        // 라이브러리 데이터 필터링
        for (idx, data) in self.lib.iter().enumerate() {
            if filter.apply(data) {
                result.push((idx, data));
            }
        }
        
        // 로그 데이터 필터링
        for (idx, data) in self.log.iter().enumerate() {
            if filter.apply(data) {
                result.push((idx + self.lib.len(), data));
            }
        }
        
        result
    }

    /// 로그 현재 페이지 가져오기
    pub fn get_current_log_page(&self) -> Vec<(usize, &VzData)> {
        let start = self.current_page * self.items_per_page;
        let end = std::cmp::min(start + self.items_per_page, self.log.len());
        
        if start >= self.log.len() {
            return Vec::new();
        }
        
        self.log[start..end].iter()
            .enumerate()
            .map(|(i, data)| (start + i, data))
            .collect()
    }

    /// 라이브러리 현재 페이지 가져오기
    pub fn get_current_lib_page(&self) -> Vec<(usize, &VzData)> {
        let start = self.current_page * self.items_per_page;
        let end = std::cmp::min(start + self.items_per_page, self.lib.len());
        
        if start >= self.lib.len() {
            return Vec::new();
        }
        
        self.lib[start..end].iter()
            .enumerate()
            .map(|(i, data)| (start + i, data))
            .collect()
    }

    /// 로그 다음 페이지로 이동
    pub fn next_log_page(&mut self, count: usize) -> VlitzResult<usize> {
        let page_count = (self.log.len() + self.items_per_page - 1) / self.items_per_page;
        
        if page_count == 0 {
            return Err(VlitzError::General("Log is empty".to_string()));
        }
        
        let new_page = self.current_page + count;
        if new_page >= page_count {
            self.current_page = page_count - 1;
        } else {
            self.current_page = new_page;
        }
        
        Ok(self.current_page)
    }

    /// 로그 이전 페이지로 이동
    pub fn prev_log_page(&mut self, count: usize) -> VlitzResult<usize> {
        if count > self.current_page {
            self.current_page = 0;
        } else {
            self.current_page -= count;
        }
        
        Ok(self.current_page)
    }

    /// 로그 정렬
    pub fn sort_log(&mut self, field: &str) -> VlitzResult<()> {
        match field {
            "name" => {
                self.log.sort_by(|a, b| {
                    let a_name = a.get_name().unwrap_or("");
                    let b_name = b.get_name().unwrap_or("");
                    a_name.cmp(b_name)
                });
            },
            "address" => {
                self.log.sort_by(|a, b| {
                    let a_addr = a.get_address().unwrap_or(0);
                    let b_addr = b.get_address().unwrap_or(0);
                    a_addr.cmp(&b_addr)
                });
            },
            "type" => {
                self.log.sort_by(|a, b| {
                    format!("{:?}", a.data_type).cmp(&format!("{:?}", b.data_type))
                });
            },
            _ => return Err(VlitzError::General(format!("Unknown sort field: {}", field))),
        }
        
        Ok(())
    }

    /// 라이브러리 데이터 가져오기
    pub fn get_lib(&self) -> &[VzData] {
        &self.lib
    }

    /// 로그 데이터 가져오기
    pub fn get_log(&self) -> &[VzData] {
        &self.log
    }

    /// 데이터 변경
    pub fn get_data_mut<'a>(&'a mut self, selector: &Selector) -> VlitzResult<Vec<&'a mut VzData>> {
        let indices = selector.get_indices(self.lib.len(), self.log.len());
        let mut result = Vec::new();
        
        // 인덱스 중복 제거
        let mut unique_indices = indices;
        unique_indices.sort_unstable();
        unique_indices.dedup();
        
        for idx in unique_indices {
            if idx < self.lib.len() {
                result.push(&mut self.lib[idx]);
            } else if idx - self.lib.len() < self.log.len() {
                result.push(&mut self.log[idx - self.lib.len()]);
            }
        }
        
        if result.is_empty() {
            return Err(VlitzError::Selector("No data found for selector".to_string()));
        }
        
        Ok(result)
    }
} 