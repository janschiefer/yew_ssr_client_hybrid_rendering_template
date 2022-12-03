pub mod pages;
pub mod yew_routes;

#[cfg(not(target_arch = "wasm32"))]
pub mod backend_ssr;
