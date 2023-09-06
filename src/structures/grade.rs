use std::default;

use crate::{structures::base::Fields, utils::Snowflake};
use nanoid::nanoid;
use sqlx::FromRow;



#[derive(FromRow, Debug, Default)]
pub struct Grade {
	pub id: Snowflake,
    pub user_id: Snowflake,
	pub grade: i8,
	pub paper: String,
}
impl Grade {
    pub fn new(
        user_id: Snowflake,
		grade: i8,
    ) -> Self {
        Grade {
            user_id,
			..Default::default()
        }
    }
}
use super::base::Base;
impl Base for Grade {
    fn fields(&self) -> Fields {
        let mut fields = Fields::default();
        fields.add("id", &self.id);
        fields.add("user_id", &self.user_id);
        fields.add("grade", &self.grade);
        fields.add("paper", &self.paper);
        return fields;
    }

    fn table_name() -> &'static str {
        "grades"
    }
}
