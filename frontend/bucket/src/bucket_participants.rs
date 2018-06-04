use Context;
use util::button::Button;
use yew::prelude::*;
use yew::format::Json;

use failure::Error;
use yew::services::fetch::Response;

use util::loadable::Loadable;
use util::uploadable::Uploadable;

use context::datatypes::bucket::BucketData;
use context::networking::RequestWrapper;
use context::datatypes::user::UserData;
use wire::user::UserResponse;
use identifiers::bucket::BucketUuid;
use identifiers::user::UserUuid;




/// A component for approving and rejecting requests to join buckets.
#[derive(Debug)]
pub struct BucketParticipants {
    users:  Loadable<Vec<UserData>>,
    is_user_bucket_owner: Loadable<bool>,
    bucket_id: Option<BucketUuid>,
    remove_user_action: Uploadable<()>
}

#[derive(Default, PartialEq, Debug, Clone)]
pub struct Props {
    pub bucket_data: Loadable<BucketData>

}

impl Default for BucketParticipants {
    fn default() -> Self {
        BucketParticipants {
            users: Loadable::default(),
            is_user_bucket_owner: Loadable::default(),
            bucket_id: None,
            remove_user_action: Uploadable::default()
        }
    }
}

pub enum Msg {
    GetBucketUserData{bucket_id: BucketUuid},
    BucketUserDataLoaded(Vec<UserData>),
    BucketUserDataFailed,
    SetIsUserOwner(bool),
    RemoveUserFromBucket{user_id: UserUuid}
}

impl BucketParticipants {
    fn get_participants_in_bucket(bucket_id: BucketUuid, participants: &mut Loadable<Vec<UserData>>, context: &mut Env<Context, Self>) {
        let callback = context.send_back(
            |response: Response<Json<Result<Vec<UserResponse>, Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                println!("META: {:?}, {:?}", meta, data);
                if meta.status.is_success() {
                    let new_bucket_users_data: Vec<UserData> = data.map(::wire::convert_vector).unwrap();
                    Msg::BucketUserDataLoaded(new_bucket_users_data)
                } else {
                    Msg::BucketUserDataFailed
                }
            },
        );

        context.make_request_and_set_ft(
            participants,
            RequestWrapper::GetUsersInBucket{bucket_id},
            callback,
        );
    }

    fn determine_if_user_is_owner(bucket_id: BucketUuid, is_owner: &mut Loadable<bool>, context: &mut Env<Context, Self>) {
        let callback = context.send_back(
            |response: Response<Json<Result<bool, Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                println!("META: {:?}, {:?}", meta, data);
                if meta.status.is_success() {
                    Msg::SetIsUserOwner(data.expect("Could not unwrap bool"))
                } else {
                    println!("Could not get user->bucket ownership info");
                    Msg::SetIsUserOwner(false)
                }
            },
        );

        context.make_request_and_set_ft(
            is_owner,
            RequestWrapper::GetIsUserOwnerOfBucket{bucket_id},
            callback,
        );
    }

    fn remove_user_from_bucket(bucket_id: BucketUuid, user_id: UserUuid, remove_user_action: &mut Uploadable<()>, context: &mut Env<Context, Self>) {
        let bucket_id: BucketUuid = bucket_id;
        let callback = context.send_back(
            move |response: Response<Json<Result<(), Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                println!("META: {:?}, {:?}", meta, data);
                Msg::GetBucketUserData {bucket_id} // This is lazy, but just get the whole list again.
            },
        );

        context.log(&format!("Removing user from bucket:{}-{}",user_id,bucket_id));

        context.make_request_and_set_ft(
            remove_user_action,
            RequestWrapper::RemoveUserFromBucket{bucket_id, user_id},
            callback,
        );
    }
}


impl Component<Context> for BucketParticipants {
    type Msg = Msg;
    type Properties = Props;


    fn create(props: Self::Properties, context: &mut Env<Context, Self>) -> Self {
        let mut participants: BucketParticipants = BucketParticipants::default();
        if let Loadable::Loaded(bucket_data) = props.bucket_data {
            Self::get_participants_in_bucket(bucket_data.id, &mut participants.users, context); // TODO Possibly don't load this on startup, only do this when opening the pane
            Self::determine_if_user_is_owner(bucket_data.id, &mut participants.is_user_bucket_owner, context);
            participants.bucket_id = Some(bucket_data.id);
        }
        participants
    }

    fn update(&mut self, msg: Msg, context: &mut Env<Context, Self>) -> ShouldRender {
        use self::Msg::*;
        match msg {
            GetBucketUserData {bucket_id} => {
                Self::get_participants_in_bucket(bucket_id, &mut self.users, context);
            }
            BucketUserDataLoaded(bucket_user_data) => {
                self.users = Loadable::Loaded(bucket_user_data);
            },
            BucketUserDataFailed => context.log("Failed to get bucket user data"),
            SetIsUserOwner(is_owner) => self.is_user_bucket_owner = Loadable::Loaded(is_owner),
            RemoveUserFromBucket{user_id} => {
                if let Some(bucket_id) = self.bucket_id {
                    Self::remove_user_from_bucket(bucket_id, user_id, &mut self.remove_user_action, context)
                } else {
                    context.log("Couldn't remove user because bucket id is unknown.")
                }
            }
        }
        true
    }


    fn change(&mut self, props: Self::Properties, context: &mut Env<Context, Self>) -> ShouldRender {
        if let Loadable::Loaded(bucket_data) = props.bucket_data {
            Self::get_participants_in_bucket(bucket_data.id, &mut self.users, context);
            Self::determine_if_user_is_owner(bucket_data.id, &mut self.is_user_bucket_owner, context);
            self.bucket_id = Some(bucket_data.id);
            true
        } else {
            false
        }
    }
}



impl Renderable<Context, BucketParticipants> for BucketParticipants {
    fn view(&self) -> Html<Context, BucketParticipants> {

        html! {
            <div class="bucket-action-pane",>
                {
                    if let Loadable::Loaded(is_owner) = self.is_user_bucket_owner {
                        if is_owner {
                            self.users.default_view( Self::users_owner_view)
                        }  else {
                            self.users.default_view( Self::users_not_owner_view )
                        }
                    } else {
                        self.users.default_view( Self::users_not_owner_view)
                    }
                }
            </div>
        }
    }
}

impl BucketParticipants {
    fn users_owner_view(users: &Vec<UserData>) -> Html<Context, BucketParticipants> {
        Self::users_view(users, true)
    }
    fn users_not_owner_view(users: &Vec<UserData>) -> Html<Context, BucketParticipants> {
        Self::users_view(users, false)
    }

    fn users_view(users: &Vec<UserData>, is_user_owner: bool) -> Html<Context, BucketParticipants> {

        fn user_view(user: &UserData, is_user_owner: bool) -> Html<Context, BucketParticipants> {
            let user_id = user.id;
            let delete_button = if is_user_owner {
                html! {
                    <Button: title="Remove", onclick=move |_| Msg::RemoveUserFromBucket{user_id} ,/>
                }
            } else {
                ::util::wrappers::empty_vdom_node()
            };

            html!{
                <div class=("flexbox-horiz","full-width"),>
                    <div class="flexbox-expand",>
                        {&user.user_name}
                    </div>
                    <div>
                        {delete_button}
                    </div>
                </div>
            }
        }

        html!{
            <>
                {for users.iter().map(|u| user_view(u, is_user_owner))}
            </>
        }
    }
}
