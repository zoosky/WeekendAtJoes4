use crate::user::UserResponse;
use identifiers::answer::AnswerUuid;
use identifiers::question::QuestionUuid;


#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AnswerResponse {
    pub uuid: AnswerUuid,
    pub answer_text: Option<String>,
    pub author: UserResponse,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NewAnswerRequest {
    pub question_uuid: QuestionUuid,
    pub answer_text: Option<String>,
}
