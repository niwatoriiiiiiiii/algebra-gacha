//! 1年生の問題生成モジュール。
//!
//! 出題範囲: 文字と式（分配法則）
//! 難易度: 初級（Easy）のみ
//!
//! アルゴリズム方針:
//!   答えを先にランダムに決めてから、逆算で問題を組み立てる。
//!   これにより「整数にならないパターン」を防ぎ、
//!   常に綺麗な答えの問題を生成できる。

use rand::Rng;

use super::{Difficulty, Problem, coeff_str, latex_frac, pick};

// ---- 公開 API ----------------------------------------------------------------

/// 1年生の問題をランダムに生成して返す。
///
/// 現在は初級パターンのみ実装。
pub fn generate(rng: &mut impl Rng) -> Problem {
    match rng.gen_range(0..3u8) {
        0 => generate_easy_distributive(rng),   // 分数係数の分配法則
        1 => generate_easy_integer(rng),         // 整数係数の分配法則
        _ => generate_easy_triple(rng),          // 3項の分配法則
    }
}

// ---- 初級: 分配法則（係数が分数）--------------------------------------------
//
// 問題形式:
//   \frac{1}{p}(ax + b) - \frac{1}{q}(cx + d)
//
// 生成方針:
//   1. 答えの x 係数 `kx` と定数 `kc` をランダムに選ぶ
//   2. 分母 p, q をランダムに選ぶ
//   3. `ax = kx * p + cx_q * p` になるよう逆算（整数にならないケースはリトライ）
//      ※ 簡略化のため、ax, b, cx, d を直接ランダムに選び答えを計算する方法を採用

fn generate_easy_distributive(rng: &mut impl Rng) -> Problem {
    // 分母の候補
    let denoms: &[i32] = &[2, 3, 4, 5, 6];

    loop {
        let p = pick(rng, denoms); // 第1項の分母
        let q = pick(rng, denoms); // 第2項の分母

        // 第1かっこの中: (a*x + b*p) / p になるように生成するのではなく、
        // (A x + B) と見せかけて展開すると A/p x + B/p になる。
        // ここでの a, b, c, d は「かっこの中の見た目の整数」である。

        // 仕様変更: かっこの中の数値は必ずしも p, q の倍数ではないようにする
        // これによって答えが分数になる。

        let a = pick(rng, &[1i32, 2, 3, 4, 5]);
        let b = pick(rng, &[-5i32, -4, -3, -2, -1, 1, 2, 3, 4, 5]);

        let c = pick(rng, &[1i32, 2, 3, 4, 5]);
        let d = pick(rng, &[-5i32, -4, -3, -2, -1, 1, 2, 3, 4, 5]);

        // (1/p)(ax + b) - (1/q)(cx + d)
        // = (a/p)x + (b/p) - (c/q)x - (d/q)
        // = (aq - cp)/(pq) x + (bq - dp)/(pq)

        // ユーザー報告の問題: 1/2(2x+1) - 1/3(4x+4)
        // a=2, b=1, p=2, c=4, d=4, q=3
        // aq - cp = 2*3 - 4*2 = -2
        // pq = 6 -> -2/6 = -1/3 x (正しい)

        let x_num = a * q - c * p;
        let x_den = p * q;
        let c_num = b * q - d * p;
        let c_den = p * q;

        // 約分
        let gx = gcd(x_num.abs(), x_den.abs());
        let gc = gcd(c_num.abs(), c_den.abs());

        // 符号を分子に寄せる
        let mut x_num_r = x_num / gx;
        let mut x_den_r = x_den / gx;
        if x_den_r < 0 {
            x_num_r = -x_num_r;
            x_den_r = -x_den_r;
        }

        let mut c_num_r = c_num / gc;
        let mut c_den_r = c_den / gc;
        if c_den_r < 0 {
            c_num_r = -c_num_r;
            c_den_r = -c_den_r;
        }

        // 答えが 0 = x 係数がゼロはスキップ
        if x_num_r == 0 {
            continue;
        }

        // 問題文 LaTeX を構築
        let frac1 = format!("\\frac{{1}}{{{}}}", p);
        let ax_str = coeff_str(a, "x");
        let bp_str = b;
        let inner1_clean = if bp_str >= 0 {
            format!("{} + {}", ax_str, bp_str)
        } else {
            format!("{} - {}", ax_str, bp_str.abs())
        };

        let frac2 = format!("\\frac{{1}}{{{}}}", q);
        let cx_str = coeff_str(c, "x");
        let dq_str = d;
        let inner2_clean = if dq_str >= 0 {
            format!("{} + {}", cx_str, dq_str)
        } else {
            format!("{} - {}", cx_str, dq_str.abs())
        };

        let instruction = "次の計算をしなさい。".to_string();
        let question = format!("{}({}) - {}({})", frac1, inner1_clean, frac2, inner2_clean);

        // 答え LaTeX
        let answer = build_linear_answer(x_num_r, x_den_r, c_num_r, c_den_r);

        // 解説ステップ
        // step1_x1: a/p（x の係数）, step1_c1: b/p（定数項）— 第1項の展開結果
        // step1_x2: c/q（x の係数）, step1_c2: d/q（定数項）— 第2項の展開結果
        let step1_x1 = latex_frac(a, p);
        let step1_c1 = latex_frac(b, p);
        let step1_x2 = latex_frac(c, q);

        let steps = vec![
            // 1. 分配後の式を展開して並べる
            //   (1/p)(ax+b) - (1/q)(cx+d)
            //   = step1_x1·x + step1_c1 - step1_x2·x - step1_c2
            format!(
                "各かっこに分数を分配する: {}x {} - {}x {}",
                step1_x1,
                signed_const_signed_frac(b, p), // +step1_c1 相当
                step1_x2,
                signed_const_signed_frac(-d, q) // -step1_c2 相当: dのマイナスを適用する
            ),
            // 2. x の係数同士をまとめる
            format!(
                "x の係数をまとめる: \\left({} {}\\right)x",
                step1_x1,
                signed_const_signed_frac(-c, q),
            ),
            // 3. 定数項同士をまとめる
            format!(
                "定数項をまとめる: {} {} → 答え: {}",
                step1_c1,
                signed_const_signed_frac(-d, q),
                answer
            ),
        ];

        return Problem {
            difficulty: Difficulty::Easy,
            instruction,
            question_latex: question,
            answer_latex: answer,
            steps,
        };
    }
}

