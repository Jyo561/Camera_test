mod app;
mod models;
mod storage;
mod camera;

mod components;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
