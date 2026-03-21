//! 2年生の問題生成モジュール。
//!
//! 出題範囲: 式の計算（累乗・除算・式の値）
//! 難易度: 初級（Easy）・中級（Medium）

use rand::Rng;

use super::{Difficulty, Problem, pick};

// ---- 公開 API ----------------------------------------------------------------

/// 指定難易度の2年生問題を生成して返す。
pub fn generate(rng: &mut impl Rng, difficulty: &Difficulty) -> Problem {
    match difficulty {
        Difficulty::Easy => match rng.gen_range(0..3u8) {
            0 => generate_easy_power(rng),
            1 => generate_easy_expression_value(rng),
            _ => generate_easy_power_frac(rng),         // 新: 分数係数の累乗
        },
        Difficulty::Medium => match rng.gen_range(0..3u8) {
            0 => generate_medium_power_div(rng),
            1 => generate_medium_expression_value(rng),
            _ => generate_medium_power_chain(rng),       // 新: 累乗の連鎖
        },
        _ => generate_easy_power(rng),
    }
}

// ---- 初級: 累乗の計算 --------------------------------------------------------
//
// 問題形式: `(Axy^m)^2 × Bxy` の形を中心に出題
//
// 生成方針: A, B, m をランダムに選んで計算する

fn generate_easy_power(rng: &mut impl Rng) -> Problem {
    // 係数の候補
    let coeffs: &[i32] = &[2, 3, 4, 5];
    let a = pick(rng, coeffs);
    let b_sign: i32 = if rng.gen_bool(0.5) { 1 } else { -1 };
    let b = b_sign * pick(rng, coeffs);
    let m: u32 = pick(rng, &[2u32, 3]);

    // (Ax y^m)^2 × Bxy の計算
    // = A^2 x^2 y^{2m} × Bxy
    // = A^2 B x^3 y^{2m+1}
    let ans_coeff = a * a * b;
    let ans_x_exp = 3u32;
    let ans_y_exp = 2 * m + 1;

    let instruction = "次の計算をしなさい。".to_string();
    let question = format!("({}xy^{{{}}})^2 \\times {}xy", a, m, b);

    // 答え: 係数が負の場合を正しく LaTeX 化
    let answer = format_monomial(ans_coeff, "x", ans_x_exp, "y", ans_y_exp);

    let steps = vec![
        format!(
            "累乗を展開する: {}^2 x^2 y^{{{}}} \\times {}xy",
            a,
            2 * m,
            b
        ),
        format!("係数をかける: {} \\times {} = {}", a * a, b, ans_coeff),
        format!(
            "指数をたす: x^{{2+1}} = x^{{{}}}, \\; y^{{{}+1}} = y^{{{}}}",
            ans_x_exp,
            2 * m,
            ans_y_exp
        ),
        format!("答え: {}", answer),
    ];

    Problem {
        difficulty: Difficulty::Easy,
        instruction,
        question_latex: question,
        answer_latex: answer,
        steps,
    }
}

// ---- 初級: 式の値 ------------------------------------------------------------
//
// 問題形式: `a=p, b=q のとき A(Ba + Cb) - (Da - Eb) の値`

fn generate_easy_expression_value(rng: &mut impl Rng) -> Problem {
    let coeffs: &[i32] = &[2, 3, 4, 5];
    let vals: &[i32] = &[-3, -2, -1, 1, 2, 3];

    let a_val = pick(rng, vals);
    let b_val = pick(rng, vals);

    // A(Ba + Cb) - (Da - Eb)
    // = ABa + ACb - Da + Eb
    // = (AB - D)a + (AC + E)b
    let cap_a = pick(rng, coeffs);
    let cap_b = pick(rng, coeffs);
    let cap_c = pick(rng, coeffs);
    let cap_d = pick(rng, coeffs);
    let cap_e = pick(rng, coeffs);

    let simplified_a_coeff = cap_a * cap_b - cap_d;
    let simplified_b_coeff = cap_a * cap_c + cap_e;

    let answer_val = simplified_a_coeff * a_val + simplified_b_coeff * b_val;

    let instruction = format!("a={}, b={} のとき, 次の式の値を求めなさい。", a_val, b_val);
    let question = format!(
        "{}({}a + {}b) - ({}a - {}b)",
        cap_a, cap_b, cap_c, cap_d, cap_e
    );

    let simplified = format!(
        "{}a {} {}b",
        simplified_a_coeff,
        if simplified_b_coeff >= 0 { "+" } else { "-" },
        simplified_b_coeff.abs()
    );

    let steps = vec![
        format!(
            "分配法則で展開する: {}a + {}b - {}a + {}b",
            cap_a * cap_b,
            cap_a * cap_c,
            cap_d,
            cap_e
        ),
        format!("同類項をまとめる: {}", simplified),
        format!(
            "a={}, b={} を代入する: {} \\times ({}) + {} \\times ({})",
            a_val, b_val, simplified_a_coeff, a_val, simplified_b_coeff, b_val
        ),
        format!("計算して答え: {}", answer_val),
    ];

    Problem {
        difficulty: Difficulty::Easy,
        instruction,
        question_latex: question,
        answer_latex: format!("{}", answer_val),
        steps,
    }
}

