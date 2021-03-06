use crate::schema::articles;
use diesel;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::ExpressionMethods;
use chrono::{NaiveDateTime, Utc};
use crate::user::User;
use diesel::BelongingToDsl;
use error::BackendResult;
use diesel::PgConnection;
use uuid::Uuid;
use identifiers::article::ArticleUuid;
use identifiers::user::UserUuid;
use crate::calls::prelude::*;
use crate::schema;


/// The database's representation of an article
#[derive(Clone, Queryable, Identifiable, Associations, Debug, PartialEq, TypeName)]
#[primary_key(uuid)]
#[belongs_to(User, foreign_key = "author_uuid")]
#[table_name = "articles"]
pub struct Article {
    /// The public key for the article.
    pub uuid: Uuid,
    /// The key of the user that authored the article.
    pub author_uuid: Uuid,
    /// The title of the article. This will be used to when showing the article in "Suggested Articles" panes.
    pub title: String,
    /// Converted title + suffix for use in urls
    pub slug: String,
    /// The body will be rendered in markdown and will constitute the main content of the article.
    pub body: String,
    /// The presence of a publish date will idicate the article's published status,
    /// and will be used in ordering sets of the most recent articles.
    pub publish_date: Option<NaiveDateTime>,
}

/// Specifies the attributes that can be changed for an article.
#[derive(AsChangeset, Clone, Debug, PartialEq)]
#[primary_key(uuid)]
#[table_name = "articles"]
pub struct ArticleChangeset {
    pub uuid: Uuid,
    pub title: Option<String>,
    pub body: Option<String>,
}



/// Represents an article that will be inserted into the database.
#[derive(Insertable, Debug, Clone)]
#[table_name = "articles"]
pub struct NewArticle {
    pub title: String,
    pub slug: String,
    pub body: String,
    pub author_uuid: Uuid,
}

pub struct ArticleData {
    pub article: Article,
    pub user: User,
}


impl Article {

    pub fn get_article(uuid: ArticleUuid,conn: &PgConnection) -> BackendResult<Article> {
        get_row::<Article,_>(schema::articles::table, uuid.0, conn)
    }
    pub fn delete_article(uuid: ArticleUuid, conn: &PgConnection) -> BackendResult<Article> {
        delete_row::<Article,_>(schema::articles::table, uuid.0, conn)
    }
    pub fn create_article(new: NewArticle, conn: &PgConnection) -> BackendResult<Article> {
        create_row::<Article, NewArticle,_>(schema::articles::table, new, conn)
    }
    pub fn update_article_2(changeset: ArticleChangeset, conn: &PgConnection) -> BackendResult<Article> {
        update_row::<Article, ArticleChangeset,_>(schema::articles::table, changeset, conn)
    }

    // /// Gets the n most recent articles, where n is specified by the number_of_articles parameter.
    // /// The the returned articles will only include ones with a publish date.
    // pub fn get_recent_published_articles(number_of_articles: i64, conn: &Conn) -> JoeResult<Vec<Article>> {
    //     use schema::articles::dsl::*;

    //     let returned_articles: Result<Vec<Article>, Error> = articles
    //         .filter(publish_date.is_not_null())
    //         .limit(number_of_articles)
    //         .order(publish_date)
    //         .load::<Article>(conn.deref());

    //     returned_articles.or(Err(
    //         WeekendAtJoesError::DatabaseError(None),
    //     ))
    // }

    pub fn get_article_data(article_uuid: ArticleUuid, conn: &PgConnection) -> BackendResult<ArticleData> {

        let article = Article::get_article(article_uuid, conn)?;
        let user = User::get_user(UserUuid(article.author_uuid), conn)?;
        Ok(ArticleData { article, user })
    }

    pub fn get_paginated(page_index: i32, page_size: i32, conn: &PgConnection) -> BackendResult<Vec<ArticleData>> {
        use crate::schema::articles::dsl::*;
        use crate::diesel_extensions::pagination::*;
        use crate::schema::users;

        let (articles_and_users, _count) = articles
            .inner_join(users::table)
            .filter(publish_date.is_not_null())
            .order(publish_date)
            .paginate(page_index.into())
            .per_page(page_size.into())
            .load_and_count_pages::<(Article, User)>(conn)
            .map_err(handle_err::<Article>)?;

        let article_data = articles_and_users
            .into_iter()
            .map(|x| {
                ArticleData {
                    article: x.0,
                    user: x.1,
                }
            })
            .collect();

        Ok(article_data)
    }




    /// Gets the unpublished articles for a given user
    pub fn get_unpublished_articles_for_user(user_uuid: UserUuid, conn: &PgConnection) -> BackendResult<Vec<Article>> {
        use crate::schema::articles::dsl::*;
        use crate::schema::users::dsl::*;
        //        use schema::users;

        let user: User = users
            .find(user_uuid.0)
            .get_result::<User>(conn)
            .map_err(handle_err::<User>)?;


        Article::belonging_to(&user)
            .filter(publish_date.is_null())
            .order(publish_date)
            .load::<Article>(conn)
            .map_err(handle_err::<Article>)

    }

    /// Sets the date for the article's publish date.
    /// If true, it will set the publish datetime to the current time, indicating it is published.
    /// If false, it will set the publish column to Null, indicating that it has not been published.
    pub fn set_publish_status(article_uuid: ArticleUuid, publish: bool, conn: &PgConnection) -> BackendResult<Article> {
        use crate::schema::articles::dsl::*;
        use crate::schema::articles;

        let publish_value: Option<NaiveDateTime> = if publish {
            Some(Utc::now().naive_utc())
        } else {
            None
        };

        diesel::update(articles::table)
            .filter(articles::uuid.eq(article_uuid.0))
            .set(publish_date.eq(publish_value))
            .get_result(conn)
            .map_err(handle_err::<Article>)
    }


    /// Applies the changeset to its corresponding article.
    pub fn update_article(changeset: ArticleChangeset, conn: &PgConnection) -> BackendResult<Article> {
        use crate::schema::articles;
        diesel::update(articles::table)
            .set(&changeset)
            .get_result(conn)
            .map_err(handle_err::<Article>)
    }
}
