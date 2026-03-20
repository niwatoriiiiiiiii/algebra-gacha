//! 問題カードコンポーネント。
//!
//! - instruction を日本語プレーンテキストで表示する
//! - question_latex を KaTeX でレンダリングする（空のときは非表示）
//! - 超級のときは `.ultra-rare` クラスを付与してダークテーマにする

use yew::prelude::*;

use crate::katex::{render_latex, render_mixed};
use crate::problems::{Difficulty, Problem};

// ---- Props ------------------------------------------------------------------

#[derive(Properties, PartialEq)]
pub struct ProblemCardProps {
    /// 表示する問題データ
    pub problem: Problem,
    /// 1始まりの問題番号（1〜5）
    pub index: usize,
}


// ---- コンポーネント ---------------------------------------------------------

/// 問題カードコンポーネント。
#[function_component(ProblemCard)]
pub fn problem_card(props: &ProblemCardProps) -> Html {
    let is_ultra = props.problem.difficulty == Difficulty::Ultra;

    let card_class = if is_ultra {
        "problem-card ultra-rare"
    } else {
        "problem-card"
    };

    let has_formula = !props.problem.question_latex.is_empty();
    let question_html = has_formula
        .then(|| render_latex(&props.problem.question_latex, true));

    html! {
        <div class={ card_class }>
            <div class="problem-label">
                { format!("No.{} — {}", props.index, props.problem.difficulty.label()) }
            </div>

            // 日本語指示文（日本語はプレーン、LaTeX 部分は KaTeX でレンダリング）
            <div class="problem-instruction">
                { Html::from_html_unchecked(AttrValue::from(render_mixed(&props.problem.instruction))) }
            </div>

            // 数式（LaTeX が空でないときだけ表示）
            if let Some(html) = question_html {
                <div class="problem-body">
                    { Html::from_html_unchecked(AttrValue::from(html)) }
                </div>
            }
        </div>
    }
}