// ---- 中級: 累乗と除算の混合 --------------------------------------------------
//
// 問題形式: `(-\frac{1}{A} x^m)^2 ÷ \frac{1}{B} x^n`
// = (1/A^2) x^{2m} × B x^{-n}
// = B/A^2 × x^{2m-n}

fn generate_medium_power_div(rng: &mut impl Rng) -> Problem {
    let denoms: &[i32] = &[2, 3, 4];
    let exps: &[u32] = &[2u32, 3];

    let cap_a = pick(rng, denoms); // 累乗の分母
    let cap_b = pick(rng, denoms); // 除数の分母
    let m = pick(rng, exps);
    let n = pick(rng, &[1u32, 2]);

    // (-1/A * x^m)^2 = (1/A^2) x^{2m}
    // ÷ (1/B x^n) = × B x^{-n}
    // = B/A^2 × x^{2m - n}
    let ans_x_exp = 2 * m as i32 - n as i32;
    let ans_num = cap_b;
    let ans_den = cap_a * cap_a;

    // 答えの x 指数が 0 や負にならないようリトライ
    if ans_x_exp <= 0 {
        return generate_medium_power_div(rng);
    }

    let g = gcd(ans_num, ans_den);
    let ans_num_r = ans_num / g;
    let ans_den_r = ans_den / g;

    let instruction = "次の計算をしなさい。".to_string();
    let question = format!(
        "\\left(-\\frac{{1}}{{{}}}x^{{{}}}\\right)^2 \\div \\frac{{1}}{{{}}}x^{{{}}}",
        cap_a, m, cap_b, n
    );

    let answer = if ans_den_r == 1 {
        format!("{}x^{{{}}}", ans_num_r, ans_x_exp)
    } else {
        format!(
            "\\frac{{{}}}{{{}}}x^{{{}}}",
            ans_num_r, ans_den_r, ans_x_exp
        )
    };

    let steps = vec![
        format!(
            "累乗を展開する: \\frac{{1}}{{{}}}x^{{{}}}",
            cap_a * cap_a,
            2 * m
        ),
        format!(
            "除算を乗算に変換: \\frac{{1}}{{{}}}x^{{{}}} \\times {}x^{{-{}}}",
            cap_a * cap_a,
            2 * m,
            cap_b,
            n
        ),
        format!("係数をかける: \\frac{{{}}}{{{}}}", cap_b, cap_a * cap_a),
        format!("指数をたす: x^{{{} - {}}} = x^{{{}}}", 2 * m, n, ans_x_exp),
        format!("約分して答え: {}", answer),
    ];

    Problem {
        difficulty: Difficulty::Medium,
        instruction,
        question_latex: question,
        answer_latex: answer,
        steps,
    }
}

// ---- 中級: 式の値（複合）-----------------------------------------------------

