use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[derive(PartialEq, Properties)]
pub struct WorkSectionProps {
    pub heading: AttrValue,
    pub time: AttrValue,
    pub text: AttrValue,
}


//TODO : How is it possible to paste html as text?
#[function_component(WorkSection)]
pub fn workSection(props: &WorkSectionProps) -> Html {
    html! {
        <div class="work-item">
            <div class="row align-items-center">
                <div class="work-item-heading col-12 col-md-6 col-lg-6">
                    {&props.heading}
                </div>
                <div class="work-item-time col-12 col-md-6 col-lg-6 text-muted text-left text-md-right">
                    {&props.time}
                </div>
            </div>
        </div>
    }
}