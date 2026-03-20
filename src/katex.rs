//! KaTeX の wasm-bindgen バインディング。
//!
//! `index.html` で CDN から読み込んだ `katex` グローバルオブジェクトを
//! Rust から呼び出せるようにする。

use wasm_bindgen::prelude::*;

// ---- JS バインディング -------------------------------------------------------

#[wasm_bindgen]
extern "C" {
    /// `katex.renderToString(latex, options)` を呼び出す。
    ///
    /// KaTeX は DOM 直接書き込み版 (`katex.render`) と HTML 文字列返却版
    /// (`katex.renderToString`) を持つ。後者を使うことで Yew の `inner_html`
    /// に流し込める。
    ///
    /// # 引数
    /// * `latex`   - LaTeX 文字列（例: `r"\frac{1}{2}"`）
    /// * `options` - JS オブジェクト。`{ displayMode: true }` など
    #[wasm_bindgen(js_namespace = katex, js_name = renderToString)]
    fn render_to_string_js(latex: &str, options: &JsValue) -> String;
}

// ---- 公開 API ----------------------------------------------------------------

/// LaTeX 文字列を KaTeX で HTML 文字列に変換して返す。
///
/// `display_mode = true` のとき、数式をブロック（センタリング）で表示する。
/// `display_mode = false` のとき、インライン表示になる。
///
/// # Panics
/// KaTeX が `index.html` で正しく読み込まれていない場合、
/// JS 例外が発生して panic する。
pub fn render_latex(latex: &str, display_mode: bool) -> String {
    // JS オブジェクト `{ displayMode: bool, throwOnError: false }` を構築する。
    // throwOnError を false にすることで、LaTeX のパースエラーをパニックではなく
    // 赤文字エラーとして表示させる（開発中のデバッグがしやすい）。
    let options = js_sys::Object::new();
    js_sys::Reflect::set(
        &options,
        &JsValue::from_str("displayMode"),
        &JsValue::from_bool(display_mode),
    )
    .unwrap_or_default();
    js_sys::Reflect::set(
        &options,
        &JsValue::from_str("throwOnError"),
        &JsValue::from_bool(false),
    )
    .unwrap_or_default();

    render_to_string_js(latex, &options.into())
}

/// 日本語と LaTeX が混在する文字列をレンダリングする。
///
/// 日本語文字の連続はプレーンテキスト（HTML エスケープ済み）として、
/// それ以外の部分は KaTeX でインライン数式としてレンダリングする。
pub fn render_mixed(text: &str) -> String {
    let mut result = String::new();
    let mut segment = String::new();
    let mut cur_jp: Option<bool> = None;

    for ch in text.chars() {
        let jp = is_japanese(ch);
        match cur_jp {
            None => {
                cur_jp = Some(jp);
                segment.push(ch);
            }
            Some(prev) if prev == jp => {
                segment.push(ch);
            }
            Some(prev) => {
                flush_segment(&mut result, &segment, prev);
                segment.clear();
                segment.push(ch);
                cur_jp = Some(jp);
            }
        }
    }

    if let Some(jp) = cur_jp {
        flush_segment(&mut result, &segment, jp);
    }

    result
}

/// セグメントを適切にレンダリングして `result` に追記する。
fn flush_segment(result: &mut String, segment: &str, is_jp: bool) {
    if segment.is_empty() {
        return;
    }
    if is_jp {
        // プレーンテキスト（HTML エスケープ）
        for ch in segment.chars() {
            match ch {
                '<' => result.push_str("&lt;"),
                '>' => result.push_str("&gt;"),
                '&' => result.push_str("&amp;"),
                _ => result.push(ch),
            }
        }
    } else {
        let trimmed = segment.trim();
        if trimmed.is_empty() {
            result.push_str(segment);
        } else {
            if segment.starts_with(' ') {
                result.push(' ');
            }
            result.push_str(&render_latex(trimmed, false));
            if segment.ends_with(' ') {
                result.push(' ');
            }
        }
    }
}

/// 日本語文字かどうかを判定する。
fn is_japanese(ch: char) -> bool {
    matches!(ch,
        '\u{3000}'..='\u{303F}'   // CJK 記号句読点（、。「」等）
        | '\u{3040}'..='\u{309F}' // ひらがな
        | '\u{30A0}'..='\u{30FF}' // カタカナ
        | '\u{4E00}'..='\u{9FFF}' // CJK 統合漢字
        | '\u{FF00}'..='\u{FFEF}' // 全角英数
    )
}
