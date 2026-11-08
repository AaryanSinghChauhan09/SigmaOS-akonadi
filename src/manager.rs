//! PIM manager for akonadi.

use std::collections::HashMap;
use crate::contact::{Contact, ContactType, ContactStatus};
use crate::calendar::{Calendar, CalendarEvent, EventType, EventStatus};
use crate::task::{Task, TaskStatus, TaskPriority, TaskCategory};
use crate::note::{Note, NoteType, NoteCategory};
use crate::addressbook::{AddressBook, AddressBookEntry, EntryType, EntryStatus};
use crate::pim::{PIMItem, PIMType, PIMStatus};

/// Manager options
#[derive(Debug, Clone)]
pub struct ManagerOptions {
    /// Auto-save changes
    pub auto_save: bool,
    /// Auto-backup
    pub auto_backup: bool,
    /// Backup interval in seconds
    pub backup_interval: u64,
    /// Max items per collection
    pub max_items_per_collection: usize,
    /// Enable notifications
    pub enable_notifications: bool,
}

impl Default for ManagerOptions {
    fn default() -> Self {
        Self {
            auto_save: true,
            auto_backup: true,
            backup_interval: 86400,
            max_items_per_collection: 10000,
            enable_notifications: true,
        }
    }
}

/// Manager statistics
#[derive(Debug, Clone)]
pub struct ManagerStatistics {
    /// Total contacts
    pub total_contacts: usize,
    /// Total calendars
    pub total_calendars: usize,
    /// Total tasks
    pub total_tasks: usize,
    /// Total notes
    pub total_notes: usize,
    /// Total address books
    pub total_address_books: usize,
    /// Active contacts
    pub active_contacts: usize,
    /// Completed tasks
    pub completed_tasks: usize,
    /// Pinned notes
    pub pinned_notes: usize,
    /// Manager uptime in seconds
    pub uptime: u64,
}

/// PIM manager
#[derive(Debug, Clone)]
pub struct PIMManager {
    /// Contacts
    contacts: HashMap<String, Contact>,
    /// Calendars
    calendars: HashMap<String, Calendar>,
    /// Tasks
    tasks: HashMap<String, Task>,
    /// Notes
    notes: HashMap<String, Note>,
    /// Address books
    address_books: HashMap<String, AddressBook>,
    /// PIM items
    pim_items: HashMap<String, PIMItem>,
    /// Options
    options: ManagerOptions,
    /// Statistics
    statistics: ManagerStatistics,
    /// Manager created timestamp
    pub created_at: u64,
}

