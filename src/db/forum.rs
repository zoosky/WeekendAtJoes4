use schema::forums;
use error::*;
use db::Conn;
use db::Retrievable;
use db::Creatable;
use db::Deletable;
use db::CRD;
use std::ops::Deref;
use diesel;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::result::Error;
use diesel::ExpressionMethods;

#[derive(Debug, Clone, Identifiable, Queryable, Crd)]
#[insertable = "NewForum"]
#[table_name = "forums"]
pub struct Forum {
    /// Primary Key.
    pub id: i32,
    /// Displayed title of the forum
    pub title: String,
    /// The description that informs users what topics should be discussed in the forum.
    pub description: String,
}

#[derive(Insertable, Debug)]
#[table_name = "forums"]
pub struct NewForum {
    pub title: String,
    pub description: String,
}

impl Forum {
    /// Gets a list of all forums.
    pub fn get_forums(conn: &Conn) -> Result<Vec<Forum>, WeekendAtJoesError> {
        use schema::forums::dsl::*;
        forums
            .load::<Forum>(conn.deref())
            .map_err(Forum::handle_error)
    }
}



impl ErrorFormatter for Forum {
    fn handle_error(diesel_error: Error) -> WeekendAtJoesError {
        handle_diesel_error(diesel_error, "Forum")
    }
}
