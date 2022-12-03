use std::path::PathBuf;

use actix_files;

use actix_web::{App as ActixApp, HttpServer, middleware, web};

use clap::Parser;

use crate::backend_ssr::yew_actix_ssr_renderer::ssr_render;

#[derive(Clone, Debug)]
pub struct IndexFilePreloaded{
    pub index_html_before : String,
    pub index_html_after : String,
}

#[derive(Clone, Debug)]
pub struct WebApplicationData{
    pub base_template: IndexFilePreloaded,
}

#[derive(Parser, Debug)]
struct Opt {
    /// the "dist" created by trunk directory to be served for hydration.
    #[clap(short, long, parse(from_os_str))]
    dir: PathBuf,
}

pub async fn run_actix_server() -> std::io::Result<()> {

    env_logger::init();

    //Get commandline options
    let opts = Opt::parse();

    //Read and parse index.html template
    let index_html_s = tokio::fs::read_to_string(opts.dir.join("index.html"))
        .await
        .expect("failed to read index.html");

    let (index_html_before, index_html_after) = index_html_s.split_once("<body>").unwrap();

    let mut index_html_before = index_html_before.to_owned();

    index_html_before.push_str("<body>");

    let web_application_data : WebApplicationData = WebApplicationData {
        base_template :
        IndexFilePreloaded {
            index_html_before: index_html_before,
            index_html_after: index_html_after.to_owned()
        },
    };

    let server = HttpServer::new( move ||
        ActixApp::new()
            .wrap(middleware::Logger::default())
            .app_data( web::Data::new(web_application_data.clone()) )


            .service( web::resource("/").route(web::get().to(ssr_render) ) )
            .service( web::resource("/about").route(web::get().to(ssr_render) ) )

            .service( actix_files::Files::new("/", "./dist/") )

    );

    println!("You can view the website at: http://localhost:8080/");
    server.bind(("127.0.0.1", 8080))?.run().await
}