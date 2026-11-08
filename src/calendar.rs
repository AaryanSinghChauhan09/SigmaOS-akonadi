//! Calendar management for akonadi.

/// Event type
#[derive(Debug, Clone, PartialEq)]
pub enum EventType {
    /// Meeting event
    Meeting,
    /// Appointment event
    Appointment,
    /// Reminder event
    Reminder,
    /// Holiday event
    Holiday,
    /// Birthday event
    Birthday,
    /// Custom event
    Custom,
}

/// Event status
#[derive(Debug, Clone, PartialEq)]
pub enum EventStatus {
    /// Pending event
    Pending,
    /// Confirmed event
    Confirmed,
    /// Cancelled event
    Cancelled,
    /// Completed event
    Completed,
}

/// Event recurrence
#[derive(Debug, Clone)]
pub struct EventRecurrence {
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

/// Calendar event
#[derive(Debug, Clone)]
pub struct CalendarEvent {
    /// Event ID
    pub id: String,
    /// Event title
    pub title: String,
    /// Event description
    pub description: String,
    /// Event type
    pub event_type: EventType,
    /// Event status
    pub status: EventStatus,
    /// Event start time
    pub start_time: u64,
    /// Event end time
    pub end_time: u64,
    /// Event location
    pub location: Option<String>,
    /// Event attendees
    pub attendees: Vec<String>,
    /// Event organizer
    pub organizer: Option<String>,
    /// Event recurrence
    pub recurrence: Option<EventRecurrence>,
    /// Event created timestamp
    pub created_at: u64,
    /// Event updated timestamp
    pub updated_at: u64,
    /// Event reminders
    pub reminders: Vec<u64>,
    /// Event tags
    pub tags: Vec<String>,
}

impl CalendarEvent {
    /// Create new calendar event
    pub fn new(id: String, title: String, event_type: EventType) -> Self {
        Self {
            id,
            title,
            description: String::new(),
            event_type,
            status: EventStatus::Pending,
            start_time: 0,
            end_time: 0,
            location: None,
            attendees: vec![],
            organizer: None,
            recurrence: None,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            updated_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            reminders: vec![],
            tags: vec![],
        }
    }
    
    /// Set description
    pub fn set_description(&mut self, description: String) {
        self.description = description;
        self.update_timestamp();
    }
    
    /// Set start time
    pub fn set_start_time(&mut self, start_time: u64) {
        self.start_time = start_time;
        self.update_timestamp();
    }
    
    /// Set end time
    pub fn set_end_time(&mut self, end_time: u64) {
        self.end_time = end_time;
        self.update_timestamp();
    }
    
    /// Set location
    pub fn set_location(&mut self, location: String) {
        self.location = Some(location);
        self.update_timestamp();
    }
    
    /// Add attendee
    pub fn add_attendee(&mut self, attendee: String) {
        if !self.attendees.contains(&attendee) {
            self.attendees.push(attendee);
            self.update_timestamp();
        }
    }
    
    /// Remove attendee
    pub fn remove_attendee(&mut self, attendee: &str) -> bool {
        if let Some(pos) = self.attendees.iter().position(|a| a == attendee) {
            self.attendees.remove(pos);
            self.update_timestamp();
            return true;
        }
        false
    }
    
    /// Set organizer
    pub fn set_organizer(&mut self, organizer: String) {
        self.organizer = Some(organizer);
        self.update_timestamp();
    }
    
    /// Set recurrence
    pub fn set_recurrence(&mut self, recurrence: EventRecurrence) {
        self.recurrence = Some(recurrence);
        self.update_timestamp();
    }
    
    /// Clear recurrence
    pub fn clear_recurrence(&mut self) {
        self.recurrence = None;
        self.update_timestamp();
    }
    
    /// Add reminder
    pub fn add_reminder(&mut self, reminder_time: u64) {
        if !self.reminders.contains(&reminder_time) {
            self.reminders.push(reminder_time);
            self.update_timestamp();
        }
    }
    
    /// Remove reminder
    pub fn remove_reminder(&mut self, reminder_time: &u64) -> bool {
        if let Some(pos) = self.reminders.iter().position(|r| r == reminder_time) {
            self.reminders.remove(pos);
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
    
    /// Set status
    pub fn set_status(&mut self, status: EventStatus) {
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
    
    /// Check if event is confirmed
    pub fn is_confirmed(&self) -> bool {
        self.status == EventStatus::Confirmed
    }
    
    /// Check if event is cancelled
    pub fn is_cancelled(&self) -> bool {
        self.status == EventStatus::Cancelled
    }
    
    /// Check if event is completed
    pub fn is_completed(&self) -> bool {
        self.status == EventStatus::Completed
    }
    
    /// Check if event is upcoming
    pub fn is_upcoming(&self) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.start_time > now
    }
    
    /// Check if event is ongoing
    pub fn is_ongoing(&self) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.start_time <= now && now <= self.end_time
    }
    
    /// Check if event is past
    pub fn is_past(&self) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.end_time < now
    }
    
