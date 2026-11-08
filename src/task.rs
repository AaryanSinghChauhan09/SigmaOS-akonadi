//! Task management for akonadi.

/// Task status
#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    /// Pending task
    Pending,
    /// In progress task
    InProgress,
    /// Completed task
    Completed,
    /// Cancelled task
    Cancelled,
    /// Deferred task
    Deferred,
}

/// Task priority
#[derive(Debug, Clone, PartialEq)]
pub enum TaskPriority {
    /// Low priority
    Low,
    /// Medium priority
    Medium,
    /// High priority
    High,
    /// Critical priority
    Critical,
}

/// Task category
#[derive(Debug, Clone, PartialEq)]
pub enum TaskCategory {
    /// Work task
    Work,
    /// Personal task
    Personal,
    /// Home task
    Home,
    /// Shopping task
    Shopping,
    /// Custom task
    Custom,
}

/// Task dependency
#[derive(Debug, Clone)]
pub struct TaskDependency {
    /// Dependency task ID
    pub task_id: String,
    /// Dependency type
    pub dependency_type: DependencyType,
}

/// Dependency type
#[derive(Debug, Clone, PartialEq)]
pub enum DependencyType {
    /// Blocks dependency
    Blocks,
    /// Blocked by dependency
    BlockedBy,
    /// Related dependency
    Related,
}

/// Task
#[derive(Debug, Clone)]
pub struct Task {
    /// Task ID
    pub id: String,
    /// Task title
    pub title: String,
    /// Task description
    pub description: String,
    /// Task status
    pub status: TaskStatus,
    /// Task priority
    pub priority: TaskPriority,
    /// Task category
    pub category: TaskCategory,
    /// Task due date
    pub due_date: Option<u64>,
    /// Task start date
    pub start_date: Option<u64>,
    /// Task completed date
    pub completed_date: Option<u64>,
    /// Task estimated duration in minutes
    pub estimated_duration: Option<u32>,
    /// Task actual duration in minutes
    pub actual_duration: Option<u32>,
    /// Task dependencies
    pub dependencies: Vec<TaskDependency>,
    /// Task tags
    pub tags: Vec<String>,
    /// Task notes
    pub notes: String,
    /// Task created timestamp
    pub created_at: u64,
    /// Task updated timestamp
    pub updated_at: u64,
    /// Task reminder
    pub reminder: Option<u64>,
    /// Task recurrence
    pub recurrence: Option<TaskRecurrence>,
}

/// Task recurrence
#[derive(Debug, Clone)]
pub struct TaskRecurrence {
    /// Recurrence type
    pub recurrence_type: RecurrenceType,
    /// Recurrence interval
    pub interval: u32,
    /// Recurrence end date
    pub end_date: Option<u64>,
    /// Recurrence count
    pub count: Option<u32>,
}

/// Recurrence type
#[derive(Debug, Clone, PartialEq)]
pub enum RecurrenceType {
    /// Daily recurrence
    Daily,
    /// Weekly recurrence
    Weekly,
    /// Monthly recurrence
    Monthly,
    /// Yearly recurrence
    Yearly,
    /// Custom recurrence
    Custom,
}

impl Task {
    /// Create new task
    pub fn new(id: String, title: String, priority: TaskPriority) -> Self {
        Self {
            id,
            title,
            description: String::new(),
            status: TaskStatus::Pending,
            priority,
            category: TaskCategory::Custom,
            due_date: None,
            start_date: None,
            completed_date: None,
            estimated_duration: None,
            actual_duration: None,
            dependencies: vec![],
            tags: vec![],
            notes: String::new(),
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            updated_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            reminder: None,
            recurrence: None,
        }
    }
    
    /// Set description
    pub fn set_description(&mut self, description: String) {
        self.description = description;
        self.update_timestamp();
    }
    
    /// Set status
    pub fn set_status(&mut self, status: TaskStatus) {
        let status_clone = status.clone();
        self.status = status;
        if status_clone == TaskStatus::Completed {
            self.completed_date = Some(std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs());
        }
        self.update_timestamp();
    }
    
    /// Set category
    pub fn set_category(&mut self, category: TaskCategory) {
        self.category = category;
        self.update_timestamp();
    }
    
    /// Set due date
    pub fn set_due_date(&mut self, due_date: u64) {
        self.due_date = Some(due_date);
        self.update_timestamp();
    }
    
