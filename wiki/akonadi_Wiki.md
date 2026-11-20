# SigmaOS-akonadi Component Wiki

## Overview
SigmaOS-akonadi provides Akonadi resource and PIM (Personal Information Management) support for the SigmaOS operating system. It demonstrates the PIM subsystem and resource management within the SigmaOS ecosystem, following the principles of clear abstraction and deterministic PIM initialization.

## Principles
- Single Responsibility
- PIM Abstraction
- Fail-Fast on PIM Initialization Errors
- Deterministic PIM Setup
- Clear Abstraction Over PIM APIs
- Composition over Inheritance for PIM extensions

## Architecture
SigmaOS-akonadi follows a PIM pattern:
- **PIM Core**: Provides resource management, contact handling, and calendar integration
- **Akonadi Integration**: Supports Akonadi resource collection, item management, and protocol handling
- **Resource Management**: Supports contact management, calendar events, and task tracking
- **Error Handling**: Fail-fast on invalid PIM configurations or unsupported resource types

## API
The component provides the following public APIs:
- `pim::resource::add(resource)` - Add a PIM resource (contact, calendar event, task)
- `pim::resource::list()` - List all PIM resources
- `pim::resource::get(id)` - Get a PIM resource by ID
- `pim::status::resources()` - Get PIM resource status and resource count
- Error types for PIM failure scenarios or unsupported resource types

## Test Structure (Arrange-Act-ASSERT)
All tests follow the AAA pattern:
1. **Arrange**: Set up test environment with specific PIM configuration
2. **Act**: Execute the code under test (resource addition, resource listing)
3. **Assert**: Verify the expected PIM results

Example:
```rust
#[test]
fn test_pim_resource_aaa() {
    // Arrange
    let resource = PimResource::new("contact", "John Doe", "john@example.com");
    
    // Act
    let result = pim::resource::add(resource);
    
    // Assert
    assert!(result.is_ok());
    let added = result.unwrap();
    assert!(added.is_valid());
}
```

## Development
- Follows Rust best practices with `#![forbid(unsafe_code)]`
- PIM initialization is best-effort; gracefully degrades on unsupported configurations
- Comprehensive error types for all PIM failure scenarios
- Unit tests provided for resource management and resource listing
- Integration tests for full PIM system with actual PIM resource integration

## TODO
- Add support for additional resource types (email, notes, attachments)
- Expand contact management capabilities (groups, organizations, contact history)
- Add calendar integration features (event creation, recurrence, reminders)
- Create integration tests with actual PIM resource systems
- Implement resource persistence and PIM state management