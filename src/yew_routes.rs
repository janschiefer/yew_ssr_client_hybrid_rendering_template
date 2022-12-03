use yew::prelude::*;
use yew_router::prelude::*;
use yew::Html;

use crate::pages::home::Home;
use crate::pages::about::About;
use crate::pages::test::Test;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum YewRoute {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/test/:id")]
    Test { id: String } ,
}

pub fn switch_yew_route( routes: YewRoute ) -> Html {
    match routes {
        YewRoute::Home => {
            html! { <Home /> }
        },
        YewRoute::About => {
            html! { <About /> }
        },
        YewRoute::Test { id } => {
            html! { <Test id={ id.clone() } /> }
        },
    }
}