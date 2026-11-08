//! Note management for akonadi.

/// Note type
#[derive(Debug, Clone, PartialEq)]
pub enum NoteType {
    /// Text note
    Text,
    /// Checklist note
    Checklist,
    /// Drawing note
    Drawing,
    /// Voice note
    Voice,
    /// Image note
    Image,
}

/// Note category
#[derive(Debug, Clone, PartialEq)]
pub enum NoteCategory {
    /// Personal note
    Personal,
    /// Work note
    Work,
    /// Meeting note
    Meeting,
    /// Idea note
    Idea,
    /// Custom note
    Custom,
}

/// Note tag
#[derive(Debug, Clone)]
pub struct NoteTag {
    /// Tag name
    pub name: String,
    /// Tag color
    pub color: String,
}

/// Note
#[derive(Debug, Clone)]
pub struct Note {
    /// Note ID
    pub id: String,
    /// Note title
    pub title: String,
    /// Note content
    pub content: String,
    /// Note type
    pub note_type: NoteType,
    /// Note category
    pub category: NoteCategory,
    /// Note tags
    pub tags: Vec<NoteTag>,
    /// Note created timestamp
    pub created_at: u64,
    /// Note updated timestamp
    pub updated_at: u64,
    /// Note pinned
    pub pinned: bool,
    /// Note archived
    pub archived: bool,
    /// Note attachments
    pub attachments: Vec<String>,
}

impl Note {
    /// Create new note
    pub fn new(id: String, title: String, note_type: NoteType) -> Self {
        Self {
            id,
            title,
            content: String::new(),
            note_type,
            category: NoteCategory::Custom,
            tags: vec![],
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            updated_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            pinned: false,
            archived: false,
            attachments: vec![],
        }
    }
    
    /// Set content
    pub fn set_content(&mut self, content: String) {
        self.content = content;
        self.update_timestamp();
    }
    
    /// Set category
    pub fn set_category(&mut self, category: NoteCategory) {
        self.category = category;
        self.update_timestamp();
    }
    
    /// Add tag
    pub fn add_tag(&mut self, tag: NoteTag) {
        if !self.tags.iter().any(|t| t.name == tag.name) {
            self.tags.push(tag);
            self.update_timestamp();
        }
    }
    
    /// Remove tag
    pub fn remove_tag(&mut self, tag_name: &str) -> bool {
        if let Some(pos) = self.tags.iter().position(|t| t.name == tag_name) {
            self.tags.remove(pos);
            self.update_timestamp();
            return true;
        }
        false
    }
    
    /// Pin note
    pub fn pin(&mut self) {
        self.pinned = true;
        self.update_timestamp();
    }
    
    /// Unpin note
    pub fn unpin(&mut self) {
        self.pinned = false;
        self.update_timestamp();
    }
    
    /// Archive note
    pub fn archive(&mut self) {
        self.archived = true;
        self.update_timestamp();
    }
    
    /// Unarchive note
    pub fn unarchive(&mut self) {
        self.archived = false;
        self.update_timestamp();
    }
    
    /// Add attachment
    pub fn add_attachment(&mut self, attachment: String) {
        if !self.attachments.contains(&attachment) {
            self.attachments.push(attachment);
            self.update_timestamp();
        }
    }
    
    /// Remove attachment
    pub fn remove_attachment(&mut self, attachment: &str) -> bool {
        if let Some(pos) = self.attachments.iter().position(|a| a == attachment) {
            self.attachments.remove(pos);
            self.update_timestamp();
            return true;
        }
        false
    }
    
    /// Update timestamp
    fn update_timestamp(&mut self) {
        self.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    /// Check if note is pinned
    pub fn is_pinned(&self) -> bool {
        self.pinned
    }
    
    /// Check if note is archived
    pub fn is_archived(&self) -> bool {
        self.archived
    }
    
    /// Get note age in seconds
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
    
    /// Get content length
    pub fn get_content_length(&self) -> usize {
        self.content.len()
    }
    
    /// Get tag count
    pub fn get_tag_count(&self) -> usize {
        self.tags.len()
    }
    
    /// Get attachment count
    pub fn get_attachment_count(&self) -> usize {
        self.attachments.len()
    }
    
    /// Check if note has content
    pub fn has_content(&self) -> bool {
        !self.content.is_empty()
    }
    
    /// Check if note has tags
    pub fn has_tags(&self) -> bool {
        !self.tags.is_empty()
    }
    
    /// Check if note has attachments
    pub fn has_attachments(&self) -> bool {
        !self.attachments.is_empty()
    }
    
    /// Get content preview
    pub fn get_content_preview(&self, max_length: usize) -> String {
        if self.content.len() > max_length {
            format!("{}...", &self.content[..max_length])
        } else {
            self.content.clone()
        }
    }
}
