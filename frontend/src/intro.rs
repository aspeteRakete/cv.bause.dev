use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct IntroProps {
}

#[function_component(Intro)]
pub fn intro(_props: &IntroProps) -> Html {
    html! {
        <div class="media flex-column flex-md-row align-items-center">
            <div class="row">
                <div class="col-12 col-md-3 col-lg-4 col-xl-3 text-center">
                     <img src="img/portrait2022_256x256.png" class="profile-image rounded mb-3 mr-md-5 ml-md-0 mx-auto"/>
                </div>
                <div class="col-12 col-md-9 col-lg-8 col-xl-9">
                    <p>
                        {"I am an experienced Software Engineer with a broad knowledge of application development and infrastructure architecture.
                        In university I started my journey in computer engineering with implementing specialized hardware on FPGAs and have since then moved up several abstraction layers.
                        Nonetheless I still have that urge to strive for efficiency and speed."}
                    </p>

                    <p>
                        {"Having spent all of my professional career in start-ups I see myself as a generalist with an agile mindset and a hands-on mentality. \
                            I learned to prioritize the most crucial tasks from a technical but also from a business perspective. \
                            As a Team Lead my principle is “leading by serving”. \
                            I have the strong believe that being an  encouraging leader yields the best results in the long run."}
                    </p>
                </div>
            </div>
        </div>
    }
}
