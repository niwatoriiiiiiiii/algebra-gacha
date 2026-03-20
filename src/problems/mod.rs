//! 問題データの共通型定義。
//!
//! 各学年モジュール（`grade1`, `grade2`, `grade3`）はこのモジュールで定義した
//! 型を使って問題を生成する。

pub mod grade_1;
pub mod grade_2;
pub mod grade_3;

// ---- 難易度 ------------------------------------------------------------------

use serde::{Deserialize, Serialize};

/// 問題の難易度。ガチャの問題番号と出題範囲に対応する。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Difficulty {
    /// 初級（1・2問目）: 教科書例題レベル
    Easy,
    /// 中級（3・4問目）: 定期テストレベル
    Medium,
    /// 上級（5問目・95%）: 公立・私立高校入試レベル
    Hard,
    /// 超級（5問目・5%）: 難関校・数検レベル
    Ultra,
}

impl Difficulty {
    /// 難易度の表示ラベルを返す。
    pub fn label(&self) -> &'static str {
        match self {
            Difficulty::Easy => "EASY",
            Difficulty::Medium => "MEDIUM",
            Difficulty::Hard => "HARD",
            Difficulty::Ultra => "ULTRA",
        }
    }
}

// ---- 問題データ --------------------------------------------------------------

/// ガチャで出題される1問分のデータ。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Problem {
    /// 難易度
    pub difficulty: Difficulty,
    /// 問題の指示文（日本語プレーンテキスト。例: "次の計算をしなさい。"）
    pub instruction: String,
    /// 問題式（純粋な LaTeX 文字列。日本語を含まない）
    pub question_latex: String,
    /// 最終的な答え（LaTeX 文字列）
    pub answer_latex: String,
    /// 解説ステップのリスト（各要素が LaTeX 文字列）
    pub steps: Vec<String>,
}

// ---- ヘルパー: ランダムな整数を選ぶ -----------------------------------------

use rand::Rng;

/// `candidates` から1つをランダムに選んで返す。
///
/// `candidates` が空の場合はパニックする（呼び出し元で必ず非空を保証すること）。
pub fn pick<T: Copy>(rng: &mut impl Rng, candidates: &[T]) -> T {
    candidates[rng.gen_range(0..candidates.len())]
}

/// 正負ランダムな符号（`1` または `-1`）を返す。
pub fn random_sign(rng: &mut impl Rng) -> i32 {
    if rng.gen_bool(0.5) { 1 } else { -1 }
}

/// LaTeX の分数文字列を生成する。
///
/// 分母が 1 のときは `numerator` のみを返す（`\frac{3}{1}` を避ける）。
pub fn latex_frac(numerator: i32, denominator: i32) -> String {
    if denominator == 1 {
        format!("{}", numerator)
    } else if numerator < 0 {
        format!("-\\frac{{{}}}{{{}}}", numerator.abs(), denominator)
    } else {
        format!("\\frac{{{}}}{{{}}}", numerator, denominator)
    }
}

/// 係数が 1 または -1 のときに変数前の数字を省略した文字列を返す。
///
/// 例: `coeff_str(1, "x")` → `"x"`, `coeff_str(-1, "x")` → `"-x"`,
///     `coeff_str(3, "x")` → `"3x"`
pub fn coeff_str(coeff: i32, var_name: &str) -> String {
    match coeff {
        1 => var_name.to_string(),
        -1 => format!("-{}", var_name),
        _ => format!("{}{}", coeff, var_name),
    }
}
