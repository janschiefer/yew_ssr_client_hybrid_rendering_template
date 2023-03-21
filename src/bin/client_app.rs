use yew::prelude::*;
use yew_router::prelude::*;

use yew_hybrid_ssr_client_template::yew_routes::{ YewRoute, switch_yew_route };

#[function_component]
pub fn ClientApp() -> Html {

    html! {
        <BrowserRouter>

            <Switch<YewRoute> render={switch_yew_route} />

        </BrowserRouter>

    }
}

fn main() {
    #[cfg(target_arch = "wasm32")]
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::Renderer::<ClientApp>::new().hydrate();
}
