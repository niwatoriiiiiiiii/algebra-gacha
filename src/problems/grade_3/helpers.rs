//! grade_3 内部ヘルパー関数。
//! 問題生成で使う LaTeX 整形・数値ユーティリティ。

use crate::problems::{Difficulty, Problem};

pub fn ultra_problem(
    instruction: String,
    question: String,
    answer: String,
    steps: Vec<String>,
) -> Problem {
    Problem {
        difficulty: Difficulty::Ultra,
        instruction,
        question_latex: question,
        answer_latex: answer,
        steps,
    }
}

/// 符号文字列を返す（正の場合 "+"、負の場合 "-"）。
pub fn sign_str(n: i32) -> &'static str {
    if n >= 0 { "+" } else { "-" }
}

/// `ax^2 + bx + c` の LaTeX を組み立てる。
pub fn format_quadratic(c2: i32, v2: &str, c1: i32, _v1: &str, c0: i32) -> String {
    let mut s = String::new();

    // x^2 の項
    if c2.abs() == 1 {
        if c2 < 0 {
            s.push_str(&format!("-{}^2", v2));
        } else {
            s.push_str(&format!("{}^2", v2));
        }
    } else {
        s.push_str(&format!("{}{}^2", c2, v2));
    }

    // x の項
    if c1 != 0 {
        if c1 > 0 {
            if c1 == 1 {
                s.push_str(&format!(" + {}", v2));
            } else {
                s.push_str(&format!(" + {}{}", c1, v2));
            }
        } else {
            if c1 == -1 {
                s.push_str(&format!(" - {}", v2));
            } else {
                s.push_str(&format!(" - {}{}", c1.abs(), v2));
            }
        }
    }

    // 定数項
    if c0 != 0 {
        if c0 > 0 {
            s.push_str(&format!(" + {}", c0));
        } else {
            s.push_str(&format!(" - {}", c0.abs()));
        }
    }

    s
}

/// 4次式の LaTeX を組み立てる。
pub fn format_poly4(c3: i32, c2: i32, c1: i32, c0: i32) -> String {
    let mut s = String::new();

    let push_term = |s: &mut String, coeff: i32, var: &str| {
        if coeff == 0 {
            return;
        }
        let sign = if s.is_empty() {
            if coeff < 0 { "-" } else { "" }
        } else {
            if coeff > 0 { " + " } else { " - " }
        };
        let abs_c = coeff.abs();
        let c_str = if abs_c == 1 && !var.is_empty() {
            String::new()
        } else {
            format!("{}", abs_c)
        };
        s.push_str(&format!("{}{}{}", sign, c_str, var));
    };

    push_term(&mut s, c3, "x^3");
    push_term(&mut s, c2, "x^2");
    push_term(&mut s, c1, "x");
    push_term(&mut s, c0, "");

    if s.is_empty() { "0".to_string() } else { s }
}

/// 完全展開（x^2, xy, y^2, x, y, const）の LaTeX を組み立てる。
pub fn format_full_expansion(c_x2: i32, c_xy: i32, c_y2: i32, c_x: i32, c_y: i32, c0: i32) -> String {
    let mut s = String::new();

    let push = |s: &mut String, coeff: i32, var: &str| {
        if coeff == 0 {
            return;
        }
        let is_first = s.is_empty();
        let sign = if is_first {
            if coeff < 0 { "-" } else { "" }
        } else {
            if coeff > 0 { " + " } else { " - " }
        };
        let abs_c = coeff.abs();
        let c_str = if abs_c == 1 && !var.is_empty() {
            String::new()
        } else {
            format!("{}", abs_c)
        };
        s.push_str(&format!("{}{}{}", sign, c_str, var));
    };

    push(&mut s, c_x2, "x^2");
    push(&mut s, c_xy, "xy");
    push(&mut s, c_y2, "y^2");
    push(&mut s, c_x, "x");
    push(&mut s, c_y, "y");
    push(&mut s, c0, "");

    if s.is_empty() { "0".to_string() } else { s }
}

/// 1次式 `ax + b` の LaTeX を組み立てる。
pub fn format_linear(a: i32, b: i32, var: &str) -> String {
    let x_part = if a == 1 {
        var.to_string()
    } else if a == -1 {
        format!("-{}", var)
    } else {
        format!("{}{}", a, var)
    };
    if b == 0 {
        x_part
    } else if b > 0 {
        format!("{} + {}", x_part, b)
    } else {
        format!("{} - {}", x_part, b.abs())
    }
}

/// 最大公約数
pub fn gcd_i(mut a: i32, mut b: i32) -> i32 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a.max(1)
}

/// 小数文字列を (分子, 分母) に変換する（"0.3" → (3, 10)）。
pub fn parse_decimal(s: &str) -> (i32, i32) {
    if let Some(pos) = s.find('.') {
        let decimal_places = s.len() - pos - 1;
        let den = 10i32.pow(decimal_places as u32);
        let num: i32 = s.replace('.', "").parse().unwrap_or(1);
        let g = gcd_i(num, den);
        (num / g, den / g)
    } else {
        (s.parse().unwrap_or(1), 1)
    }
}

// ---- テスト ------------------------------------------------------------------



// ============================================================================
// 追加: 初級パターン
// ============================================================================


pub fn format_poly3(c2: i32, c1: i32, c0: i32) -> String {
    let mut parts: Vec<String> = Vec::new();
    if c2 != 0 {
        let s = if c2 == 1 { "x^2".to_string() } else { format!("{}x^2", c2) };
        parts.push(s);
    }
    if c1 != 0 {
        let abs = c1.abs();
        let s = if abs == 1 { "x".to_string() } else { format!("{}x", abs) };
        if parts.is_empty() {
            parts.push(if c1 < 0 { format!("-{}", s) } else { s });
        } else {
            parts.push(if c1 < 0 { format!("- {}", s) } else { format!("+ {}", s) });
        }
    }
    if c0 != 0 {
        if parts.is_empty() {
            parts.push(format!("{}", c0));
        } else {
            parts.push(if c0 < 0 { format!("- {}", c0.abs()) } else { format!("+ {}", c0) });
        }
    }
    if parts.is_empty() { "0".to_string() } else { parts.join(" ") }
}

/// ax^3 + bx^2 + cx + d の LaTeX
pub fn format_poly4_full(c3: i32, c2: i32, c1: i32, c0: i32) -> String {
    let mut parts: Vec<String> = Vec::new();
    let terms = [(c3, "x^3"), (c2, "x^2"), (c1, "x"), (c0, "")];
    for (coef, var) in &terms {
        if *coef == 0 { continue; }
        let abs = coef.abs();
        let s = if var.is_empty() {
            format!("{}", abs)
        } else if abs == 1 {
            var.to_string()
        } else {
            format!("{}{}", abs, var)
        };
        if parts.is_empty() {
            parts.push(if *coef < 0 { format!("-{}", s) } else { s });
        } else {
            parts.push(if *coef < 0 { format!("- {}", s) } else { format!("+ {}", s) });
        }
    }
    if parts.is_empty() { "0".to_string() } else { parts.join(" ") }
}
