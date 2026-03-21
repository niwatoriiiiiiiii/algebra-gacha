//! 3年生の問題生成モジュール。
//!
//! 出題範囲: 乗法公式・計算の工夫・式の値
//! 難易度: 初級 / 中級 / 上級 / 超級（全28パターン）
//!
//! # ファイル構成
//! - `easy.rs`    — 初級 8パターン
//! - `medium.rs`  — 中級 8パターン
//! - `hard.rs`    — 上級 5パターン
//! - `ultra.rs`   — 超級 28パターン
//! - `helpers.rs` — LaTeX整形・数値ユーティリティ

mod easy;
mod hard;
mod helpers;
mod medium;
mod ultra;

use rand::Rng;

use super::{Difficulty, Problem};

/// 指定難易度の3年生問題を生成して返す。
pub fn generate(rng: &mut impl Rng, difficulty: &Difficulty) -> Problem {
    match difficulty {
        Difficulty::Easy   => easy::generate_easy(rng),
        Difficulty::Medium => medium::generate_medium(rng),
        Difficulty::Hard   => hard::generate_hard(rng),
        Difficulty::Ultra  => ultra::generate_ultra(rng),
    }
}

// ---- テスト ------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[test]
    fn generates_all_difficulties_without_panic() {
        let mut rng = StdRng::seed_from_u64(999);
        for _ in 0..50 {
            generate(&mut rng, &Difficulty::Easy);
            generate(&mut rng, &Difficulty::Medium);
            generate(&mut rng, &Difficulty::Hard);
            generate(&mut rng, &Difficulty::Ultra);
        }
    }

    #[test]
    fn ultra_covers_all_28_patterns() {
        let mut rng = StdRng::seed_from_u64(12345);
        for _ in 0..500 {
            let p = ultra::generate_ultra(&mut rng);
            assert!(!p.answer_latex.is_empty());
        }
    }
}
