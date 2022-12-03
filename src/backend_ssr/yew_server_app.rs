use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};

use crate::yew_routes::{ YewRoute, switch_yew_route };

#[derive(Properties, PartialEq, Eq, Debug, Clone)]
pub struct RequestData {
    pub url: AttrValue,
    pub queries: Vec< (String, String) >,
}

#[derive(Properties, PartialEq, Eq, Debug, Clone)]
pub struct ServerAppProps {
    pub request_data: RequestData,
}

#[function_component]
pub fn ServerApp( props: &ServerAppProps ) -> Html {

    let history = AnyHistory::from(MemoryHistory::new());
    history
        .push_with_query(&*props.request_data.url, &props.request_data.queries)
        .unwrap();

    html! {

        <Router history={history}>

            <Switch<YewRoute> render={ switch_yew_route }  />

        </Router>

    }
}