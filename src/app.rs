use uuid::Uuid;
use yew::prelude::*;

use super::components::footer::Footer;
use super::components::gacha_button::GachaButton;
use super::components::header::Header;
use super::components::problem_card::ProblemCard;
use super::gacha;
use super::katex::{render_latex, render_mixed};
use crate::problems::Difficulty;

#[function_component(App)]
pub fn app() -> Html {
    let problems = use_state(gacha::load_problems);

    // ガチャを引いた回数。
    // details の key に使い、ガチャのたびに再マウント（＝閉じる）させる。
    // open prop で制御すると GachaButton 内の 1秒タイマーの再レンダリングで
    // 強制クローズされるためこの方式を採用。
    let gacha_count = use_state(|| 0u32);

    let on_draw = {
        let problems = problems.clone();
        let gacha_count = gacha_count.clone();
        Callback::from(move |_| {
            problems.set(gacha::draw());
            gacha_count.set(*gacha_count + 1);
        })
    };

    let has_problems = !(*problems).is_empty();
    let has_ultra = (*problems).iter().any(|p| p.difficulty == Difficulty::Ultra);
    let container_class = if has_ultra { "app-container has-ultra" } else { "app-container" };

    html! {
        <main class={container_class}>
            <Header />

            <section class="info-area">
                <GachaButton on_draw={on_draw} />
            </section>

            <section class="results-area">
                {
                    // key に UUID を使うことで、ガチャのたびに DOM が再マウントされ
                    // CSS の登場アニメーションが毎回再生される
                    for (*problems).iter().enumerate().map(|(i, p)| {
                        html! {
                            <ProblemCard
                                key={Uuid::new_v4().to_string()}
                                problem={p.clone()}
                                index={i + 1}
                            />
                        }
                    })
                }
            </section>

            if has_problems {
                <section class="answer-area">
                    // key = gacha_count により、ガチャを引くたびに再マウントされ閉じる。
                    // タイマー再レンダリングでは gacha_count が変わらないので開閉状態が保たれる。
                    <details key={*gacha_count} class="answer-details">
                        <summary class="answer-details-summary">
                            <span>{ "問題の解答" }</span>
                        </summary>
                        {
                            for (*problems).iter().enumerate().map(|(i, p)| {
                                let answer_html = render_latex(&p.answer_latex, true);
                                let steps_html: Vec<String> = p.steps
                                    .iter()
                                    .map(|s| render_mixed(s))
                                    .collect();

                                html! {
                                    <div key={i} class="answer-problem">
                                        <span class="answer-problem-num">
                                            { format!("({})", i + 1) }
                                        </span>
                                        <div class="answer-content">
                                            <div class="final-answer">
                                                { Html::from_html_unchecked(AttrValue::from(answer_html)) }
                                            </div>
                                            <ul class="explanation-steps">
                                                { for steps_html.into_iter().map(|s| html! {
                                                    <li>
                                                        { Html::from_html_unchecked(AttrValue::from(s)) }
                                                    </li>
                                                }) }
                                            </ul>
                                        </div>
                                    </div>
                                }
                            })
                        }
                    </details>
                </section>
            }

            <Footer />
        </main>
    }
}
