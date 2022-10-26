use yew::prelude::*;
use yew_icons::{Icon, IconId};
use yew::virtual_dom::AttrValue;


#[derive(PartialEq, Properties)]
pub struct ContactListProps {
    pub mail: AttrValue,
    pub website: AttrValue,
    pub location: AttrValue
}

#[function_component(ContactList)]
pub fn contact_list(props: &ContactListProps) -> Html {
    let mut mailto = String::from("mailto:");
    mailto.push_str(&props.mail);

    html! {
        <div class="contact-list vstack gap-2">
            <div>
                <a href={mailto} class="link-secondary list-group-item">
                    <Icon icon_id={IconId::FeatherMail}/> {&props.mail}
                </a>
            </div>
            <div>
                <a href={props.website.clone()} class="link-secondary">
                    <Icon icon_id={IconId::FeatherGlobe}/> {&props.website}
                </a>
            </div>
            <div>
                <Icon icon_id={IconId::FeatherMapPin}/> {&props.location}
            </div>
        </div>
    }
}