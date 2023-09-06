use std::default;

use crate::{structures::base::Fields, utils::Snowflake};
use nanoid::nanoid;
use sqlx::FromRow;
#[derive(FromRow, Debug)]
pub struct Session {
	pub id: Snowflake,
    pub user_id: Snowflake,
    pub token: String
}
impl Session {
    pub fn new(
        user_id: Snowflake,
    ) -> Self {
        Session {
            id: Snowflake::generate(),
            token: nanoid!(64),
            user_id,
        }
    }
}
use super::base::Base;
impl Base for Session {
    fn fields(&self) -> Fields {
        let mut fields = Fields::default();
        fields.add("id", &self.id);
        fields.add("user_id", &self.user_id);
        fields.add("token", &self.token);
        return fields;
    }

    fn table_name() -> &'static str {
        "sessions"
    }
}
