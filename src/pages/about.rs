use yew::prelude::*;
use yew_router::prelude::*;

use crate::yew_routes::YewRoute;

#[function_component]
pub fn About( ) -> HtmlResult {

    Ok(html! {
        <>
        <div> { "Made by Jan Schiefer."  } </div>

        <Link<YewRoute> to={YewRoute::Home}>
                        { "Go to home" }
        </Link<YewRoute>>

        </>
    })

}