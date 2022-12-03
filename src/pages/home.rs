use yew::prelude::*;
use yew_router::prelude::*;

use crate::yew_routes::YewRoute;

#[function_component]
pub fn Home( ) -> HtmlResult {

    Ok(html! {
        <>
            <div> { "Hello world!" } </div>

                <Link<YewRoute> to={YewRoute::Test { id: "12345".to_string() } }>
                        { "Test with parameter" }
                </Link<YewRoute>>

                <br />

                <Link<YewRoute> to={YewRoute::About}>
                        { "About" }
                </Link<YewRoute>>
        </>
    })

}