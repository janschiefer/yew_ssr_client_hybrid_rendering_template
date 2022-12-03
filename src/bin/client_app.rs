extern crate wee_alloc;

use yew::prelude::*;
use yew_router::prelude::*;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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