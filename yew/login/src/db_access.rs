#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DbPrivilege {
    CanRead,
    CanWrite,
}

#[derive(Clone, Debug, PartialEq)]
pub struct User {
    pub username: String,
    pub password: String,
    pub privileges: Vec<DbPrivilege>,
}

#[derive(PartialEq, Clone)]
pub struct DbConnection {
    users: Vec<User>,
}

impl DbConnection {
    pub fn new() -> DbConnection {
        DbConnection {
            users: vec![
                User {
                    username: "admin".to_string(),
                    password: "admin".to_string(),
                    privileges: vec![DbPrivilege::CanRead, DbPrivilege::CanWrite],
                },
                User {
                    username: "reader".to_string(),
                    password: "reader".to_string(),
                    privileges: vec![DbPrivilege::CanRead],
                },
            ],
        }
    }

    pub fn get_user_by_username(&self, username: &str) -> Option<&User> {
        self.users.iter().find(|u| u.username == username)
    }
}
