use yew::prelude::*;
use yew_router::prelude::*;

use crate::yew_routes::YewRoute;

#[function_component]
pub fn HomeContent( ) -> HtmlResult {

    Ok(html! {
        <>
            <div class={ "column is-one-quarter" }>

            <Link<YewRoute> to={YewRoute::About }>
                { "About page" }
                </Link<YewRoute>>

            </div>

            <div class={ "column is-one-quarter" }>

                <Link<YewRoute> to={YewRoute::Home }>
                    { "Home" }
                </Link<YewRoute>>

            </div>

        </>
    })

}