use warp::Filter;
use warp::filters::BoxedFilter;
use warp::reply::Reply;
use error::Error;
//use crate::db_integration::s.db.clone();
//use db::Conn;
use uuid::Uuid;
use crate::util::convert_and_json;
use crate::util::convert_vector_and_json;
//use crate::uuid_integration::uuid_filter;
use crate::util::json_body_filter;
use identifiers::user::UserUuid;
use crate::state::jwt::normal_user_filter;
use db::Message;
use db::Chat;
use identifiers::chat::ChatUuid;
use wire::message::MessageResponse;
use db::message::MessageData;
use wire::message::NewMessageRequest;
use db::message::NewMessage;
use crate::logging::log_attach;
use crate::logging::HttpMethod;
use crate::util::query_uuid;
use crate::state::State;
use pool::PooledConn;

pub fn message_api(s: &State) -> BoxedFilter<(impl Reply,)> {
    info!("Attaching Message API");
    let api = get_messages_for_chat(s)
        .or(send_message(s))
        ;

    warp::path("message")
        .and(api)
        .with(warp::log("message"))
        .boxed()
}


fn get_messages_for_chat(s: &State) -> BoxedFilter<(impl Reply,)> {

    log_attach(HttpMethod::Get, "message/<index=i32>?chat_uuid=<uuid>");

    warp::get2()
        .and(warp::path::param())
        .and(query_uuid("chat_uuid")) // TODO Is this the query??
        .and(normal_user_filter(s))
        .and(s.db.clone())
        .and_then(|index: i32, chat_uuid: Uuid, user_uuid: UserUuid, conn: PooledConn|{
            let chat_uuid = ChatUuid(chat_uuid);
            if !Chat::is_user_in_chat(&chat_uuid, user_uuid, &conn).map_err(Error::simple_reject)? {
                return Error::BadRequest.reject()
            }

            Message::get_messages_for_chat(chat_uuid, index, 25, &conn)
                .map(convert_vector_and_json::<MessageData, MessageResponse>)
                .map_err(Error::simple_reject)
        })
        .boxed()
}

fn send_message(s: &State) -> BoxedFilter<(impl Reply,)> {

    log_attach(HttpMethod::Post, "message/send");

    warp::post2()
        .and(warp::path::path("send"))
        .and(json_body_filter(20))
        .and(normal_user_filter(s))
        .and(s.db.clone())
        .and_then(|request: NewMessageRequest, user_uuid: UserUuid, conn: PooledConn|{
            if !Chat::is_user_in_chat(&request.chat_uuid, user_uuid, &conn).map_err(Error::simple_reject)? {
                return Error::BadRequest.reject()
            }
            if request.author_uuid != user_uuid {
                return Error::BadRequest.reject()
            }

            let new_message: NewMessage = request.into();
            Message::create_message(new_message, &conn)
                  .map(convert_and_json::<MessageData, MessageResponse>)
                .map_err(Error::simple_reject)
        })
        .boxed()
}