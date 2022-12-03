use yew::prelude::*;
use yew_router::prelude::*;
use yew::Html;

use crate::pages::base_page::BasePage;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum YewRoute {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/test")]
    Test,
}

pub fn switch_yew_route( routes: YewRoute ) -> Html {
    match routes {
        YewRoute::Home => {
            html! { <BasePage subpage = { "home" } /> }
        },
        YewRoute::About => {
            html! { <BasePage subpage = { "about" } /> }
        },
        YewRoute::Test => {
            html! { <BasePage subpage = { "test" } /> }
        }
    }
}