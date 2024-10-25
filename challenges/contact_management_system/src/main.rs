use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Contact {
    name: String,
    email: String,
    phone_numbers: Vec<String>,
    tags: Vec<String>,
}

struct ContactManager {
    contacts: HashMap<String, Contact>,
    tags_index: HashMap<String, Vec<String>>,

}

impl Contact {
    fn new(name: String, email: String) -> Self {
        Self {
            name,
            email,
            phone_numbers: Vec::new(),
            tags: Vec::new(),
        }
    }

    fn add_phone(&mut self, phone: String) {
        // TODO: Add a phone number if it doesn't exist
        if !self.phone_numbers.contains(&phone) {
            self.phone_numbers.push(phone)
        }
    }

    fn add_tag(&mut self, tag: String) {
        // TODO: Add a tag if it doesn't exist
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }
}

impl ContactManager {
    fn new() -> Self {
        Self {
            contacts: HashMap::new(),
            tags_index: HashMap::new(),
        }
    }

    fn add_contact(&mut self, contact: Contact) -> Result<(), String> {
        // TODO: Add a new contact
        // If contact with same name exists, return Err
        // Don't forget to update tags_index
        match self.contacts.entry(contact.name.clone()) {
            Entry::Occupied(_) => Err("Contact already exists".to_string()),
            Entry::Vacant(entry) => {
                // update tags_index
                for tag in &contact.tags {
                    self.tags_index
                        .entry(tag.clone())
                        .or_default()
                        .push(contact.name.clone())
                }
                entry.insert(contact);
                Ok(())
            }
        }
    }

    fn remove_contact(&mut self, name: &str) -> Result<Contact, String> {
        // TODO: Remove and return the contact
        // Don't forget to update tags_index
        match self.contacts.remove(name) {
            None => Err("Contact does not exists".to_string()),
            Some(contact) => {
                for tag in &contact.tags {
                    self.tags_index
                        .entry(tag.clone())
                        .and_modify(|contacts| contacts.retain(|n| n != name));

                    // Optionally: Remove empty tag entries
                    if let Some(contacts) = self.tags_index.get(tag) {
                        if contacts.is_empty() {
                            self.tags_index.remove(tag);
                        }
                    }
                }
                Ok(contact)
            }
        }
    }

    fn get_contacts_by_tag(&self, tag: &str) -> Vec<&Contact> {
        // TODO: Return all contacts that have the specified tag
        self.tags_index.get(tag)
            .map_or(Vec::new(), |names| {
                names.iter()
                    .filter_map(|name| self.contacts.get(name))
                    .collect()
            })
    }

    fn update_email(&mut self, name: &str, new_email: String) -> Result<(), String> {
        // TODO: Update contact's email
        match self.contacts.get_mut(name) {
            None => Err("Contact does not exist".to_string()),
            Some(contact) => {
                contact.email = new_email;
                Ok(())
            }
        }
    }

    fn merge_contacts(&mut self, name1: &str, name2: &str) -> Result<(), String> {
        // TODO: Merge two contacts (combine phone numbers and tags, keep first email)
        // Remove second contact after merging
        if !self.contacts.contains_key(name1) {
            return Err("Contact 1 does not exist".to_string());
        }
        if !self.contacts.contains_key(name2) {
            return Err("Contact 2 does not exist".to_string());
        }

        let contact2 = self.contacts.get(name2).unwrap().clone();

        if let Some(contact1) = self.contacts.get_mut(name1) {
            for phone in contact2.phone_numbers {
                contact1.add_phone(phone);
            }

            for tag in &contact2.tags {
                contact1.add_tag(tag.clone());
                if let Some(contacts) = self.tags_index.get_mut(tag) {
                    contacts.retain(|n| n != name2);
                    if !contacts.contains(&name1.to_string()) {
                        contacts.push(name1.to_string());
                    }
                }
            }
        }

        self.remove_contact(name2)?;

        Ok(())
    }

}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_contact() -> Contact {
        let mut contact = Contact::new(
            "John Doe".to_string(),
            "john@example.com".to_string(),
        );
        contact.add_phone("123-456-7890".to_string());
        contact.add_tag("friend".to_string());
        contact
    }

    #[test]
    fn test_add_contact() {
        let mut manager = ContactManager::new();
        let contact = create_test_contact();
        assert!(manager.add_contact(contact.clone()).is_ok());
        assert!(manager.add_contact(contact).is_err()); // Should fail on duplicate
    }

    #[test]
    fn test_get_by_tag() {
        let mut manager = ContactManager::new();
        let contact = create_test_contact();
        manager.add_contact(contact).unwrap();

        let contacts = manager.get_contacts_by_tag("friend");
        assert_eq!(contacts.len(), 1);
        assert_eq!(contacts[0].name, "John Doe");
    }

    #[test]
    fn test_merge_contacts() {
        let mut manager = ContactManager::new();

        let mut contact1 = Contact::new(
            "John Doe".to_string(),
            "john@example.com".to_string(),
        );
        contact1.add_phone("123-456-7890".to_string());
        contact1.add_tag("friend".to_string());

        let mut contact2 = Contact::new(
            "Johnny D".to_string(),
            "johnny@example.com".to_string(),
        );
        contact2.add_phone("987-654-3210".to_string());
        contact2.add_tag("work".to_string());

        manager.add_contact(contact1).unwrap();
        manager.add_contact(contact2).unwrap();

        manager.merge_contacts("John Doe", "Johnny D").unwrap();

        let merged = manager.contacts.get("John Doe").unwrap();
        assert_eq!(merged.phone_numbers.len(), 2);
        assert_eq!(merged.tags.len(), 2);
    }

    #[test]
    fn test_update_email() {
        let mut manager = ContactManager::new();

        // Setup: Create and add a contact
        let mut contact = Contact::new(
            "John Doe".to_string(),
            "john@old.com".to_string(),
        );
        contact.add_tag("work".to_string());
        contact.add_phone("123-456-7890".to_string());
        manager.add_contact(contact).unwrap();

        // Test Cases:

        // 1. Successful update
        let result = manager.update_email("John Doe", "john@new.com".to_string());
        assert!(result.is_ok());

        // Verify email was updated
        let updated_contact = manager.contacts.get("John Doe").unwrap();
        assert_eq!(updated_contact.email, "john@new.com");
        // Verify other fields weren't affected
        assert!(updated_contact.tags.contains(&"work".to_string()));
        assert!(updated_contact.phone_numbers.contains(&"123-456-7890".to_string()));

        // 2. Non-existent contact
        let result = manager.update_email("Not Found", "test@test.com".to_string());
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Contact does not exist".to_string()
        );
    }
}