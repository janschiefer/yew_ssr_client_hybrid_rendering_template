use yew::prelude::*;
use yew_router::prelude::*;

use crate::yew_routes::YewRoute;

#[derive(Properties, PartialEq, Eq, Debug, Clone)]
pub struct TestProps {
    pub id: String,
}

#[function_component]
pub fn Test( props: &TestProps ) -> HtmlResult {

    Ok(html! {
        <>
            <div> { format!("Passed id parameter: {}", props.id.clone() ) } </div>

                <Link<YewRoute> to={YewRoute::Home}>
                        { "Home" }
                </Link<YewRoute>>
        </>
    })

}