#![allow(unused_imports)]
use yew::prelude::*;

use yew_router::prelude::*;

use crate::yew_routes::YewRoute;


#[function_component]
pub fn TestContent( ) -> HtmlResult {

    let loc = use_location().unwrap();

    Ok(html! {
        <>
            <div class={ "column" }>

                { format!("Query string: {}", loc.query_str()) }

            </div>

            <div class={ "column is-one-quarter" }>

                <Link<YewRoute> to={YewRoute::Home }>
                    { "Home" }
                </Link<YewRoute>>

            </div>

        </>
    })

}