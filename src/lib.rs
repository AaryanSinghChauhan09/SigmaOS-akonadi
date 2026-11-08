//! SigmaOS-akonadi — comprehensive PIM (Personal Information Management) framework component.
//!
//! Provides PIM functionality with contacts, calendars, tasks, notes,
//! and address book management.

pub mod contact;
pub mod calendar;
pub mod task;
pub mod note;
pub mod addressbook;
pub mod pim;
pub mod manager;
pub mod options;

pub use contact::{Contact, ContactType, ContactStatus, ContactGroup, ContactRelationship};
pub use calendar::{Calendar, CalendarEvent, EventType, EventStatus, EventRecurrence};
pub use task::{Task, TaskStatus, TaskPriority, TaskCategory, TaskDependency};
pub use note::{Note, NoteType, NoteCategory, NoteTag};
pub use addressbook::{AddressBook, AddressBookEntry, EntryType, EntryStatus};
pub use pim::{PIMItem, PIMType, PIMStatus};
pub use manager::{PIMManager, ManagerOptions, ManagerStatistics};
pub use options::{Mode, Options};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_contact_type() {
        let types = vec![
            ContactType::Person,
            ContactType::Organization,
            ContactType::Group,
            ContactType::Custom,
        ];
        assert_eq!(types.len(), 4);
    }
    
    #[test]
    fn test_contact_status() {
        let statuses = vec![
            ContactStatus::Active,
            ContactStatus::Inactive,
            ContactStatus::Deleted,
        ];
        assert_eq!(statuses.len(), 3);
    }
    
    #[test]
    fn test_event_type() {
        let types = vec![
            EventType::Meeting,
            EventType::Appointment,
            EventType::Reminder,
            EventType::Holiday,
            EventType::Birthday,
            EventType::Custom,
        ];
        assert_eq!(types.len(), 6);
    }
    
    #[test]
    fn test_event_status() {
        let statuses = vec![
            EventStatus::Pending,
            EventStatus::Confirmed,
            EventStatus::Cancelled,
            EventStatus::Completed,
        ];
        assert_eq!(statuses.len(), 4);
    }
    
    #[test]
    fn test_task_status() {
        let statuses = vec![
            TaskStatus::Pending,
            TaskStatus::InProgress,
            TaskStatus::Completed,
            TaskStatus::Cancelled,
            TaskStatus::Deferred,
        ];
        assert_eq!(statuses.len(), 5);
    }
    
    #[test]
    fn test_task_priority() {
        let priorities = vec![
            TaskPriority::Low,
            TaskPriority::Medium,
            TaskPriority::High,
            TaskPriority::Critical,
        ];
        assert_eq!(priorities.len(), 4);
    }
    
    #[test]
    fn test_note_type() {
        let types = vec![
            NoteType::Text,
            NoteType::Checklist,
            NoteType::Drawing,
            NoteType::Voice,
            NoteType::Image,
        ];
        assert_eq!(types.len(), 5);
    }
    
    #[test]
    fn test_entry_type() {
        let types = vec![
            EntryType::Contact,
            EntryType::Organization,
            EntryType::Group,
            EntryType::Location,
            EntryType::Custom,
        ];
        assert_eq!(types.len(), 5);
    }
    
    #[test]
    fn test_contact_creation() {
        let contact = Contact::new(
            "contact-1".into(),
            "John Doe".into(),
            ContactType::Person,
        );
        
        assert_eq!(contact.id, "contact-1");
        assert_eq!(contact.name, "John Doe");
        assert_eq!(contact.contact_type, ContactType::Person);
    }
    
    #[test]
    fn test_calendar_event_creation() {
        let event = CalendarEvent::new(
            "event-1".into(),
            "Team Meeting".into(),
            EventType::Meeting,
        );
        
        assert_eq!(event.id, "event-1");
        assert_eq!(event.title, "Team Meeting");
        assert_eq!(event.event_type, EventType::Meeting);
    }
    
    #[test]
    fn test_task_creation() {
        let task = Task::new(
            "task-1".into(),
            "Complete project".into(),
            TaskPriority::High,
        );
        
        assert_eq!(task.id, "task-1");
        assert_eq!(task.title, "Complete project");
        assert_eq!(task.priority, TaskPriority::High);
    }
    
    #[test]
    fn test_note_creation() {
        let note = Note::new(
            "note-1".into(),
            "Meeting notes".into(),
            NoteType::Text,
        );
        
        assert_eq!(note.id, "note-1");
        assert_eq!(note.title, "Meeting notes");
        assert_eq!(note.note_type, NoteType::Text);
    }
    
    #[test]
    fn test_addressbook_entry_creation() {
        let entry = AddressBookEntry::new(
            "entry-1".into(),
            "John Doe".into(),
            EntryType::Contact,
        );
        
        assert_eq!(entry.id, "entry-1");
        assert_eq!(entry.name, "John Doe");
        assert_eq!(entry.entry_type, EntryType::Contact);
    }
    
    #[test]
    fn test_manager_creation() {
        let mut manager = PIMManager::new();
        assert!(manager.list_contacts().is_empty());
        assert!(manager.list_calendars().is_empty());
        assert!(manager.list_tasks().is_empty());
        assert!(manager.list_notes().is_empty());
    }
    
    #[test]
    fn test_add_contact() {
        let mut manager = PIMManager::new();
        let contact_id = manager.add_contact(
            "John Doe".into(),
            ContactType::Person,
        );
        
        assert!(manager.get_contact(&contact_id).is_some());
        assert_eq!(manager.list_contacts().len(), 1);
    }
    
    #[test]
    fn test_add_calendar_event() {
        let mut manager = PIMManager::new();
        let calendar_id = manager.create_calendar("Work".into());
        
        let event_id = manager.add_calendar_event(
            calendar_id.clone(),
            "Team Meeting".into(),
            EventType::Meeting,
        );
        
        assert!(manager.get_calendar_event(&event_id).is_some());
    }
    
    #[test]
    fn test_add_task() {
        let mut manager = PIMManager::new();
        let task_id = manager.add_task(
            "Complete project".into(),
            TaskPriority::High,
        );
        
        assert!(manager.get_task(&task_id).is_some());
        assert_eq!(manager.list_tasks().len(), 1);
    }
    
    #[test]
    fn test_add_note() {
        let mut manager = PIMManager::new();
        let note_id = manager.add_note(
            "Meeting notes".into(),
            NoteType::Text,
        );
        
        assert!(manager.get_note(&note_id).is_some());
        assert_eq!(manager.list_notes().len(), 1);
    }
    
    #[test]
    fn test_export_import() {
        let mut manager = PIMManager::new();
        
        manager.add_contact("John Doe".into(), ContactType::Person);
        manager.add_task("Complete project".into(), TaskPriority::High);
        
        let export = manager.export_data();
        
        let mut manager2 = PIMManager::new();
        manager2.import_data(export);
        
        assert_eq!(manager2.list_contacts().len(), 1);
        assert_eq!(manager2.list_tasks().len(), 1);
    }
    
    #[test]
    fn test_manager_statistics() {
        let mut manager = PIMManager::new();
        
        manager.add_contact("Contact1".into(), ContactType::Person);
        manager.add_contact("Contact2".into(), ContactType::Organization);
        manager.add_task("Task1".into(), TaskPriority::High);
        manager.add_task("Task2".into(), TaskPriority::Low);
        
        let stats = manager.get_statistics();
        assert_eq!(stats.total_contacts, 2);
        assert_eq!(stats.total_tasks, 2);
        assert_eq!(stats.total_notes, 0);
    }
}
