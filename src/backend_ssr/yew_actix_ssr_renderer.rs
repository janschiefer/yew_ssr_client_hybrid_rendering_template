use actix_web::{ HttpResponse, HttpRequest, web };
use tokio::task::{ LocalSet, spawn_blocking };
use qstring::QString;

use crate::backend_ssr::actix_server::WebApplicationData;

use crate::backend_ssr::yew_server_app::{ ServerApp, ServerAppProps, RequestData };

pub async fn ssr_render( req: HttpRequest, data: web::Data<WebApplicationData> ) -> Result<HttpResponse, actix_web::Error> {

    let req_url = req.uri().path().to_string().clone();

    let req_queries : Vec< (String, String) > = QString::from(req.query_string()).into();

    let content = spawn_blocking(move || {
        use tokio::runtime::Builder;
        let set = LocalSet::new();

        let rt = Builder::new_current_thread().enable_all().build().unwrap();

        set.block_on(&rt, async {
            let renderer = yew::ServerRenderer::<ServerApp>::with_props(move ||
                ServerAppProps {
                    request_data :
                        RequestData {
                            url: req_url.into(),
                            queries: req_queries,
                        }
            });

            renderer.render().await

        })

    })
        .await
        .expect("the thread has failed.");

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!(
            r#" {}
                    {}
                {}
            "#,
            data.base_template.index_html_before,
            content,
            data.base_template.index_html_after
        )))
}