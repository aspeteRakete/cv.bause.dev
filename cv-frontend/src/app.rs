use yew::prelude::*;


#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <header>
                <div class={"header-name"}>
                    <h1 class={"flexbox-item"}>{"Felix Dubrownik"}</h1>
                    <h2 class={"flexbox-item"}>{"Senior Software Engineer"}</h2>
                </div>
                <div class={"header-contact"}>
                    <h3 class={"contact-item"}>{"Mail"}</h3>
                    <h3 class={"contact-item"}>{"Website"}</h3>
                    <h3 class={"contact-item"}>{"Location"}</h3>
                </div>
            </header>
            <main>
                {"Main Content"}
            </main>

            <footer>
                {"Footer Content"}
            </footer>
        </>
    }
}
