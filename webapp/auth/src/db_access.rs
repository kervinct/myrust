use serde_derive::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct Person {
    pub id: u32,
    pub name: String,
}

#[derive(Serialize, Clone, Copy, PartialEq, Debug)]
pub enum DbPrivilege {
    CanRead,
    CanWrite,
}

#[derive(Serialize, Clone, Debug)]
pub struct User {
    pub username: String,
    pub password: String,
    pub privileges: Vec<DbPrivilege>,
}

pub struct DbConnection {
    persons: Vec<Person>,
    users: Vec<User>,
}

impl DbConnection {
    pub fn new() -> DbConnection {
        use DbPrivilege::*;
        DbConnection {
            persons: vec![],
            users: vec![
                User {
                    username: "admin".to_string(),
                    password: "admin".to_string(),
                    privileges: vec![CanRead, CanWrite],
                },
                User {
                    username: "reader".to_string(),
                    password: "xreader".to_string(),
                    privileges: vec![CanRead],
                },
            ],
        }
    }

    pub fn get_user_by_username(&self, username: &str) -> Option<&User> {
        self.users.iter().find(|u| u.username == username)
    }

    pub fn get_person_by_id(&self, id: u32) -> Option<&Person> {
        self.persons.iter().find(|p| p.id == id)
    }

    pub fn get_persons_by_partial_name<'a>(
        &'a self,
        subname: &'a str,
    ) -> impl Iterator<Item = &Person> + 'a {
        self.persons
            .iter()
            .filter(move |p| p.name.contains(subname))
    }

    pub fn delete_by_id(&mut self, id: u32) -> bool {
        let len = self.persons.len();
        self.persons.retain(|p| p.id != id);
        len != self.persons.len()
    }

    pub fn insert_person(&mut self, mut person: Person) -> u32 {
        let new_id = if self.persons.is_empty() {
            1
        } else {
            self.persons[self.persons.len() - 1].id + 1
        };

        person.id = new_id;
        self.persons.push(person);
        new_id
    }

    pub fn update_person(&mut self, person: Person) -> bool {
        if let Some((n, _)) = self
            .persons
            .iter()
            .enumerate()
            .find(|(_, p)| p.id == person.id)
        {
            self.persons[n] = person;
            true
        } else {
            false
        }
    }
}