impl Default for PIMManager {
    fn default() -> Self {
        Self {
            contacts: HashMap::new(),
            calendars: HashMap::new(),
            tasks: HashMap::new(),
            notes: HashMap::new(),
            address_books: HashMap::new(),
            pim_items: HashMap::new(),
            options: ManagerOptions::default(),
            statistics: ManagerStatistics {
                total_contacts: 0,
                total_calendars: 0,
                total_tasks: 0,
                total_notes: 0,
                total_address_books: 0,
                active_contacts: 0,
                completed_tasks: 0,
                pinned_notes: 0,
                uptime: 0,
            },
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

impl PIMManager {
    /// Create new PIM manager
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create manager with options
    pub fn with_options(options: ManagerOptions) -> Self {
        Self {
            contacts: HashMap::new(),
            calendars: HashMap::new(),
            tasks: HashMap::new(),
            notes: HashMap::new(),
            address_books: HashMap::new(),
            pim_items: HashMap::new(),
            options,
            statistics: ManagerStatistics {
                total_contacts: 0,
                total_calendars: 0,
                total_tasks: 0,
                total_notes: 0,
                total_address_books: 0,
                active_contacts: 0,
                completed_tasks: 0,
                pinned_notes: 0,
                uptime: 0,
            },
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Initialize manager
    pub fn initialize(&mut self) {
        // Create default calendar
        let default_calendar = Calendar::new("default".into(), "Default".into());
        self.calendars.insert("default".into(), default_calendar);
        
        // Create default address book
        let default_address_book = AddressBook::new("default".into(), "Default".into());
        self.address_books.insert("default".into(), default_address_book);
        
        self.update_statistics();
    }
    
    // Contact methods
    
    /// Add contact
    pub fn add_contact(&mut self, name: String, contact_type: ContactType) -> String {
        let id = format!("contact-{}", self.contacts.len() + 1);
        let contact = Contact::new(id.clone(), name, contact_type);
        
        self.contacts.insert(id.clone(), contact);
        self.update_statistics();
        id
    }
    
    /// Get contact
    pub fn get_contact(&self, contact_id: &str) -> Option<&Contact> {
        self.contacts.get(contact_id)
    }
    
    /// Get contact mutably
    pub fn get_contact_mut(&mut self, contact_id: &str) -> Option<&mut Contact> {
        self.contacts.get_mut(contact_id)
    }
    
    /// Get all contacts
    pub fn list_contacts(&self) -> Vec<&Contact> {
        self.contacts.values().collect()
    }
    
    /// Delete contact
    pub fn delete_contact(&mut self, contact_id: &str) -> bool {
        if self.contacts.remove(contact_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Update contact status
    pub fn update_contact_status(&mut self, contact_id: &str, status: ContactStatus) -> bool {
        if let Some(contact) = self.contacts.get_mut(contact_id) {
            contact.set_status(status);
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Search contacts
    pub fn search_contacts(&self, query: &str) -> Vec<&Contact> {
        let query_lower = query.to_lowercase();
        self.contacts.values()
            .filter(|c| c.name.to_lowercase().contains(&query_lower) ||
                       c.email.as_ref().map(|e| e.to_lowercase().contains(&query_lower)).unwrap_or(false) ||
                       c.phone.as_ref().map(|p| p.to_lowercase().contains(&query_lower)).unwrap_or(false))
            .collect()
    }
    
    /// Get contacts by type
    pub fn get_contacts_by_type(&self, contact_type: &ContactType) -> Vec<&Contact> {
        self.contacts.values()
            .filter(|c| c.contact_type == *contact_type)
            .collect()
    }
    
    /// Get contacts by status
    pub fn get_contacts_by_status(&self, status: &ContactStatus) -> Vec<&Contact> {
        self.contacts.values()
            .filter(|c| c.status == *status)
            .collect()
    }
    
    // Calendar methods
    
    /// Create calendar
    pub fn create_calendar(&mut self, name: String) -> String {
        let id = format!("calendar-{}", self.calendars.len() + 1);
        let calendar = Calendar::new(id.clone(), name);
        
        self.calendars.insert(id.clone(), calendar);
        self.update_statistics();
        id
    }
    
    /// Get calendar
    pub fn get_calendar(&self, calendar_id: &str) -> Option<&Calendar> {
        self.calendars.get(calendar_id)
    }
    
    /// Get calendar mutably
    pub fn get_calendar_mut(&mut self, calendar_id: &str) -> Option<&mut Calendar> {
        self.calendars.get_mut(calendar_id)
    }
    
    /// Get all calendars
    pub fn list_calendars(&self) -> Vec<&Calendar> {
        self.calendars.values().collect()
    }
    
    /// Delete calendar
    pub fn delete_calendar(&mut self, calendar_id: &str) -> bool {
        if self.calendars.remove(calendar_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Add calendar event
    pub fn add_calendar_event(&mut self, calendar_id: String, title: String, event_type: EventType) -> String {
        let event_id = format!("event-{}", self.calendars.len() + self.tasks.len() + self.notes.len() + 1);
        let event = CalendarEvent::new(event_id.clone(), title, event_type);
        
        if let Some(calendar) = self.calendars.get_mut(&calendar_id) {
            calendar.add_event(event);
            self.update_statistics();
            return event_id;
        }
        
        String::new()
    }
    
    /// Get calendar event
    pub fn get_calendar_event(&self, event_id: &str) -> Option<&CalendarEvent> {
        for calendar in self.calendars.values() {
            if let Some(event) = calendar.get_event(event_id) {
                return Some(event);
            }
        }
        None
    }
    
    /// Get calendar event mutably
    pub fn get_calendar_event_mut(&mut self, event_id: &str) -> Option<&mut CalendarEvent> {
        for calendar in self.calendars.values_mut() {
            if let Some(event) = calendar.get_event_mut(event_id) {
                return Some(event);
            }
        }
        None
    }
    
    /// Delete calendar event
    pub fn delete_calendar_event(&mut self, event_id: &str) -> bool {
        for calendar in self.calendars.values_mut() {
            if calendar.remove_event(event_id) {
                self.update_statistics();
                return true;
            }
        }
        false
    }
    
    /// Get all calendar events
    pub fn list_calendar_events(&self) -> Vec<&CalendarEvent> {
        let mut events = vec![];
        for calendar in self.calendars.values() {
            events.extend(calendar.get_events());
        }
        events
    }
    
    /// Get upcoming calendar events
    pub fn get_upcoming_calendar_events(&self) -> Vec<&CalendarEvent> {
        let mut events = vec![];
        for calendar in self.calendars.values() {
            events.extend(calendar.get_upcoming_events());
        }
        events
    }
    
    /// Search calendar events
    pub fn search_calendar_events(&self, query: &str) -> Vec<&CalendarEvent> {
        let mut events = vec![];
        for calendar in self.calendars.values() {
            events.extend(calendar.search_events(query));
        }
        events
    }
    
    // Task methods
    
    /// Add task
    pub fn add_task(&mut self, title: String, priority: TaskPriority) -> String {
        let id = format!("task-{}", self.tasks.len() + 1);
        let task = Task::new(id.clone(), title, priority);
        
        self.tasks.insert(id.clone(), task);
        self.update_statistics();
        id
    }
    
    /// Get task
    pub fn get_task(&self, task_id: &str) -> Option<&Task> {
        self.tasks.get(task_id)
    }
    
    /// Get task mutably
    pub fn get_task_mut(&mut self, task_id: &str) -> Option<&mut Task> {
        self.tasks.get_mut(task_id)
    }
    
    /// Get all tasks
    pub fn list_tasks(&self) -> Vec<&Task> {
        self.tasks.values().collect()
    }
    
    /// Delete task
    pub fn delete_task(&mut self, task_id: &str) -> bool {
        if self.tasks.remove(task_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Update task status
    pub fn update_task_status(&mut self, task_id: &str, status: TaskStatus) -> bool {
        if let Some(task) = self.tasks.get_mut(task_id) {
            task.set_status(status);
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Search tasks
    pub fn search_tasks(&self, query: &str) -> Vec<&Task> {
        let query_lower = query.to_lowercase();
        self.tasks.values()
            .filter(|t| t.title.to_lowercase().contains(&query_lower) ||
                       t.description.to_lowercase().contains(&query_lower))
            .collect()
    }
    
    /// Get tasks by status
    pub fn get_tasks_by_status(&self, status: &TaskStatus) -> Vec<&Task> {
        self.tasks.values()
            .filter(|t| t.status == *status)
            .collect()
    }
    
    /// Get tasks by priority
    pub fn get_tasks_by_priority(&self, priority: &TaskPriority) -> Vec<&Task> {
        self.tasks.values()
            .filter(|t| t.priority == *priority)
            .collect()
    }
    
    /// Get tasks by category
    pub fn get_tasks_by_category(&self, category: &TaskCategory) -> Vec<&Task> {
        self.tasks.values()
            .filter(|t| t.category == *category)
            .collect()
    }
    
    /// Get overdue tasks
    pub fn get_overdue_tasks(&self) -> Vec<&Task> {
        self.tasks.values()
            .filter(|t| t.is_overdue())
            .collect()
    }
    
    /// Get tasks due today
    pub fn get_tasks_due_today(&self) -> Vec<&Task> {
        self.tasks.values()
            .filter(|t| t.is_due_today())
            .collect()
    }
    
    /// Get tasks due this week
    pub fn get_tasks_due_this_week(&self) -> Vec<&Task> {
        self.tasks.values()
            .filter(|t| t.is_due_this_week())
            .collect()
    }
    
    // Note methods
    
    /// Add note
    pub fn add_note(&mut self, title: String, note_type: NoteType) -> String {
        let id = format!("note-{}", self.notes.len() + 1);
        let note = Note::new(id.clone(), title, note_type);
        
        self.notes.insert(id.clone(), note);
        self.update_statistics();
        id
    }
    
    /// Get note
    pub fn get_note(&self, note_id: &str) -> Option<&Note> {
        self.notes.get(note_id)
    }
    
    /// Get note mutably
    pub fn get_note_mut(&mut self, note_id: &str) -> Option<&mut Note> {
        self.notes.get_mut(note_id)
    }
    
    /// Get all notes
    pub fn list_notes(&self) -> Vec<&Note> {
        self.notes.values().collect()
    }
    
    /// Delete note
    pub fn delete_note(&mut self, note_id: &str) -> bool {
        if self.notes.remove(note_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Search notes
    pub fn search_notes(&self, query: &str) -> Vec<&Note> {
        let query_lower = query.to_lowercase();
        self.notes.values()
            .filter(|n| n.title.to_lowercase().contains(&query_lower) ||
                       n.content.to_lowercase().contains(&query_lower))
            .collect()
    }
    
    /// Get notes by type
    pub fn get_notes_by_type(&self, note_type: &NoteType) -> Vec<&Note> {
        self.notes.values()
            .filter(|n| n.note_type == *note_type)
            .collect()
    }
    
    /// Get notes by category
    pub fn get_notes_by_category(&self, category: &NoteCategory) -> Vec<&Note> {
        self.notes.values()
            .filter(|n| n.category == *category)
            .collect()
    }
    
    /// Get pinned notes
    pub fn get_pinned_notes(&self) -> Vec<&Note> {
        self.notes.values()
            .filter(|n| n.is_pinned())
            .collect()
    }
    
    /// Get archived notes
    pub fn get_archived_notes(&self) -> Vec<&Note> {
        self.notes.values()
            .filter(|n| n.is_archived())
            .collect()
    }
    
    // Address book methods
    
    /// Create address book
    pub fn create_address_book(&mut self, name: String) -> String {
        let id = format!("addressbook-{}", self.address_books.len() + 1);
        let address_book = AddressBook::new(id.clone(), name);
        
        self.address_books.insert(id.clone(), address_book);
        self.update_statistics();
        id
    }
    
    /// Get address book
    pub fn get_address_book(&self, address_book_id: &str) -> Option<&AddressBook> {
        self.address_books.get(address_book_id)
    }
    
    /// Get address book mutably
    pub fn get_address_book_mut(&mut self, address_book_id: &str) -> Option<&mut AddressBook> {
        self.address_books.get_mut(address_book_id)
    }
    
    /// Get all address books
    pub fn list_address_books(&self) -> Vec<&AddressBook> {
        self.address_books.values().collect()
    }
    
    /// Delete address book
    pub fn delete_address_book(&mut self, address_book_id: &str) -> bool {
        if self.address_books.remove(address_book_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Add address book entry
    pub fn add_address_book_entry(&mut self, address_book_id: String, name: String, entry_type: EntryType) -> String {
        let entry_id = format!("entry-{}", self.address_books.len() + self.contacts.len() + self.tasks.len() + self.notes.len() + 1);
        let entry = AddressBookEntry::new(entry_id.clone(), name, entry_type);
        
        if let Some(address_book) = self.address_books.get_mut(&address_book_id) {
            address_book.add_entry(entry);
            self.update_statistics();
            return entry_id;
        }
        
        String::new()
    }
    
    /// Get address book entry
    pub fn get_address_book_entry(&self, entry_id: &str) -> Option<&AddressBookEntry> {
        for address_book in self.address_books.values() {
            if let Some(entry) = address_book.get_entry(entry_id) {
                return Some(entry);
            }
        }
        None
    }
    
    /// Get address book entry mutably
    pub fn get_address_book_entry_mut(&mut self, entry_id: &str) -> Option<&mut AddressBookEntry> {
        for address_book in self.address_books.values_mut() {
            if let Some(entry) = address_book.get_entry_mut(entry_id) {
                return Some(entry);
            }
        }
        None
    }
    
    /// Delete address book entry
    pub fn delete_address_book_entry(&mut self, entry_id: &str) -> bool {
        for address_book in self.address_books.values_mut() {
            if address_book.remove_entry(entry_id) {
                self.update_statistics();
                return true;
            }
        }
        false
    }
    
    /// Get all address book entries
    pub fn list_address_book_entries(&self) -> Vec<&AddressBookEntry> {
        let mut entries = vec![];
        for address_book in self.address_books.values() {
            entries.extend(address_book.get_entries());
        }
        entries
    }
    
    /// Search address book entries
    pub fn search_address_book_entries(&self, query: &str) -> Vec<&AddressBookEntry> {
        let mut entries = vec![];
        for address_book in self.address_books.values() {
            entries.extend(address_book.search_entries(query));
        }
        entries
    }
    
    // PIM item methods
    
    /// Add PIM item
    pub fn add_pim_item(&mut self, pim_type: PIMType, name: String) -> String {
        let id = format!("pim-{}", self.pim_items.len() + 1);
        let item = PIMItem::new(id.clone(), pim_type, name);
        
        self.pim_items.insert(id.clone(), item);
        self.update_statistics();
        id
    }
    
    /// Get PIM item
    pub fn get_pim_item(&self, item_id: &str) -> Option<&PIMItem> {
        self.pim_items.get(item_id)
    }
    
    /// Get PIM item mutably
    pub fn get_pim_item_mut(&mut self, item_id: &str) -> Option<&mut PIMItem> {
        self.pim_items.get_mut(item_id)
    }
    
    /// Get all PIM items
    pub fn list_pim_items(&self) -> Vec<&PIMItem> {
        self.pim_items.values().collect()
    }
    
    /// Delete PIM item
    pub fn delete_pim_item(&mut self, item_id: &str) -> bool {
        if self.pim_items.remove(item_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Get manager uptime
    pub fn get_uptime(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.created_at
    }
    
    /// Update statistics
    fn update_statistics(&mut self) {
        self.statistics.total_contacts = self.contacts.len();
        self.statistics.total_calendars = self.calendars.len();
        self.statistics.total_tasks = self.tasks.len();
        self.statistics.total_notes = self.notes.len();
        self.statistics.total_address_books = self.address_books.len();
        
        self.statistics.active_contacts = self.contacts.values()
            .filter(|c| c.is_active())
            .count();
        
        self.statistics.completed_tasks = self.tasks.values()
            .filter(|t| t.is_completed())
            .count();
        
        self.statistics.pinned_notes = self.notes.values()
            .filter(|n| n.is_pinned())
            .count();
        
        self.statistics.uptime = self.get_uptime();
    }
    
    /// Get statistics
    pub fn get_statistics(&self) -> &ManagerStatistics {
        &self.statistics
    }
    
    /// Export manager data
    pub fn export_data(&self) -> ManagerExport {
        ManagerExport {
            contacts: self.contacts.values().cloned().collect(),
            calendars: self.calendars.values().cloned().collect(),
            tasks: self.tasks.values().cloned().collect(),
            notes: self.notes.values().cloned().collect(),
            address_books: self.address_books.values().cloned().collect(),
            pim_items: self.pim_items.values().cloned().collect(),
        }
    }
    
    /// Import manager data
    pub fn import_data(&mut self, export: ManagerExport) {
        self.contacts.clear();
        for contact in export.contacts {
            self.contacts.insert(contact.id.clone(), contact);
        }
        
        self.calendars.clear();
        for calendar in export.calendars {
            self.calendars.insert(calendar.id.clone(), calendar);
        }
        
        self.tasks.clear();
        for task in export.tasks {
            self.tasks.insert(task.id.clone(), task);
        }
        
        self.notes.clear();
        for note in export.notes {
            self.notes.insert(note.id.clone(), note);
        }
        
        self.address_books.clear();
        for address_book in export.address_books {
            self.address_books.insert(address_book.id.clone(), address_book);
        }
        
        self.pim_items.clear();
        for item in export.pim_items {
            self.pim_items.insert(item.id.clone(), item);
        }
        
        self.update_statistics();
    }
}

/// Manager export data
#[derive(Debug, Clone)]
pub struct ManagerExport {
    /// Contacts
    pub contacts: Vec<Contact>,
    /// Calendars
    pub calendars: Vec<Calendar>,
    /// Tasks
    pub tasks: Vec<Task>,
    /// Notes
    pub notes: Vec<Note>,
    /// Address books
    pub address_books: Vec<AddressBook>,
    /// PIM items
    pub pim_items: Vec<PIMItem>,
}
