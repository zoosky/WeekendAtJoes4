use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::virtual_dom::VList;

pub fn empty<CTX, CMP>() -> Html<CTX, CMP>
    where
        CTX: 'static,
        CMP: Component<CTX>
{
    VNode::from(VList::new())
}