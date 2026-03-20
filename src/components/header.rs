//! ヘッダーコンポーネント。
//!
//! タイトル「EXPANSION-GACHA」を Orbitron フォントで表示する。

use yew::prelude::*;

/// ヘッダーコンポーネント。Props なし。
#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header>
            <svg viewBox="0 0 200 200" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M40 40L80 80" stroke="currentColor" stroke-width="12" stroke-linecap="square"></path>
                <path d="M160 40L120 80" stroke="currentColor" stroke-width="12" stroke-linecap="square"></path>
                <path d="M40 160L80 120" stroke="currentColor" stroke-width="12" stroke-linecap="square"></path>
                <path d="M160 160L120 120" stroke="currentColor" stroke-width="12" stroke-linecap="square"></path>
                <path d="M20 60V20H60" stroke="currentColor" stroke-width="2"></path>
                <path d="M140 20H180V60" stroke="currentColor" stroke-width="2"></path>
                <path d="M20 140V180H60" stroke="currentColor" stroke-width="2"></path>
                <path d="M140 180H180V140" stroke="currentColor" stroke-width="2"></path>
            </svg>
            <h1>{ "EXPANSION-" }<br/>{ "GACHA" }</h1>
        </header>
    }
}
