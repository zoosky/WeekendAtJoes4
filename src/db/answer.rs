use schema::answers;
use db::user::User;
use db::question::Question;
use db::Retrievable;
use db::Creatable;
use db::Deletable;
use db::CRD;
use db::Conn;
use error::*;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel;
use diesel::result::Error;
use std::ops::Deref;
use diesel::ExpressionMethods;



#[derive(Debug, Clone, Identifiable, Queryable, Associations, Crd)]
#[table_name = "answers"]
#[insertable = "NewAnswer"]
#[belongs_to(User, foreign_key = "author_id")]
#[belongs_to(Question, foreign_key = "question_id")]
pub struct Answer {
    /// Primary Key.
    pub id: i32,
    pub question_id: i32,
    pub author_id: i32,
    pub answer_text: Option<String>,
}

#[derive(Insertable, Debug)]
#[table_name = "answers"]
pub struct NewAnswer {
    pub author_id: i32,
    pub question_id: i32,
    pub answer_text: Option<String>,
}

pub struct AnswerData {
    pub answer: Answer,
    pub user: User,
}


impl ErrorFormatter for Answer {
    fn handle_error(diesel_error: Error) -> WeekendAtJoesError {
        handle_diesel_error(diesel_error, "Answer")
    }
}