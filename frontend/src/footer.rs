use yew::prelude::*;
use yew_icons::{Icon, IconId};


#[derive(PartialEq, Properties)]
pub struct FooterProps {

}

#[function_component(Footer)]
pub fn footer(_props: &FooterProps) -> Html {
    html! {
        <div class="text-center">
            <ul class="list-inline mx-auto mb-0 d-inline-block text-muted">
                <li class="list-inline-item mb-lg-0 mr-3">
                    <a href="//github.com/like-a-bause" class="link-secondary">
                        <Icon icon_id={IconId::BootstrapGithub}/>
                        <span class="d-none d-lg-inline-block text-muted"> {"like-a-bause"} </span>
                    </a>
                </li>
                <li class="list-inline-item mb-lg-0 mr-3">
                    <a href="//linkedin.com/in/felix-b-bause-32996b1bb/" class="link-secondary">
                        <Icon icon_id={IconId::BootstrapLinkedin}/>
                        <span class="d-none d-lg-inline-block text-muted"> {"felix-bause"} </span>
                    </a>
                </li>
                // <li class="list-inline-item mb-lg-0 mr-3">
                //     <a href="//twitter.com/aspeterakete" class="link-secondary">
                //         <Icon icon_id={IconId::BootstrapTwitter}/>
                //         <span class="d-none d-lg-inline-block text-muted"> {"@aspeterakete"} </span>
                //     </a>
                // </li>
            </ul>
        </div>
    }
}