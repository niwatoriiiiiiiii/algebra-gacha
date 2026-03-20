//! フッターコンポーネント。

use yew::prelude::*;

/// フッターコンポーネント。
#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer>
            { "© 2026 niwatoriiiiiiiii" }
        </footer>
    }
}
