//! ガチャボタンコンポーネント。
//!
//! - クールダウン中: ボタンを disabled にして残り時間をカウントダウン表示
//! - 引ける状態: クリックで `on_draw` コールバックを呼び出す

use gloo::timers::callback::Interval;
use yew::prelude::*;

use crate::gacha;

// ---- Props ------------------------------------------------------------------

#[derive(Properties, PartialEq)]
pub struct GachaButtonProps {
    /// ガチャが引かれたときに呼び出すコールバック
    pub on_draw: Callback<()>,
}

// ---- コンポーネント ---------------------------------------------------------

/// ガチャボタンコンポーネント。
#[function_component(GachaButton)]
pub fn gacha_button(props: &GachaButtonProps) -> Html {
    // 残り時間の文字列（空文字列 = 引ける状態）
    let remaining = use_state(gacha::remaining_display);

    // 1秒ごとに残り時間を更新するインターバルタイマー
    {
        let remaining = remaining.clone();
        use_effect_with((), move |_| {
            let interval = Interval::new(1_000, move || {
                remaining.set(gacha::remaining_display());
            });
            // クリーンアップ: コンポーネントがアンマウントされたらタイマーを止める
            move || drop(interval)
        });
    }

    let is_ready = remaining.is_empty();

    let on_click = {
        let on_draw = props.on_draw.clone();
        let remaining = remaining.clone();
        Callback::from(move |_| {
            if gacha::is_ready() {
                on_draw.emit(());
                // クリック直後に残り時間を即時更新
                remaining.set(gacha::remaining_display());
            }
        })
    };

    html! {
        <div>
            <button
                class="gacha-trigger"
                disabled={ !is_ready }
                onclick={ on_click }
            >
                { "ガチャを引く" }
            </button>
            if !is_ready {
                <p class="cooldown-timer">
                    { format!("NEXT: {}", *remaining) }
                </p>
            }
        </div>
    }
}
