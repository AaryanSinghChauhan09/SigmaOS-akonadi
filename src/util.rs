//! akonadi model — contacts/address book.

#[derive(Debug, Clone)]
pub struct Contact {
    pub id: usize,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub org: String,
}

#[derive(Debug, Clone)]
pub struct AddressBook {
    pub name: String,
    pub contacts: Vec<Contact>,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub books: Vec<AddressBook>,
    next_id: usize,
}

impl Default for Model {
    fn default() -> Self { Self { books: vec![], next_id: 1 } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn create_book(&mut self, name: &str) -> usize {
        let id = self.books.len();
        self.books.push(AddressBook { name: name.into(), contacts: vec![] });
        id
    }

    pub fn add_contact(&mut self, book_id: usize, name: &str, email: &str, phone: &str, org: &str) -> Option<usize> {
        let b = self.books.get_mut(book_id)?;
        let id = self.next_id;
        self.next_id += 1;
        b.contacts.push(Contact { id, name: name.into(), email: email.into(), phone: phone.into(), org: org.into() });
        Some(id)
    }

    pub fn find(&self, book_id: usize, name: &str) -> Option<&Contact> {
        self.books.get(book_id)?.contacts.iter().find(|c| c.name == name)
    }

    pub fn remove(&mut self, book_id: usize, contact_id: usize) -> bool {
        if let Some(b) = self.books.get_mut(book_id) {
            let before = b.contacts.len();
            b.contacts.retain(|c| c.id != contact_id);
            return b.contacts.len() < before;
        }
        false
    }

    pub fn list(&self, book_id: usize) -> Option<&[Contact]> {
        self.books.get(book_id).map(|b| b.contacts.as_slice())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn book_contact_crud() {
        let mut m = Model::new();
        let bid = m.create_book("personal");
        let cid = m.add_contact(bid, "Alice", "a@x.com", "123", "Acme").unwrap();
        assert!(m.find(bid, "Alice").is_some());
        assert!(m.remove(bid, cid));
    }
}
