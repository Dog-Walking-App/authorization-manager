use diesel::prelude::*;
use crate::utils::password::hash_password;
use crate::utils::service_error::ServiceError;
use super::model::{User, NewUser};
use super::db::{UsersDB, UsersDBTrait};

pub trait UsersServiceTrait {
    fn create_user(&mut self, new_user: &NewUser) -> Result<User, ServiceError>;
    fn get_user_by_id(&mut self, user_id: i32) -> Result<User, ServiceError>;
    fn get_all_users(&mut self) -> Result<Vec<User>, ServiceError>;
    fn update_user(&mut self, user_id: i32, updated_user: &NewUser) -> Result<User, ServiceError>;
    fn delete_user(&mut self, user_id: i32) -> Result<User, ServiceError>;
}

pub struct UsersService<'a> {
    users_db: Box<dyn UsersDBTrait + 'a>,
}

impl<'a> UsersService<'a> {
    pub fn new(connection: &'a mut PgConnection) -> UsersService<'a> {
        let users_db = Box::new(UsersDB::new(connection));
        UsersService { users_db }
    }

    fn hash_password_in_user(&mut self, user: &NewUser) -> NewUser {
        let hashed_password: String = hash_password(&user.password)
            .expect("Failed to hash password");
        let new_user = NewUser {
            username: user.username.to_owned(),
            password: hashed_password,
        };
        new_user
    }
}

impl<'a> UsersServiceTrait for UsersService<'a> {
    fn create_user(&mut self, new_user: &NewUser) -> Result<User, ServiceError> {
        let user = self.hash_password_in_user(new_user);
        self.users_db.insert(&user)
            .map_err(|_| ServiceError::new("Failed to create user"))
    }

    fn get_user_by_id(&mut self, user_id: i32) -> Result<User, ServiceError> {
        self.users_db.find_by_id(user_id)
            .map_err(|_| ServiceError::new("Failed to get user"))
    }

    fn get_all_users(&mut self) -> Result<Vec<User>, ServiceError> {
        self.users_db.find_all()
           .map_err(|_| ServiceError::new("Failed to get users"))
    }

    fn update_user(&mut self, user_id: i32, updated_user: &NewUser) -> Result<User, ServiceError> {
        let user = self.hash_password_in_user(updated_user);
        self.users_db.update(user_id, &user)
           .map_err(|_| ServiceError::new("Failed to update user"))
    }

    fn delete_user(&mut self, user_id: i32) -> Result<User, ServiceError> {
        self.users_db.delete(user_id)
            .map_err(|_| ServiceError::new("Failed to delete user"))
    }
}