    /// Set start date
    pub fn set_start_date(&mut self, start_date: u64) {
        self.start_date = Some(start_date);
        self.update_timestamp();
    }
    
    /// Set estimated duration
    pub fn set_estimated_duration(&mut self, duration: u32) {
        self.estimated_duration = Some(duration);
        self.update_timestamp();
    }
    
    /// Set actual duration
    pub fn set_actual_duration(&mut self, duration: u32) {
        self.actual_duration = Some(duration);
        self.update_timestamp();
    }
    
    /// Add dependency
    pub fn add_dependency(&mut self, task_id: String, dependency_type: DependencyType) {
        self.dependencies.push(TaskDependency {
            task_id,
            dependency_type,
        });
        self.update_timestamp();
    }
    
    /// Remove dependency
    pub fn remove_dependency(&mut self, task_id: &str) -> bool {
        if let Some(pos) = self.dependencies.iter().position(|d| d.task_id == task_id) {
            self.dependencies.remove(pos);
            self.update_timestamp();
            return true;
        }
        false
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
    
    /// Set notes
    pub fn set_notes(&mut self, notes: String) {
        self.notes = notes;
        self.update_timestamp();
    }
    
    /// Set reminder
    pub fn set_reminder(&mut self, reminder: u64) {
        self.reminder = Some(reminder);
        self.update_timestamp();
    }
    
    /// Set recurrence
    pub fn set_recurrence(&mut self, recurrence: TaskRecurrence) {
        self.recurrence = Some(recurrence);
        self.update_timestamp();
    }
    
    /// Clear recurrence
    pub fn clear_recurrence(&mut self) {
        self.recurrence = None;
        self.update_timestamp();
    }
    
    /// Update timestamp
    fn update_timestamp(&mut self) {
        self.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    /// Check if task is pending
    pub fn is_pending(&self) -> bool {
        self.status == TaskStatus::Pending
    }
    
    /// Check if task is in progress
    pub fn is_in_progress(&self) -> bool {
        self.status == TaskStatus::InProgress
    }
    
    /// Check if task is completed
    pub fn is_completed(&self) -> bool {
        self.status == TaskStatus::Completed
    }
    
    /// Check if task is cancelled
    pub fn is_cancelled(&self) -> bool {
        self.status == TaskStatus::Cancelled
    }
    
    /// Check if task is deferred
    pub fn is_deferred(&self) -> bool {
        self.status == TaskStatus::Deferred
    }
    
    /// Check if task is overdue
    pub fn is_overdue(&self) -> bool {
        if let Some(due_date) = self.due_date {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            due_date < now && !self.is_completed()
        } else {
            false
        }
    }
    
    /// Check if task is due today
    pub fn is_due_today(&self) -> bool {
        if let Some(due_date) = self.due_date {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let today_start = now - (now % 86400);
            let today_end = today_start + 86400;
            due_date >= today_start && due_date < today_end
        } else {
            false
        }
    }
    
    /// Check if task is due this week
    pub fn is_due_this_week(&self) -> bool {
        if let Some(due_date) = self.due_date {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let week_start = now - (now % 604800);
            let week_end = week_start + 604800;
            due_date >= week_start && due_date < week_end
        } else {
            false
        }
    }
    
    /// Get task age in seconds
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
    
    /// Get task duration in seconds
    pub fn get_duration_seconds(&self) -> Option<u64> {
        if let (Some(start), Some(completed)) = (self.start_date, self.completed_date) {
            Some(completed - start)
        } else {
            None
        }
    }
    
    /// Get dependency count
    pub fn get_dependency_count(&self) -> usize {
        self.dependencies.len()
    }
    
    /// Get tag count
    pub fn get_tag_count(&self) -> usize {
        self.tags.len()
    }
    
    /// Check if task has dependencies
    pub fn has_dependencies(&self) -> bool {
        !self.dependencies.is_empty()
    }
    
    /// Check if task has tags
    pub fn has_tags(&self) -> bool {
        !self.tags.is_empty()
    }
    
    /// Check if task has due date
    pub fn has_due_date(&self) -> bool {
        self.due_date.is_some()
    }
    
    /// Check if task has recurrence
    pub fn has_recurrence(&self) -> bool {
        self.recurrence.is_some()
    }
}
