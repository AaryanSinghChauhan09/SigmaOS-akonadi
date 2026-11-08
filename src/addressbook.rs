//! Address book management for akonadi.

/// Entry type
#[derive(Debug, Clone, PartialEq)]
pub enum EntryType {
    /// Contact entry
    Contact,
    /// Organization entry
    Organization,
    /// Group entry
    Group,
    /// Location entry
    Location,
    /// Custom entry
    Custom,
}

/// Entry status
#[derive(Debug, Clone, PartialEq)]
pub enum EntryStatus {
    /// Active entry
    Active,
    /// Inactive entry
    Inactive,
    /// Deleted entry
    Deleted,
}

/// Address book entry
#[derive(Debug, Clone)]
pub struct AddressBookEntry {
    /// Entry ID
    pub id: String,
    /// Entry name
    pub name: String,
    /// Entry type
    pub entry_type: EntryType,
    /// Entry status
    pub status: EntryStatus,
    /// Entry email
    pub email: Option<String>,
    /// Entry phone
    pub phone: Option<String>,
    /// Entry address
    pub address: Option<String>,
    /// Entry notes
    pub notes: String,
    /// Entry created timestamp
    pub created_at: u64,
    /// Entry updated timestamp
    pub updated_at: u64,
    /// Entry tags
    pub tags: Vec<String>,
}

impl AddressBookEntry {
    /// Create new address book entry
    pub fn new(id: String, name: String, entry_type: EntryType) -> Self {
        Self {
            id,
            name,
            entry_type,
            status: EntryStatus::Active,
            email: None,
            phone: None,
            address: None,
            notes: String::new(),
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            updated_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            tags: vec![],
        }
    }
    
    /// Set email
    pub fn set_email(&mut self, email: String) {
        self.email = Some(email);
        self.update_timestamp();
    }
    
    /// Set phone
    pub fn set_phone(&mut self, phone: String) {
        self.phone = Some(phone);
        self.update_timestamp();
    }
    
    /// Set address
    pub fn set_address(&mut self, address: String) {
        self.address = Some(address);
        self.update_timestamp();
    }
    
    /// Set notes
    pub fn set_notes(&mut self, notes: String) {
        self.notes = notes;
        self.update_timestamp();
    }
    
    /// Add tag
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
            self.update_timestamp();
        }
    }
    
    /// Remove tag
    pub fn remove_tag(&mut self, tag: &str) -> bool {
        if let Some(pos) = self.tags.iter().position(|t| t == tag) {
            self.tags.remove(pos);
            self.update_timestamp();
            return true;
        }
        false
    }
    
    /// Set status
    pub fn set_status(&mut self, status: EntryStatus) {
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
    
    /// Check if entry is active
    pub fn is_active(&self) -> bool {
        self.status == EntryStatus::Active
    }
    
    /// Check if entry is deleted
    pub fn is_deleted(&self) -> bool {
        self.status == EntryStatus::Deleted
    }
    
    /// Get entry age in seconds
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
    
    /// Get tag count
    pub fn get_tag_count(&self) -> usize {
        self.tags.len()
    }
    
    /// Check if entry has email
    pub fn has_email(&self) -> bool {
        self.email.is_some()
    }
    
    /// Check if entry has phone
    pub fn has_phone(&self) -> bool {
        self.phone.is_some()
    }
    
    /// Check if entry has address
    pub fn has_address(&self) -> bool {
        self.address.is_some()
    }
    
    /// Check if entry has tags
    pub fn has_tags(&self) -> bool {
        !self.tags.is_empty()
    }
    
    /// Get initials
    pub fn get_initials(&self) -> String {
        self.name.split_whitespace()
            .map(|word| word.chars().next().unwrap_or(' '))
            .collect::<String>()
            .to_uppercase()
    }
}

/// Address book
#[derive(Debug, Clone)]
pub struct AddressBook {
    /// Address book ID
    pub id: String,
    /// Address book name
    pub name: String,
    /// Address book description
    pub description: String,
    /// Address book entries
    pub entries: Vec<AddressBookEntry>,
    /// Address book created timestamp
    pub created_at: u64,
    /// Address book updated timestamp
    pub updated_at: u64,
    /// Address book color
    pub color: String,
    /// Address book visible
    pub visible: bool,
    /// Address book readonly
    pub readonly: bool,
}

impl AddressBook {
    /// Create new address book
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            description: String::new(),
            entries: vec![],
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            updated_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            color: "#000000".into(),
            visible: true,
            readonly: false,
        }
    }
    
    /// Set description
    pub fn set_description(&mut self, description: String) {
        self.description = description;
        self.update_timestamp();
    }
    
    /// Set color
    pub fn set_color(&mut self, color: String) {
        self.color = color;
        self.update_timestamp();
    }
    
    /// Set visible
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
        self.update_timestamp();
    }
    
    /// Set readonly
    pub fn set_readonly(&mut self, readonly: bool) {
        self.readonly = readonly;
        self.update_timestamp();
    }
    
    /// Add entry
    pub fn add_entry(&mut self, entry: AddressBookEntry) -> String {
        let id = entry.id.clone();
        self.entries.push(entry);
        self.update_timestamp();
        id
    }
    
    /// Remove entry
    pub fn remove_entry(&mut self, entry_id: &str) -> bool {
        if let Some(pos) = self.entries.iter().position(|e| e.id == entry_id) {
            self.entries.remove(pos);
            self.update_timestamp();
            return true;
        }
        false
    }
    
    /// Get entry
    pub fn get_entry(&self, entry_id: &str) -> Option<&AddressBookEntry> {
        self.entries.iter().find(|e| e.id == entry_id)
    }
    
    /// Get entry mutably
    pub fn get_entry_mut(&mut self, entry_id: &str) -> Option<&mut AddressBookEntry> {
        self.entries.iter_mut().find(|e| e.id == entry_id)
    }
    
    /// Get all entries
    pub fn get_entries(&self) -> &[AddressBookEntry] {
        &self.entries
    }
    
    /// Get entries by type
    pub fn get_entries_by_type(&self, entry_type: &EntryType) -> Vec<&AddressBookEntry> {
        self.entries.iter()
            .filter(|e| e.entry_type == *entry_type)
            .collect()
    }
    
    /// Get entries by status
    pub fn get_entries_by_status(&self, status: &EntryStatus) -> Vec<&AddressBookEntry> {
        self.entries.iter()
            .filter(|e| e.status == *status)
            .collect()
    }
    
    /// Search entries
    pub fn search_entries(&self, query: &str) -> Vec<&AddressBookEntry> {
        let query_lower = query.to_lowercase();
        self.entries.iter()
            .filter(|e| e.name.to_lowercase().contains(&query_lower) ||
                       e.email.as_ref().map(|e| e.to_lowercase().contains(&query_lower)).unwrap_or(false) ||
                       e.phone.as_ref().map(|p| p.to_lowercase().contains(&query_lower)).unwrap_or(false))
            .collect()
    }
    
    /// Update timestamp
    fn update_timestamp(&mut self) {
        self.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    /// Get entry count
    pub fn get_entry_count(&self) -> usize {
        self.entries.len()
    }
    
    /// Get address book age in seconds
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
    
    /// Check if address book is visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }
    
    /// Check if address book is readonly
    pub fn is_readonly(&self) -> bool {
        self.readonly
    }
}