    /// Get event duration in seconds
    pub fn get_duration_seconds(&self) -> u64 {
        self.end_time.saturating_sub(self.start_time)
    }
    
    /// Get event age in seconds
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
    
    /// Get attendee count
    pub fn get_attendee_count(&self) -> usize {
        self.attendees.len()
    }
    
    /// Get reminder count
    pub fn get_reminder_count(&self) -> usize {
        self.reminders.len()
    }
    
    /// Get tag count
    pub fn get_tag_count(&self) -> usize {
        self.tags.len()
    }
    
    /// Check if event has attendees
    pub fn has_attendees(&self) -> bool {
        !self.attendees.is_empty()
    }
    
    /// Check if event has location
    pub fn has_location(&self) -> bool {
        self.location.is_some()
    }
    
    /// Check if event has recurrence
    pub fn has_recurrence(&self) -> bool {
        self.recurrence.is_some()
    }
    
    /// Check if event has reminders
    pub fn has_reminders(&self) -> bool {
        !self.reminders.is_empty()
    }
    
    /// Check if event has tags
    pub fn has_tags(&self) -> bool {
        !self.tags.is_empty()
    }
}

/// Calendar
#[derive(Debug, Clone)]
pub struct Calendar {
    /// Calendar ID
    pub id: String,
    /// Calendar name
    pub name: String,
    /// Calendar description
    pub description: String,
    /// Calendar events
    pub events: Vec<CalendarEvent>,
    /// Calendar created timestamp
    pub created_at: u64,
    /// Calendar updated timestamp
    pub updated_at: u64,
    /// Calendar color
    pub color: String,
    /// Calendar visible
    pub visible: bool,
    /// Calendar readonly
    pub readonly: bool,
}

impl Calendar {
    /// Create new calendar
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            description: String::new(),
            events: vec![],
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
    
    /// Add event
    pub fn add_event(&mut self, event: CalendarEvent) -> String {
        let id = event.id.clone();
        self.events.push(event);
        self.update_timestamp();
        id
    }
    
    /// Remove event
    pub fn remove_event(&mut self, event_id: &str) -> bool {
        if let Some(pos) = self.events.iter().position(|e| e.id == event_id) {
            self.events.remove(pos);
            self.update_timestamp();
            return true;
        }
        false
    }
    
    /// Get event
    pub fn get_event(&self, event_id: &str) -> Option<&CalendarEvent> {
        self.events.iter().find(|e| e.id == event_id)
    }
    
    /// Get event mutably
    pub fn get_event_mut(&mut self, event_id: &str) -> Option<&mut CalendarEvent> {
        self.events.iter_mut().find(|e| e.id == event_id)
    }
    
    /// Get all events
    pub fn get_events(&self) -> &[CalendarEvent] {
        &self.events
    }
    
    /// Get upcoming events
    pub fn get_upcoming_events(&self) -> Vec<&CalendarEvent> {
        self.events.iter()
            .filter(|e| e.is_upcoming())
            .collect()
    }
    
    /// Get past events
    pub fn get_past_events(&self) -> Vec<&CalendarEvent> {
        self.events.iter()
            .filter(|e| e.is_past())
            .collect()
    }
    
    /// Get ongoing events
    pub fn get_ongoing_events(&self) -> Vec<&CalendarEvent> {
        self.events.iter()
            .filter(|e| e.is_ongoing())
            .collect()
    }
    
    /// Get events by type
    pub fn get_events_by_type(&self, event_type: &EventType) -> Vec<&CalendarEvent> {
        self.events.iter()
            .filter(|e| e.event_type == *event_type)
            .collect()
    }
    
    /// Get events by status
    pub fn get_events_by_status(&self, status: &EventStatus) -> Vec<&CalendarEvent> {
        self.events.iter()
            .filter(|e| e.status == *status)
            .collect()
    }
    
    /// Search events
    pub fn search_events(&self, query: &str) -> Vec<&CalendarEvent> {
        let query_lower = query.to_lowercase();
        self.events.iter()
            .filter(|e| e.title.to_lowercase().contains(&query_lower) || 
                       e.description.to_lowercase().contains(&query_lower))
            .collect()
    }
    
    /// Update timestamp
    fn update_timestamp(&mut self) {
        self.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    /// Get event count
    pub fn get_event_count(&self) -> usize {
        self.events.len()
    }
    
    /// Get calendar age in seconds
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
    
    /// Check if calendar is visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }
    
    /// Check if calendar is readonly
    pub fn is_readonly(&self) -> bool {
        self.readonly
    }
}
