mod app;
mod components;
mod gacha;
mod katex;
mod problems;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
