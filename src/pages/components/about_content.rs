#![allow(unused_imports)]
use yew::prelude::*;

use yew_router::prelude::*;

use crate::yew_routes::YewRoute;


#[function_component]
pub fn AboutContent( ) -> HtmlResult {

    Ok(html! {
        <>
            <div class={ "column" }>

                { "Created by Dr. med. Jan Schiefer" }

            </div>

            <div class={ "column is-one-quarter" }>

                <Link<YewRoute> to={YewRoute::Home }>
                    { "Home" }
                </Link<YewRoute>>

            </div>

        </>
    })

}