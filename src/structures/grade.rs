use std::default;

use crate::{structures::base::Fields, utils::snowflake::Snowflake};
use nanoid::nanoid;
use sqlx::FromRow;

#[derive(FromRow, Debug, Default, Serialize, Deserialize, Clone)]
pub struct Grade {
    pub id: Snowflake,
    pub user_id: Snowflake,
    pub exam_id: Snowflake,
    pub grade: i32,
    pub paper: String,
}
impl Grade {
    pub fn new(user_id: Snowflake, exam_id: Snowflake, grade: i32) -> Self {
        Self {
            user_id,
            grade,
            exam_id,
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
        fields.add("exam_id", &self.exam_id);
        fields.add("grade", &self.grade);
        fields.add("paper", &self.paper);
        return fields;
    }

    fn table_name() -> &'static str {
        "grades"
    }
    fn id(&self) -> Snowflake {
        self.id
    }
}