// ---- ヘルパー ----------------------------------------------------------------

/// ax + b の LaTeX 文字列を組み立てる（分数係数対応）。
fn build_linear_answer(x_num: i32, x_den: i32, c_num: i32, c_den: i32) -> String {
    let x_part = if x_den == 1 {
        coeff_str(x_num, "x")
    } else if x_num < 0 {
        format!("-\\frac{{{}}}{{{}}}x", x_num.abs(), x_den)
    } else {
        format!("\\frac{{{}}}{{{}}}x", x_num, x_den)
    };

    if c_num == 0 {
        return x_part;
    }

    let c_part = if c_den == 1 {
        format!("{}", c_num.abs())
    } else {
        format!("\\frac{{{}}}{{{}}}", c_num.abs(), c_den)
    };

    if c_num > 0 {
        if x_part.is_empty() {
            c_part
        } else {
            format!("{} + {}", x_part, c_part)
        }
    } else {
        if x_part.is_empty() {
            format!("-{}", c_part)
        } else {
            format!("{} - {}", x_part, c_part)
        }
    }
}

/// 分数 `num/den` を符号付き LaTeX 文字列として返す（ステップ説明用）。
///
/// 例: `signed_const_signed_frac(3, 4)`  → `"+ \\frac{3}{4}"`
///     `signed_const_signed_frac(-3, 4)` → `"- \\frac{3}{4}"`
///     `signed_const_signed_frac(6, 3)`  → `"+ 2"`（約分）
fn signed_const_signed_frac(num: i32, den: i32) -> String {
    let g = gcd(num.abs(), den);
    let n = num / g;
    let d = den / g;
    if n >= 0 {
        format!("+ {}", latex_frac(n, d))
    } else {
        format!("- {}", latex_frac(n.abs(), d))
    }
}

/// 最大公約数（ユークリッド互除法）。
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a.max(1) // 0 除算を防ぐ
}

// ---- テスト ------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[test]
    fn generates_without_panic() {
        let mut rng = StdRng::seed_from_u64(42);
        for _ in 0..100 {
            let p = generate(&mut rng);
            assert!(!p.question_latex.is_empty());
            assert!(!p.answer_latex.is_empty());
            assert_eq!(p.difficulty, Difficulty::Easy);
        }
    }
}

// ---- 初級: 整数係数の分配法則（2パターン目）----------------------------------
//
// 問題形式: A(ax + b) - B(cx + d)
// 係数はすべて整数で、計算力を見る基本問題

