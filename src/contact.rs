//! Contact management for akonadi.

/// Contact type
#[derive(Debug, Clone, PartialEq)]
pub enum ContactType {
    /// Person contact
    Person,
    /// Organization contact
    Organization,
    /// Group contact
    Group,
    /// Custom contact
    Custom,
}

/// Contact status
#[derive(Debug, Clone, PartialEq)]
pub enum ContactStatus {
    /// Active contact
    Active,
    /// Inactive contact
    Inactive,
    /// Deleted contact
    Deleted,
}

/// Contact relationship
#[derive(Debug, Clone, PartialEq)]
pub enum ContactRelationship {
    /// Family
    Family,
    /// Friend
    Friend,
    /// Colleague
    Colleague,
    /// Acquaintance
    Acquaintance,
    /// Custom
    Custom,
}

/// Contact group
#[derive(Debug, Clone)]
pub struct ContactGroup {
    /// Group ID
    pub id: String,
    /// Group name
    pub name: String,
    /// Group description
    pub description: String,
    /// Group contacts
    pub contacts: Vec<String>,
}

/// Contact
#[derive(Debug, Clone)]
pub struct Contact {
    /// Contact ID
    pub id: String,
    /// Contact name
    pub name: String,
    /// Contact type
    pub contact_type: ContactType,
    /// Contact status
    pub status: ContactStatus,
    /// Contact email
    pub email: Option<String>,
    /// Contact phone
    pub phone: Option<String>,
    /// Contact organization
    pub organization: Option<String>,
    /// Contact title
    pub title: Option<String>,
    /// Contact address
    pub address: Option<String>,
    /// Contact website
    pub website: Option<String>,
    /// Contact notes
    pub notes: String,
    /// Contact relationships
    pub relationships: Vec<ContactRelationship>,
    /// Contact groups
    pub groups: Vec<String>,
    /// Contact created timestamp
    pub created_at: u64,
    /// Contact updated timestamp
    pub updated_at: u64,
    /// Contact birthday
    pub birthday: Option<u64>,
    /// Contact anniversary
    pub anniversary: Option<u64>,
}

impl Contact {
    /// Create new contact
    pub fn new(id: String, name: String, contact_type: ContactType) -> Self {
        Self {
            id,
            name,
            contact_type,
            status: ContactStatus::Active,
            email: None,
            phone: None,
            organization: None,
            title: None,
            address: None,
            website: None,
            notes: String::new(),
            relationships: vec![],
            groups: vec![],
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            updated_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            birthday: None,
            anniversary: None,
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
    
    /// Set organization
    pub fn set_organization(&mut self, organization: String) {
        self.organization = Some(organization);
        self.update_timestamp();
    }
    
    /// Set title
    pub fn set_title(&mut self, title: String) {
        self.title = Some(title);
        self.update_timestamp();
    }
    
    /// Set address
    pub fn set_address(&mut self, address: String) {
        self.address = Some(address);
        self.update_timestamp();
    }
    
    /// Set website
    pub fn set_website(&mut self, website: String) {
        self.website = Some(website);
        self.update_timestamp();
    }
    
    /// Set notes
    pub fn set_notes(&mut self, notes: String) {
        self.notes = notes;
        self.update_timestamp();
    }
    
    /// Add relationship
    pub fn add_relationship(&mut self, relationship: ContactRelationship) {
        if !self.relationships.contains(&relationship) {
            self.relationships.push(relationship);
            self.update_timestamp();
        }
    }
    
    /// Remove relationship
    pub fn remove_relationship(&mut self, relationship: &ContactRelationship) -> bool {
        if let Some(pos) = self.relationships.iter().position(|r| r == relationship) {
            self.relationships.remove(pos);
            self.update_timestamp();
            return true;
        }
        false
    }
    
    /// Add to group
    pub fn add_to_group(&mut self, group_id: String) {
        if !self.groups.contains(&group_id) {
            self.groups.push(group_id);
            self.update_timestamp();
        }
    }
    
    /// Remove from group
    pub fn remove_from_group(&mut self, group_id: &str) -> bool {
        if let Some(pos) = self.groups.iter().position(|g| g == group_id) {
            self.groups.remove(pos);
            self.update_timestamp();
            return true;
        }
        false
    }
    
    /// Set birthday
    pub fn set_birthday(&mut self, birthday: u64) {
        self.birthday = Some(birthday);
        self.update_timestamp();
    }
    
    /// Set anniversary
    pub fn set_anniversary(&mut self, anniversary: u64) {
        self.anniversary = Some(anniversary);
        self.update_timestamp();
    }
    
    /// Set status
    pub fn set_status(&mut self, status: ContactStatus) {
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
    
    /// Check if contact is active
    pub fn is_active(&self) -> bool {
        self.status == ContactStatus::Active
    }
    
    /// Check if contact is deleted
    pub fn is_deleted(&self) -> bool {
        self.status == ContactStatus::Deleted
    }
    
    /// Get contact age in seconds
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
    
    /// Get initials
    pub fn get_initials(&self) -> String {
        self.name.split_whitespace()
            .map(|word| word.chars().next().unwrap_or(' '))
            .collect::<String>()
            .to_uppercase()
    }
    
    /// Check if contact has email
    pub fn has_email(&self) -> bool {
        self.email.is_some()
    }
    
    /// Check if contact has phone
    pub fn has_phone(&self) -> bool {
        self.phone.is_some()
    }
    
    /// Check if contact has organization
    pub fn has_organization(&self) -> bool {
        self.organization.is_some()
    }
    
    /// Check if contact has groups
    pub fn has_groups(&self) -> bool {
        !self.groups.is_empty()
    }
    
    /// Check if contact has relationships
    pub fn has_relationships(&self) -> bool {
        !self.relationships.is_empty()
    }
}
