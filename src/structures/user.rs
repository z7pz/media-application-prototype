
use crate::{structures::base::Fields, utils::snowflake::Snowflake};
use sqlx::FromRow;

#[derive(sqlx::Type, Debug, Default, PartialEq, Serialize)]
#[sqlx(type_name = "roles_enum", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserRoles {
    #[default]
    Student,
    Teacher,
    Admin,
}

#[derive(FromRow, Debug, Default, Serialize)]
pub struct User {
    pub id: Snowflake,
    pub username: String,
    pub display_name: String,
    pub password_hash: String,
    pub role: UserRoles,
}
impl User {
    pub fn new<T: Into<String>, P: Into<String>, C: Into<String>>(
        display_name: C,
        username: T,
        password: P,
    ) -> Self {
        User {
            username: username.into(),
            password_hash: password.into(),
            display_name: display_name.into(),
            ..Default::default()
        }
    }
}
use super::base::Base;
impl Base for User {
    fn fields(&self) -> Fields {
        let mut fields = Fields::default();
        fields.add("id", &self.id);
        fields.add("display_name", &self.display_name);
        fields.add("username", &self.username);
        fields.add("password_hash", &self.password_hash);
        fields.add("role", &self.role);
        return fields;
    }

    fn table_name() -> &'static str {
        "users"
    }
    fn id(&self) -> Snowflake {
        self.id
    }
}
