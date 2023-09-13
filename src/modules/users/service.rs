use super::model::{User, NewUser};
use crate::schema::users::dsl::*;
use diesel::prelude::*;
use crate::utils::password::hash_password;

pub trait UsersServiceTrait {
  fn create_user(&mut self, new_user: &NewUser) -> QueryResult<User>;
  fn get_user_by_id(&mut self, user_id: i32) -> QueryResult<User>;
  fn get_all_users(&mut self) -> QueryResult<Vec<User>>;
  fn update_user(&mut self, user_id: i32, updated_user: &NewUser) -> QueryResult<User>;
  fn delete_user(&mut self, user_id: i32) -> QueryResult<User>;
}

pub struct UsersService<'a> {
  connection: &'a mut PgConnection,
}

impl<'a> UsersService<'a> {
  pub fn new(connection: &'a mut PgConnection) -> UsersService<'a> {
    UsersService { connection }
  }

  fn hash_password_in_user(&mut self, user: &NewUser) -> NewUser {
    let hashed_password: String = hash_password(&user.password).expect("Failed to hash password");
    let new_user = NewUser {
      username: user.username.to_owned(),
      password: hashed_password,
    };
    new_user
  }
}

impl<'a> UsersServiceTrait for UsersService<'a> {
  fn create_user(&mut self, new_user: &NewUser) -> QueryResult<User> {
    let user = self.hash_password_in_user(new_user);
    
    diesel::insert_into(users)
      .values(&user)
      .get_result(self.connection)
  }

  fn get_user_by_id(&mut self, user_id: i32) -> QueryResult<User> {
    users.find(user_id).get_result(self.connection)
  }

  fn get_all_users(&mut self) -> QueryResult<Vec<User>> {
    users.load::<User>(self.connection)
  }

  fn update_user(&mut self, user_id: i32, updated_user: &NewUser) -> QueryResult<User> {
    let user = self.hash_password_in_user(updated_user);

    diesel::update(users.find(user_id))
      .set(&user)
      .get_result(self.connection)
  }

  fn delete_user(&mut self, user_id: i32) -> QueryResult<User> {
    diesel::delete(users.find(user_id)).get_result(self.connection)
  }
}
