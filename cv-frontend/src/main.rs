mod app;
mod contact_list;
mod footer;
mod intro;
mod experience;

use app::App;

fn main() {
    yew::start_app::<App>();
}
