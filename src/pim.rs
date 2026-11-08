//! PIM item definitions for akonadi.

/// PIM type
#[derive(Debug, Clone, PartialEq)]
pub enum PIMType {
    /// Contact PIM
    Contact,
    /// Calendar PIM
    Calendar,
    /// Task PIM
    Task,
    /// Note PIM
    Note,
    /// Address book PIM
    AddressBook,
}

/// PIM status
#[derive(Debug, Clone, PartialEq)]
pub enum PIMStatus {
    /// Active PIM
    Active,
    /// Inactive PIM
    Inactive,
    /// Deleted PIM
    Deleted,
}

/// PIM item
#[derive(Debug, Clone)]
pub struct PIMItem {
    /// PIM ID
    pub id: String,
    /// PIM type
    pub pim_type: PIMType,
    /// PIM status
    pub status: PIMStatus,
    /// PIM name
    pub name: String,
    /// PIM description
    pub description: String,
    /// PIM created timestamp
    pub created_at: u64,
    /// PIM updated timestamp
    pub updated_at: u64,
    /// PIM data
    pub data: std::collections::HashMap<String, String>,
}

impl PIMItem {
    /// Create new PIM item
    pub fn new(id: String, pim_type: PIMType, name: String) -> Self {
        Self {
            id,
            pim_type,
            status: PIMStatus::Active,
            name,
            description: String::new(),
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            updated_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            data: std::collections::HashMap::new(),
        }
    }
    
    /// Set description
    pub fn set_description(&mut self, description: String) {
        self.description = description;
        self.update_timestamp();
    }
    
    /// Set data
    pub fn set_data(&mut self, key: String, value: String) {
        self.data.insert(key, value);
        self.update_timestamp();
    }
    
    /// Get data
    pub fn get_data(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }
    
    /// Remove data
    pub fn remove_data(&mut self, key: &str) -> bool {
        if self.data.remove(key).is_some() {
            self.update_timestamp();
            return true;
        }
        false
    }
    
    /// Set status
    pub fn set_status(&mut self, status: PIMStatus) {
        self.status = status;
        self.update_timestamp();
    }
    
    /// Update timestamp
    fn update_timestamp(&mut self) {
        self.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    /// Check if PIM is active
    pub fn is_active(&self) -> bool {
        self.status == PIMStatus::Active
    }
    
    /// Check if PIM is deleted
    pub fn is_deleted(&self) -> bool {
        self.status == PIMStatus::Deleted
    }
    
    /// Get PIM age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.created_at
    }
    
    /// Get time since update in seconds
    pub fn get_time_since_update(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.updated_at
    }
    
    /// Get data count
    pub fn get_data_count(&self) -> usize {
        self.data.len()
    }
    
    /// Check if PIM has data
    pub fn has_data(&self) -> bool {
        !self.data.is_empty()
    }
}
