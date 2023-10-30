use std::sync::{Arc, Mutex};
use diesel::prelude::*;
use crate::schema::users::dsl::*;
use super::model::{User, NewUser};

pub trait UsersDBTrait {
    fn insert(&mut self, new_user: &NewUser) -> QueryResult<User>;
    fn find_by_id(&mut self, user_id: i32) -> QueryResult<User>;
    fn find_by_username(&mut self, username: &str) -> QueryResult<User>;
    fn find_all(&mut self) -> QueryResult<Vec<User>>;
    fn update(&mut self, user_id: i32, user: &NewUser) -> QueryResult<User>;
    fn delete(&mut self, user_id: i32) -> QueryResult<User>;
}

pub struct UsersDB {
    connection: Arc<Mutex<PgConnection>>,
}

impl UsersDB {
    pub fn new(connection: Arc<Mutex<PgConnection>>) -> UsersDB {
        UsersDB { connection }
    }
}

impl UsersDBTrait for UsersDB {
    fn insert(
        &mut self,
        new_user: &NewUser,
    ) -> QueryResult<User> {
        let connection = &mut *self.connection.lock().unwrap();
        diesel::insert_into(users)
            .values(new_user)
            .get_result(connection)
    }
    
    fn find_by_id(
        &mut self,
        user_id: i32,
    ) -> QueryResult<User> {
        let connection = &mut *self.connection.lock().unwrap();
        users.find(user_id).get_result(connection)
    }

    fn find_by_username(
        &mut self,
        value: &str,
    ) -> QueryResult<User> {
        let connection = &mut *self.connection.lock().unwrap();
        users
            .filter(username.eq(value))
            .load::<User>(connection)?
            .pop()
            .ok_or(diesel::result::Error::NotFound)
    }
    
    fn find_all(&mut self) -> QueryResult<Vec<User>> {
        let connection = &mut *self.connection.lock().unwrap();
        users.load::<User>(connection)
    }
    
    fn update(
        &mut self,
        user_id: i32,
        user: &NewUser,
    ) -> QueryResult<User> {
        let connection = &mut *self.connection.lock().unwrap();
        diesel::update(users.find(user_id))
            .set(user)
            .get_result(connection)
    }
    
    fn delete(
        &mut self,
        user_id: i32,
    ) -> QueryResult<User> {
        let connection = &mut *self.connection.lock().unwrap();
        diesel::delete(users.find(user_id)).get_result(connection)
    }
}
