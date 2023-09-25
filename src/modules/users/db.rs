use diesel::prelude::*;
use crate::schema::users::dsl::*;
use super::model::{User, NewUser};

pub trait UsersDBTrait {
    fn insert(&mut self, new_user: &NewUser) -> QueryResult<User>;
    fn find_by_id(&mut self, user_id: i32) -> QueryResult<User>;
    fn find_all(&mut self) -> QueryResult<Vec<User>>;
    fn update(&mut self, user_id: i32, user: &NewUser) -> QueryResult<User>;
    fn delete(&mut self, user_id: i32) -> QueryResult<User>;
}

pub struct UsersDB<'a> {
    connection: &'a mut PgConnection,
}

impl<'a> UsersDB<'a> {
    pub fn new(connection: &'a mut PgConnection) -> UsersDB<'a> {
        UsersDB { connection }
    }
}

impl<'a> UsersDBTrait for UsersDB<'a> {
    fn insert(
        &mut self,
        new_user: &NewUser,
    ) -> QueryResult<User> {
        diesel::insert_into(users)
            .values(new_user)
            .get_result(self.connection)
    }
    
    fn find_by_id(
        &mut self,
        user_id: i32,
    ) -> QueryResult<User> {
        users.find(user_id).get_result(self.connection)
    }
    
    fn find_all(&mut self) -> QueryResult<Vec<User>> {
        users.load::<User>(self.connection)
    }
    
    fn update(
        &mut self,
        user_id: i32,
        user: &NewUser,
    ) -> QueryResult<User> {
        diesel::update(users.find(user_id))
            .set(user)
            .get_result(self.connection)
    }
    
    fn delete(
        &mut self,
        user_id: i32,
    ) -> QueryResult<User> {
        diesel::delete(users.find(user_id)).get_result(self.connection)
    }
}
