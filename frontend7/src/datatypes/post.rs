use requests_and_responses::post::PostResponse;
use datatypes::user::UserData;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct MinimalNewPostData {
    pub author_id: i32,
    pub content: String,
}


#[derive(Debug, Clone, PartialEq, Default)]
pub struct PostData {
    pub id: i32,
    pub author: UserData,
    pub created_date: i64,
    pub modified_date: Option<i64>,
    pub content: String,
    pub censored: bool,
    pub children: Vec<PostData>,
}

impl From<PostResponse> for PostData {
    fn from(response: PostResponse) -> Self{
        PostData {
            id: response.id,
            author: UserData::from(response.author),
            created_date: response.created_date,
            modified_date: response.modified_date,
            content: response.content,
            censored: response.censored,
            children: response.children.into_iter().map(PostData::from).collect()
        }
    }
}