fn generate_medium_expression_value(rng: &mut impl Rng) -> Problem {
    let coeffs: &[i32] = &[2, 3, 4, 6, 8];
    // a, b は小さな値
    let vals: &[i32] = &[-3, -2, -1, 2, 3];

    let a_val = pick(rng, vals);
    let b_val = pick(rng, vals);
    if a_val == 0 || b_val == 0 {
        return generate_medium_expression_value(rng);
    }

    // Aa^2 b ÷ Bab × Cb
    // = A*C/B × a^2 b^2 / (ab) = A*C/B × ab
    let cap_a = pick(rng, coeffs);
    let cap_c = pick(rng, coeffs);

    // A*C/B が整数になるよう cap_b を cap_a*cap_c の約数から選ぶ
    let product = cap_a * cap_c;
    // cap_b は product の約数
    let divisors: Vec<i32> = (1..=product).filter(|d| product % d == 0).collect();
    if divisors.is_empty() {
        return generate_medium_expression_value(rng);
    }
    let cap_b_actual = pick(rng, &divisors);

    let coeff = product / cap_b_actual; // 整数
    let answer_val = coeff * a_val * b_val;

    let instruction = format!("a={}, b={} のとき, 次の式の値を求めなさい。", a_val, b_val);
    let question = format!("{}a^2b \\div {}ab \\times {}b", cap_a, cap_b_actual, cap_c);

    let steps = vec![
        format!(
            "乗除を左から順に: {}a^2b \\times \\frac{{1}}{{{}ab}} \\times {}b",
            cap_a, cap_b_actual, cap_c
        ),
        format!(
            "係数をまとめる: \\frac{{{}}}{{{}}} = {}",
            product, cap_b_actual, coeff
        ),
        format!("文字をまとめる: {} \\times ab", coeff),
        format!(
            "a={}, b={} を代入: {} \\times ({}) \\times ({}) = {}",
            a_val, b_val, coeff, a_val, b_val, answer_val
        ),
    ];

    Problem {
        difficulty: Difficulty::Medium,
        instruction,
        question_latex: question,
        answer_latex: format!("{}", answer_val),
        steps,
    }
}

// ---- ヘルパー ----------------------------------------------------------------

/// 単項式の LaTeX 文字列を作る。
/// 例: `format_monomial(6, "x", 3, "y", 5)` → `"6x^{3}y^{5}"`
fn format_monomial(coeff: i32, var1: &str, exp1: u32, var2: &str, exp2: u32) -> String {
    let sign = if coeff < 0 { "-" } else { "" };
    let abs_coeff = coeff.abs();
    let c_str = if abs_coeff == 1 {
        String::new()
    } else {
        format!("{}", abs_coeff)
    };
    let v1 = if exp1 == 1 {
        var1.to_string()
    } else {
        format!("{}^{{{}}}", var1, exp1)
    };
    let v2 = if exp2 == 1 {
        var2.to_string()
    } else {
        format!("{}^{{{}}}", var2, exp2)
    };
    format!("{}{}{}{}", sign, c_str, v1, v2)
}

/// 最大公約数
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a.max(1)
}

// ---- テスト ------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[test]
    fn generates_easy_without_panic() {
        let mut rng = StdRng::seed_from_u64(123);
        for _ in 0..100 {
            let p = generate(&mut rng, &Difficulty::Easy);
            assert!(!p.question_latex.is_empty());
        }
    }

    #[test]
    fn generates_medium_without_panic() {
        let mut rng = StdRng::seed_from_u64(456);
        for _ in 0..100 {
            let p = generate(&mut rng, &Difficulty::Medium);
            assert!(!p.question_latex.is_empty());
        }
    }
}

// ---- 初級(新): 分数係数の累乗 -----------------------------------------------
//
// 問題形式: (-\frac{a}{b} x^m y^n)^2
// 平方なので符号が消え、係数は (a/b)^2 になる

fn generate_easy_power_frac(rng: &mut impl Rng) -> Problem {
    let nums: &[i32] = &[1, 2, 3];
    let dens: &[i32] = &[2, 3, 4, 5];
    let exps: &[u32] = &[1, 2, 3];

    let a = pick(rng, nums); // 分子
    let b = pick(rng, dens); // 分母（a と互いに素でなくても約分する）
    let m = pick(rng, exps); // x の指数
    let n = pick(rng, exps); // y の指数

    // (-a/b * x^m * y^n)^2 = (a^2/b^2) x^{2m} y^{2n}
    let ans_num = a * a;
    let ans_den = b * b;
    let g = gcd(ans_num, ans_den);
    let ans_num_r = ans_num / g;
    let ans_den_r = ans_den / g;

    let x_exp = 2 * m;
    let y_exp = 2 * n;

    let instruction = "次の計算をしなさい。".to_string();
    let x_part = if m == 1 { "x".to_string() } else { format!("x^{{{}}}", m) };
    let y_part = if n == 1 { "y".to_string() } else { format!("y^{{{}}}", n) };
    let coeff_str = if a == 1 { String::new() } else { format!("{}", a) };
    let question = format!(
        "\\left(-\\frac{{{}}}{{{}}}{}{}\\right)^2",
        coeff_str.clone() + if a == 1 { "1" } else { "" }, b, x_part, y_part
    );

    let ans_coeff = if ans_den_r == 1 {
        format!("{}", ans_num_r)
    } else {
        format!("\\frac{{{}}}{{{}}}", ans_num_r, ans_den_r)
    };
    let ans_x = if x_exp == 1 { "x".to_string() } else { format!("x^{{{}}}", x_exp) };
    let ans_y = if y_exp == 1 { "y".to_string() } else { format!("y^{{{}}}", y_exp) };
    let answer = format!("{}{}{}", ans_coeff, ans_x, ans_y);

    let steps = vec![
        format!("符号: (-...)^2 なので正になる"),
        format!("係数: \\left(\\frac{{{}}}{{{}}}\\right)^2 = \\frac{{{}}}{{{}}}",
            a, b, a*a, b*b),
        format!("指数: x^{{{}\\times 2}} = x^{{{}}}, \\; y^{{{}\\times 2}} = y^{{{}}}",
            m, x_exp, n, y_exp),
        format!("答え: {}", answer),
    ];

    Problem {
        difficulty: Difficulty::Easy,
        instruction,
        question_latex: question,
        answer_latex: answer,
        steps,
    }
}

