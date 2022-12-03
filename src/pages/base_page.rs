#![allow(unused_imports)]
use yew::prelude::*;

use yew_router::prelude::*;

use crate::pages::components::home_content::HomeContent;
use crate::pages::components::about_content::AboutContent;


#[derive(Properties, PartialEq, Eq, Debug, Clone)]
pub struct HomeProps {
    pub subpage: String,
}

#[function_component]
pub fn BasePage( props: &HomeProps  ) -> HtmlResult {

    Ok(html! {
        <>

         <div class={ "columns" }>
                    <div class={ "column is-one-third" } > </div>
                    <div class={ "column is-one-third" } >

                        <div class={ "card" }>
                            <div class= { "card-header" }>
                                <p class= { "card-header-title" }>
                                   { "This is hybrid rendering" }
                                 </p>

                            </div>
                          <div class= {"card-content"}>

                                <div class={ "columns" }>

                                    if props.subpage.eq("home") {
                                        <HomeContent />
                                    }
                                    else if props.subpage.eq("about") {
                                        <AboutContent />
                                    }

                                </div>

                            </div>
                        </div>
                    </div>
                </div>

        </>
    })

}