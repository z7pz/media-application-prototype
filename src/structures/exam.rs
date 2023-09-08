use std::default;

use crate::{structures::base::Fields, utils::Snowflake};
use nanoid::nanoid;
use sqlx::FromRow;

#[derive(FromRow, Debug, Default, Serialize)]
pub struct Exam {
    pub id: Snowflake,
    pub name: String,
    pub outof: i32,
    pub grades: Vec<Snowflake>,
}
impl Exam {
    pub fn new(name: String, outof: i32) -> Self {
        Exam {
            name,
            outof,
            ..Default::default()
        }
    }
}
use super::base::Base;
impl Base for Exam {
    fn fields(&self) -> Fields {
        let mut fields = Fields::default();
        fields.add("id", &self.id);
        fields.add("name", &self.name);
        fields.add("outof", &self.outof);
        fields.add("grades", &self.grades);
        return fields;
    }   

    fn table_name() -> &'static str {
        "exams"
    }

    fn id(&self) -> Snowflake {
        self.id
    }
}
