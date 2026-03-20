//! ガチャロジックとクールダウン管理。
//!
//! - 5問セットの生成
//! - 超級の10%抽選
//! - 3時間クールダウンの保存・判定

use gloo::storage::{LocalStorage, Storage};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use crate::problems::{Difficulty, Problem, grade_1, grade_2, grade_3};

// ---- 定数 -------------------------------------------------------------------

/// クールダウン時間（ミリ秒）: 3時間
const COOLDOWN_MS: f64 = 3.0 * 60.0 * 60.0 * 1000.0;

/// LocalStorage に保存するキー
const STORAGE_KEY: &str = "expansion_gacha_last_played";

/// 超級が出る確率
const ULTRA_PROB: f64 = 0.10;

/// LocalStorage に保存するキー（問題データ用）
const PROBLEMS_STORAGE_KEY: &str = "expansion_gacha_last_problems";

// ---- 公開 API ---------------------------------------------------------------

/// LocalStorage から保存済みの問題リストを読み込む。
/// 保存されていない、またはパースに失敗した場合は空の Vec を返す。
pub fn load_problems() -> Vec<Problem> {
    LocalStorage::get::<String>(PROBLEMS_STORAGE_KEY)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_else(Vec::new)
}

/// ガチャを1回引き、5問のセットを返す。
///
/// 問題構成:
/// 1問目: 初級（1・2・3年からランダム）
/// 2問目: 初級（1・2・3年からランダム）
/// 3問目: 中級（2・3年からランダム）
/// 4問目: 中級（2・3年からランダム）
/// 5問目: 上級（90%）または超級（10%）
pub fn draw() -> Vec<Problem> {
    // 現在時刻をシードにすることで毎回異なる問題が出る
    let seed = js_sys::Date::now() as u64;
    let mut rng = StdRng::seed_from_u64(seed);

    let p1 = draw_easy(&mut rng);
    let p2 = draw_easy(&mut rng);
    let p3 = draw_medium(&mut rng);
    let p4 = draw_medium(&mut rng);
    let p5 = draw_last(&mut rng);

    let problems = vec![p1, p2, p3, p4, p5];

    // 最終プレイ時刻と問題リストを LocalStorage に保存
    save_last_played();
    if let Ok(json) = serde_json::to_string(&problems) {
        let _ = LocalStorage::set(PROBLEMS_STORAGE_KEY, json);
    }

    problems
}

/// クールダウンが終了しているか（= ガチャを引けるか）を返す。
pub fn is_ready() -> bool {
    remaining_ms() == 0.0
}

/// クールダウンの残り時間をミリ秒で返す。0 のときは引ける状態。
pub fn remaining_ms() -> f64 {
    let last = load_last_played();
    if last == 0.0 {
        return 0.0; // 一度も引いたことがない
    }
    let elapsed = js_sys::Date::now() - last;
    (COOLDOWN_MS - elapsed).max(0.0)
}

/// 残り時間を `HH:MM:SS` 形式の文字列で返す。
pub fn remaining_display() -> String {
    let ms = remaining_ms();
    if ms == 0.0 {
        return String::new();
    }
    let total_secs = (ms / 1000.0) as u64;
    let h = total_secs / 3600;
    let m = (total_secs % 3600) / 60;
    let s = total_secs % 60;
    format!("{:02}:{:02}:{:02}", h, m, s)
}

// ---- 内部ロジック -----------------------------------------------------------

/// 初級問題を生成する（全学年からランダム）。
fn draw_easy(rng: &mut impl Rng) -> Problem {
    match rng.gen_range(0..3u8) {
        0 => grade_1::generate(rng),
        1 => grade_2::generate(rng, &Difficulty::Easy),
        _ => grade_3::generate(rng, &Difficulty::Easy),
    }
}

/// 中級問題を生成する（2・3年からランダム）。
fn draw_medium(rng: &mut impl Rng) -> Problem {
    if rng.gen_bool(0.5) {
        grade_2::generate(rng, &Difficulty::Medium)
    } else {
        grade_3::generate(rng, &Difficulty::Medium)
    }
}

/// 5問目: 90% で上級、10% で超級。
fn draw_last(rng: &mut impl Rng) -> Problem {
    if rng.gen_bool(ULTRA_PROB) {
        grade_3::generate(rng, &Difficulty::Ultra)
    } else {
        grade_3::generate(rng, &Difficulty::Hard)
    }
}

/// 最終プレイ時刻を LocalStorage に保存する。
fn save_last_played() {
    let now = js_sys::Date::now();
    let _ = LocalStorage::set(STORAGE_KEY, now.to_string());
}

/// LocalStorage から最終プレイ時刻（Unix ms）を読み込む。
/// 取得できない場合は 0.0 を返す。
fn load_last_played() -> f64 {
    LocalStorage::get::<String>(STORAGE_KEY)
        .ok()
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.0)
}

// ---- テスト ------------------------------------------------------------------

#[cfg(test)]
mod tests {
    /// remaining_display が HH:MM:SS フォーマットを返すことを確認する。
    #[test]
    fn remaining_display_format() {
        // 残り時間 0 のとき空文字列
        assert_eq!(remaining_display_from_ms(0.0), "");
        // 残り 1時間30分45秒
        assert_eq!(remaining_display_from_ms(5445_000.0), "01:30:45");
        // 残り 2時間59分59秒
        assert_eq!(remaining_display_from_ms(10799_000.0), "02:59:59");
    }

    /// LocalStorage に依存しないバージョンのテスト用ヘルパー。
    fn remaining_display_from_ms(ms: f64) -> String {
        if ms == 0.0 {
            return String::new();
        }
        let total_secs = (ms / 1000.0) as u64;
        let h = total_secs / 3600;
        let m = (total_secs % 3600) / 60;
        let s = total_secs % 60;
        format!("{:02}:{:02}:{:02}", h, m, s)
    }
}
