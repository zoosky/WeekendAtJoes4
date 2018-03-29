pub mod login_component;
pub mod create_account_component;

use yew::prelude::*;
use self::login_component::Login;
use self::create_account_component::CreateAccount;
use Context;


#[derive(Clone, PartialEq)]
pub enum AuthPage {
    Login,
    Create
}

pub struct Auth {
    pub child: AuthPage,
    pub callback: Option<Callback<()>>
}


pub enum Msg {
    Callback,
    SetChild(AuthPage)
}

#[derive(Clone, PartialEq)]
pub struct Props {
    pub child: AuthPage,
    pub callback: Option<Callback<()>>
}

impl Default for Props {
    fn default() -> Self {
        Props {
            child: AuthPage::Login,
            callback: None
        }
    }
}

impl Component<Context> for Auth {
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _:&mut Env<Context, Self>) -> Self {
        Auth {
            child: props.child,
            callback: props.callback
        }
    }

    fn update(&mut self, msg: Self::Msg, _: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::Callback => {
                if let Some(ref mut cb) = self.callback {
                    cb.emit(())
                }
                false
            }
            Msg::SetChild(child) => {
                self.child = child;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties, _: &mut Env<Context, Self>) -> ShouldRender {
        self.callback = props.callback;
        self.child = props.child;

        true
    }
}

impl Renderable<Context, Auth> for Auth{

    fn view(&self) -> Html<Context, Self> {

        let page = || {
            match &self.child {
                &AuthPage::Login => {
                    html! {
                        <>
                            <Login: login_nav_cb=|_| Msg::Callback, create_account_nav_cb=|_| Msg::SetChild(AuthPage::Create), />
                        </>
                    }
                }
                &AuthPage::Create => {
                    html! {
                        <>
                            <CreateAccount: nav_cb=|_| Msg::SetChild(AuthPage::Login), />
                        </>
                    }
                }
            }
        };

        return html! {
            <>
                {page()}
            </>
        }
    }
}