pub fn generate_easy_integer(rng: &mut impl Rng) -> Problem {
    let outer: &[i32] = &[2, 3, 4, 5, 6];
    let inner_coeffs: &[i32] = &[1, 2, 3, 4, 5];
    let consts: &[i32] = &[-6, -5, -4, -3, -2, -1, 1, 2, 3, 4, 5, 6];

    let cap_a = pick(rng, outer);
    let cap_b = pick(rng, outer);
    let a = pick(rng, inner_coeffs);
    let b = pick(rng, consts);
    let c = pick(rng, inner_coeffs);
    let d = pick(rng, consts);

    // A(ax+b) - B(cx+d) = (Aa-Bc)x + (Ab-Bd)
    let x_coeff = cap_a * a - cap_b * c;
    let const_term = cap_a * b - cap_b * d;

    // x係数が0はスキップ
    if x_coeff == 0 {
        return generate_easy_integer(rng);
    }

    let instruction = "次の計算をしなさい。".to_string();
    let inner1 = if b >= 0 {
        format!("{}x + {}", a, b)
    } else {
        format!("{}x - {}", a, b.abs())
    };
    let inner2 = if d >= 0 {
        format!("{}x + {}", c, d)
    } else {
        format!("{}x - {}", c, d.abs())
    };
    let question = format!("{}({}) - {}({})", cap_a, inner1, cap_b, inner2);

    let answer = if const_term == 0 {
        super::coeff_str(x_coeff, "x")
    } else if const_term > 0 {
        format!("{} + {}", super::coeff_str(x_coeff, "x"), const_term)
    } else {
        format!("{} - {}", super::coeff_str(x_coeff, "x"), const_term.abs())
    };

    let steps = vec![
        format!("分配する: {}・{}x + {}・{} - {}・{}x - {}・{}",
            cap_a, a, cap_a, b, cap_b, c, cap_b, d),
        format!("xの係数: {}・{} - {}・{} = {}", cap_a, a, cap_b, c, x_coeff),
        format!("定数項: {}・{} - {}・{} = {}", cap_a, b, cap_b, d, const_term),
        format!("答え: {}", answer),
    ];

    Problem {
        difficulty: super::Difficulty::Easy,
        instruction,
        question_latex: question,
        answer_latex: answer,
        steps,
    }
}

// ---- 初級: 3項の分配法則（3パターン目）----------------------------------------
//
// 問題形式: A(ax + b) + B(cx + d) + C(ex + f)

pub fn generate_easy_triple(rng: &mut impl Rng) -> Problem {
    let outer: &[i32] = &[2, 3, 4, 5];
    let inner_coeffs: &[i32] = &[1, 2, 3];
    let consts: &[i32] = &[-4, -3, -2, -1, 1, 2, 3, 4];
    let signs: &[i32] = &[1, -1];

    let cap_a = pick(rng, outer);
    let cap_b = pick(rng, outer);
    let cap_c = pick(rng, outer);
    let s1 = pick(rng, signs);
    let s2 = pick(rng, signs);
    let a = pick(rng, inner_coeffs);
    let b = pick(rng, consts);
    let c = pick(rng, inner_coeffs);
    let d = pick(rng, consts);
    let e = pick(rng, inner_coeffs);
    let f = pick(rng, consts);

    // A(ax+b) ±s1 B(cx+d) ±s2 C(ex+f)
    let x_coeff = cap_a * a + s1 * cap_b * c + s2 * cap_c * e;
    let const_term = cap_a * b + s1 * cap_b * d + s2 * cap_c * f;

    if x_coeff == 0 {
        return generate_easy_triple(rng);
    }

    let sign_str1 = if s1 > 0 { "+" } else { "-" };
    let sign_str2 = if s2 > 0 { "+" } else { "-" };

    let fmt_inner = |coeff: i32, con: i32| -> String {
        if con >= 0 { format!("{}x + {}", coeff, con) }
        else { format!("{}x - {}", coeff, con.abs()) }
    };

    let instruction = "次の計算をしなさい。".to_string();
    let question = format!("{}({}) {} {}({}) {} {}({})",
        cap_a, fmt_inner(a, b),
        sign_str1, cap_b, fmt_inner(c, d),
        sign_str2, cap_c, fmt_inner(e, f)
    );

    let answer = if const_term == 0 {
        super::coeff_str(x_coeff, "x")
    } else if const_term > 0 {
        format!("{} + {}", super::coeff_str(x_coeff, "x"), const_term)
    } else {
        format!("{} - {}", super::coeff_str(x_coeff, "x"), const_term.abs())
    };

    let steps = vec![
        format!("各かっこを展開する"),
        format!("xの係数: {} {} {} {} {} = {}",
            cap_a * a,
            if s1 > 0 { "+" } else { "-" },
            cap_b * c,
            if s2 > 0 { "+" } else { "-" },
            cap_c * e,
            x_coeff),
        format!("定数項: {} {} {} {} {} = {}",
            cap_a * b,
            if s1 * d >= 0 { "+" } else { "-" },
            (s1 * cap_b * d).abs(),
            if s2 * f >= 0 { "+" } else { "-" },
            (s2 * cap_c * f).abs(),
            const_term),
        format!("答え: {}", answer),
    ];

    Problem {
        difficulty: super::Difficulty::Easy,
        instruction,
        question_latex: question,
        answer_latex: answer,
        steps,
    }
}
