use yew::services::fetch::FetchTask;
use yew::prelude::*;
use std::fmt::Formatter;
use std::fmt::Debug;

use context::networking::FtWrapper;

use util::loading::LoadingType;

use util::empty::empty_vdom_node;



pub enum Loadable<T> {
    Unloaded,
    Loading(FetchTask),
    Loaded(T),
    Failed(Option<String>)
}

impl <T> FtWrapper for Loadable<T> where T: Default {
    fn set_ft(&mut self, ft: FetchTask) {
        *self = Loadable::Loading(ft)
    }
}

impl <T> Debug for Loadable<T> where T: Debug {
    fn fmt(&self, f: &mut Formatter) -> Result<(), ::std::fmt::Error> {
        match self {
            Loadable::Unloaded => write!(f, "Unloaded"),
            Loadable::Loading(_) => write!(f, "Loading"),
            Loadable::Loaded(t) => write!(f, "Loaded: {:?}", t),
            Loadable::Failed(_) => write!(f, "Failed"),
        }
    }
}

impl <T> Default for Loadable<T> {
    fn default() -> Self {
        Loadable::Unloaded
    }
}

impl <T> Clone for Loadable<T>
    where T: Clone
{
    fn clone(&self) -> Self {
        match self {
            Loadable::Unloaded => Loadable::Unloaded,
            Loadable::Loading(_) => Loadable::Unloaded, // Any loading loadable throws away the FT because it can't be cloned
            Loadable::Loaded(t) => Loadable::Loaded(t.clone()),
            Loadable::Failed(o) => Loadable::Failed(o.clone())
        }
    }
}



impl <T> Loadable<T> {


    fn custom_view<U, CTX, LoadedFn, FailedFn>(&self, loaded_fn: LoadedFn, loading: Html<CTX,U>, failed_fn: FailedFn) -> Html<CTX, U>
        where
        CTX: 'static,
        U: Component<CTX>,
        LoadedFn: Fn(&T) -> Html<CTX, U>,
        FailedFn: Fn(&Option<String>) -> Html<CTX, U>
    {
        match self {
            Loadable::Unloaded => empty_vdom_node(),
            Loadable::Loading(_) => loading,
            Loadable::Loaded(ref t) => loaded_fn(t),
            Loadable::Failed(ref error) => failed_fn(error)
        }
    }


    fn failed<U, CTX> (error: &Option<String>) -> Html<CTX, U>
    where
        CTX: 'static,
        U: Component<CTX>,
    {
        if let Some(message) = error {
            html! {
                <div class="flexbox-center",>
                    {message}
                </div>
            }
        }
        else {
            html! {
                <div class="flexbox-center",>
                    {"Request Failed"}
                </div>
            }
        }
    }

    /// Uses a 100x100 icons for loading and error.
    /// This should work for medium to large sized views, but if a view dimension can be less than that, visual bugs will result.
    pub fn default_view<U, CTX>(&self, render_fn: fn(&T) -> Html<CTX, U> ) -> Html<CTX, U>
        where
            CTX: 'static,
            U: Component<CTX>
    {
        self.custom_view(
            render_fn,
            LoadingType::Fidget{diameter: 100}.view(),
            Self::failed
        )
    }

    /// Uses text for all error and loading fillers.
    /// This should allow it to be used most flexibly.
    pub fn small_view<U, CTX>(&self, render_fn: fn(&T) -> Html<CTX, U> ) -> Html<CTX, U>
        where
            CTX: 'static,
            U: Component<CTX>
    {
        self.custom_view(
            render_fn,
            LoadingType::Empty.view(),
            Self::failed
        )
    }

}