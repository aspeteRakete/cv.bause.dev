use yew::prelude::*;
use crate::contact_list::ContactList;
use crate::footer::Footer;
use crate::intro::Intro;
use crate::experience::Experience;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main class="container px-3 px-lg-5 mx-auto theme-bg-light p-5 mb-5 my-5 shadow-lg">
            <header class={"row align-items-center"}>
                    <div class={"header-title col-12 col-md-6 col-lg-8 col-xl-9"}>
                        <h2>{"Felix B. Bause"}</h2>
                        <h4>{"Senior Software Engineer"}</h4>
                    </div>
                    <div class={"header-contact col-12 col-md-6 col-lg-4 col-xl-3"}>

                        <ContactList mail={"felix+cv@bause.dev"} location={"Kiel, Germany"} website="https://cv.bause.dev"/>
                    </div>
            </header>
            <hr />
            <div>
                <Intro />
                <hr />
                <Experience />
            </div>
            <hr />
            <footer>
                <Footer />
            </footer>
        </main>
    }
}