// ---- 中級(新): 累乗の連鎖 ---------------------------------------------------
//
// 問題形式: (a^m b^n)^p ÷ (a^r b^s)
// 指数法則を2回使う問題

fn generate_medium_power_chain(rng: &mut impl Rng) -> Problem {
    let coeffs: &[i32] = &[2, 3, 4, 5];
    let outer_exps: &[u32] = &[2, 3];
    let inner_exps: &[u32] = &[1, 2];
    let div_exps: &[u32] = &[1, 2, 3];

    let a = pick(rng, coeffs);
    let b = pick(rng, coeffs);
    let m = pick(rng, inner_exps); // x の指数（累乗の中）
    let n = pick(rng, inner_exps); // y の指数
    let p = pick(rng, outer_exps); // 外の指数
    let r = pick(rng, div_exps);   // 除数の x 指数
    let s = pick(rng, div_exps);   // 除数の y 指数

    // (a x^m y^n)^p ÷ (b x^r y^s)
    // = a^p x^{mp} y^{np} ÷ b x^r y^s
    // = (a^p / b) x^{mp-r} y^{np-s}

    let x_exp = m as i32 * p as i32 - r as i32;
    let y_exp = n as i32 * p as i32 - s as i32;

    // 答えの指数が負にならないようリトライ
    if x_exp <= 0 || y_exp <= 0 {
        return generate_medium_power_chain(rng);
    }

    let num = i32::pow(a, p);
    let den = b;
    let g = gcd(num, den);
    let num_r = num / g;
    let den_r = den / g;

    let instruction = "次の計算をしなさい。".to_string();
    let x_in = if m == 1 { "x".to_string() } else { format!("x^{{{}}}", m) };
    let y_in = if n == 1 { "y".to_string() } else { format!("y^{{{}}}", n) };
    let x_div = if r == 1 { "x".to_string() } else { format!("x^{{{}}}", r) };
    let y_div = if s == 1 { "y".to_string() } else { format!("y^{{{}}}", s) };
    let question = format!(
        "({}{}{}){} \\div {}{}{}",
        a, x_in, y_in,
        if p == 1 { String::new() } else { format!("^{{{}}}", p) },
        b, x_div, y_div
    );

    let coeff_str = if den_r == 1 {
        format!("{}", num_r)
    } else {
        format!("\\frac{{{}}}{{{}}}", num_r, den_r)
    };
    let ans_x = if x_exp == 1 { "x".to_string() } else { format!("x^{{{}}}", x_exp) };
    let ans_y = if y_exp == 1 { "y".to_string() } else { format!("y^{{{}}}", y_exp) };
    let answer = format!("{}{}{}", coeff_str, ans_x, ans_y);

    let steps = vec![
        format!("累乗を展開: {}^{{{}}} x^{{{}\\times{}}} y^{{{}\\times{}}} ÷ {}{}{}",
            a, p, m, p, n, p, b, x_div, y_div),
        format!("= {} x^{{{}}} y^{{{}}} ÷ {}{}{}",
            i32::pow(a, p), m*p, n*p, b, x_div, y_div),
        format!("係数: {} ÷ {} = {}", i32::pow(a, p), b,
            if den_r == 1 { format!("{}", num_r) } else { format!("{}/{}", num_r, den_r) }),
        format!("指数: x^{{{}-{}}} = x^{{{}}}, y^{{{}-{}}} = y^{{{}}}",
            m*p, r, x_exp, n*p, s, y_exp),
        format!("答え: {}", answer),
    ];

    Problem {
        difficulty: Difficulty::Medium,
        instruction,
        question_latex: question,
        answer_latex: answer,
        steps,
    }
}
