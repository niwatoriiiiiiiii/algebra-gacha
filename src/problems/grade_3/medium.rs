//! 3年生 中級問題。
//!
//! パターン: 係数あり展開・分数係数・分数の差の積・置き換え・数値の工夫・式の値・3乗・対称式

use rand::Rng;

/// 中級パターンをランダムに選んで返す（全16パターン）。
pub fn generate_medium(rng: &mut impl Rng) -> Problem {
    match rng.gen_range(0..16u8) {
        0  => medium_xaxb_coeff(rng),
        1  => medium_square_frac(rng),
        2  => medium_diff_squares_frac(rng),
        3  => medium_substitution(),
        4  => medium_numerical_trick(rng),
        5  => medium_expression_value(rng),
        6  => medium_cube_expand(rng),
        7  => medium_symmetric_value(rng),
        8  => medium_three_var_square(rng),
        9  => medium_sqrt_sum_squares(rng),
        10  => medium_diff_two_squares_expand(rng),
        11  => medium_unit_difference(rng),
        12  => medium_consecutive_product(rng),
        13  => medium_fraction_expression(rng),
        14  => medium_substitution_constant2(rng),
        _  => medium_irrational_simplify(rng),
    }
}
use super::helpers::{format_poly4_full, format_quadratic, sign_str};
use crate::problems::{coeff_str, Difficulty, Problem, pick, random_sign};

fn medium_xaxb_coeff(rng: &mut impl Rng) -> Problem {
    let a = pick(rng, &[2i32, 3, 4]);
    let b = pick(rng, &[-3i32, -2, -1, 1, 2, 3]);
    let c = pick(rng, &[-3i32, -2, -1, 1, 2, 3]);

    // (Ax + b)(Ax + c) = A^2 x^2 + A(b+c)x + bc
    let sum = b + c;
    let prod = b * c;
    let c2 = a * a;
    let c1 = a * sum;

    let instruction = "次の計算をしなさい。".to_string();
    let question = format!(
        "({}a {} {})({}a {} {})",
        a,
        sign_str(b),
        b.abs(),
        a,
        sign_str(c),
        c.abs()
    );
    let answer = format_quadratic(c2, "a", c1, "a", prod);

    let steps = vec![
        format!("(x+b)(x+c) = x^2 + (b+c)x + bc で x = {}a", a),
        format!("({}a)^2 = {}a^2", a, c2),
        format!("({} + {}) \\times {}a = {}a", b, c, a, c1),
        format!("{} \\times {} = {}", b, c, prod),
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

/// `(x + \frac{1}{A}y)^2` 型（分数係数・平方根）
fn medium_square_frac(rng: &mut impl Rng) -> Problem {
    if rng.gen_bool(0.5) {
        // 分数係数パターン: (x + 1/A y)^2
        let a = pick(rng, &[2i32, 3, 4, 5]);
        let sign = random_sign(rng);

        // (x ± y/A)^2 = x^2 ± (2/A)xy + (1/A^2)y^2
        let instruction = "次の計算をしなさい。".to_string();
        let question = format!(
            "\\left(x {} \\frac{{1}}{{{}}}y\\right)^2",
            if sign > 0 { "+" } else { "-" },
            a
        );
        let middle_sign = if sign > 0 { "+" } else { "-" };
        let answer = format!(
            "x^2 {} \\frac{{2}}{{{}}}xy + \\frac{{1}}{{{}}}y^2",
            middle_sign,
            a,
            a * a
        );

        let steps = vec![
            format!(
                "(x \\pm a)^2 = x^2 \\pm 2ax + a^2 で a = \\frac{{1}}{{{}}}y",
                a
            ),
            format!("2 \\times \\frac{{1}}{{{}}}y = \\frac{{2}}{{{}}}y", a, a),
            format!(
                "\\left(\\frac{{1}}{{{}}}y\\right)^2 = \\frac{{1}}{{{}}}y^2",
                a,
                a * a
            ),
            format!("答え: {}", answer),
        ];

        return Problem {
            difficulty: Difficulty::Medium,
            instruction,
            question_latex: question,
            answer_latex: answer,
            steps,
        };
    }

    // 平方根の積: (A√B - √C)(A√B + √C) = A^2 B - C
    let cap_a = pick(rng, &[2i32, 3]);
    let cap_b = pick(rng, &[2u32, 3, 5]);
    let cap_c = pick(rng, &[2u32, 3, 5, 6, 7]);

    let answer_val = cap_a * cap_a * cap_b as i32 - cap_c as i32;

    let instruction = "次の計算をしなさい。".to_string();
    let question = format!(
        "({}\\sqrt{{{}}} - \\sqrt{{{}}})({}\\sqrt{{{}}} + \\sqrt{{{}}})",
        cap_a, cap_b, cap_c, cap_a, cap_b, cap_c
    );

    let steps = vec![
        format!(
            "(x-y)(x+y) = x^2 - y^2 で x = {}\\sqrt{{{}}}, y = \\sqrt{{{}}}",
            cap_a, cap_b, cap_c
        ),
        format!(
            "({}\\sqrt{{{}}})^2 = {} \\times {} = {}",
            cap_a,
            cap_b,
            cap_a * cap_a,
            cap_b,
            cap_a * cap_a * cap_b as i32
        ),
        format!("(\\sqrt{{{}}})^2 = {}", cap_c, cap_c),
        format!(
            "答え: {} - {} = {}",
            cap_a * cap_a * cap_b as i32,
            cap_c,
            answer_val
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

/// `(\frac{x}{A} + \frac{y}{B})(\frac{y}{B} - \frac{x}{A})` 型（分数係数・差の積）
fn medium_diff_squares_frac(rng: &mut impl Rng) -> Problem {
    let a = pick(rng, &[2i32, 3, 4]);
    let b = pick(rng, &[2i32, 3, 4, 5, 7]);

    // (x/A + y/B)(y/B - x/A) = (y/B)^2 - (x/A)^2 = y^2/B^2 - x^2/A^2
    let instruction = "次の計算をしなさい。".to_string();
    let question = format!(
        "\\left(\\frac{{x}}{{{}}} + \\frac{{y}}{{{}}}\\right)\\left(\\frac{{y}}{{{}}} - \\frac{{x}}{{{}}}\\right)",
        a, b, b, a
    );
    let answer = format!("\\frac{{y^2}}{{{}}} - \\frac{{x^2}}{{{}}}", b * b, a * a);

    let steps = vec![
        format!("(p+q)(q-p) = q^2 - p^2 と読み替える"),
        format!("p = \\frac{{x}}{{{}}}, q = \\frac{{y}}{{{}}}", a, b),
        format!("q^2 = \\frac{{y^2}}{{{}}}", b * b),
        format!("p^2 = \\frac{{x^2}}{{{}}}", a * a),
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

/// 置き換えによる展開（中級）: `(x + y - z)(x - y + z)`
fn medium_substitution() -> Problem {
    // (x + y - z)(x - y + z)
    // = (x + (y-z))(x - (y-z)) = x^2 - (y-z)^2
    // = x^2 - y^2 + 2yz - z^2
    let instruction = "次の計算をしなさい。".to_string();
    let question = "(x + y - z)(x - y + z)".to_string();
    let answer = "x^2 - y^2 + 2yz - z^2".to_string();

    let steps = vec![
        "A = y - z と置くと (x + A)(x - A) と読み替えられる".to_string(),
        "= x^2 - A^2 = x^2 - (y-z)^2".to_string(),
        "(y-z)^2 = y^2 - 2yz + z^2 を展開する".to_string(),
        "答え: x^2 - (y^2 - 2yz + z^2) = x^2 - y^2 + 2yz - z^2".to_string(),
    ];

    Problem {
        difficulty: Difficulty::Medium,
        instruction,
        question_latex: question,
        answer_latex: answer,
        steps,
    }
}

/// 計算の工夫（中級）: `N^2 - (N-1)(N+1)` = 1
fn medium_numerical_trick(rng: &mut impl Rng) -> Problem {
    let n = pick(rng, &[777i32, 888, 999, 1234, 2025, 3141]);

    // n^2 - (n-1)(n+1) = n^2 - (n^2 - 1) = 1
    let instruction = "次の計算をしなさい。".to_string();
    let question = format!("{}^2 - {} \\times {}", n, n - 1, n + 1);

    let steps = vec![
        format!(
            "{} \\times {} = (N-1)(N+1) = N^2 - 1 の形と見る（N = {}）",
            n - 1,
            n + 1,
            n
        ),
        format!("{}^2 - ({}^2 - 1)", n, n),
        format!("答え: 1"),
    ];

    Problem {
        difficulty: Difficulty::Medium,
        instruction,
        question_latex: question,
        answer_latex: "1".to_string(),
        steps,
    }
}

/// 式の値（中級）: `x=A, y=B のとき x(x + Cy) - (x-y)(x+Dy)`
fn medium_expression_value(rng: &mut impl Rng) -> Problem {
    let x_val = pick(rng, &[29i32, 31, 19, 21, 11, 49]);
    let y_val = pick(rng, &[3i32, 4, 5, 6, 7]);
    let c = pick(rng, &[3i32, 4, 5]);
    let d = pick(rng, &[3i32, 4, 5]);

    // x(x+cy) - (x-y)(x+dy)
    // = x^2 + cxy - (x^2 + (d-1)xy - dy^2)
    // = x^2 + cxy - x^2 - (d-1)xy + dy^2
    // = (c - d + 1)xy + dy^2
    let coeff_xy = c - d + 1;
    let coeff_y2 = d;
    let answer_val = coeff_xy * x_val * y_val + coeff_y2 * y_val * y_val;

    let instruction = format!("x={}, y={} のとき, 次の式の値を求めなさい。", x_val, y_val);
    let question = format!(
        "x(x + {}) - (x - y)(x + {})",
        coeff_str(c, "y"),
        coeff_str(d, "y")
    );

    let steps = vec![
        format!(
            "x^2 + {} - x^2 + {} - {} と展開",
            coeff_str(c, "xy"),
            coeff_str(d - 1, "xy"),
            coeff_str(d, "y^2")
        ),
        format!(
            "x^2 が消えて: ({} - {} + 1)xy + {}y^2 = {} + {}",
            c,
            d,
            d,
            coeff_str(coeff_xy, "xy"),
            coeff_str(coeff_y2, "y^2")
        ),
        format!(
            "x={}, y={} を代入: {} \\times {} \\times {} + {} \\times {}^2",
            x_val, y_val, coeff_xy, x_val, y_val, coeff_y2, y_val
        ),
        format!("答え: {}", answer_val),
    ];

    Problem {
        difficulty: Difficulty::Medium,
        instruction,
        question_latex: question,
        answer_latex: format!("{}", answer_val),
        steps,
    }
}

// ============================================================================
// 上級（Hard）

fn medium_cube_expand(rng: &mut impl Rng) -> Problem {
    let coeffs: &[i32] = &[1, 2, 3];
    let consts: &[i32] = &[1, 2, 3, 4];
    let sign = if rng.gen_bool(0.5) { 1i32 } else { -1 };

    let a = pick(rng, coeffs);
    let b = pick(rng, consts);

    // (ax ± b)^3 = a^3 x^3 ± 3a^2b x^2 + 3ab^2 x ± b^3
    let c3 = a * a * a;
    let c2 = sign * 3 * a * a * b;
    let c1 = 3 * a * b * b;
    let c0 = sign * b * b * b;

    let sign_str = if sign > 0 { "+" } else { "-" };
    let inner = format!("{}x {} {}", a, sign_str, b);
    let instruction = "次の計算をしなさい。".to_string();
    let question = format!("({})^3", inner);

    let answer = format_poly4_full(c3, c2, c1, c0);

    let steps = vec![
        format!("(A \\pm B)^3 = A^3 \\pm 3A^2B + 3AB^2 \\pm B^3 を利用する"),
        format!("A = {}x, B = {}", a, b),
        format!("A^3 = {}x^3", c3),
        format!("3A^2B = 3 \\times {}x^2 \\times {} = {}x^2", a*a, b, 3*a*a*b),
        format!("3AB^2 = 3 \\times {}x \\times {} = {}x", a, b*b, 3*a*b*b),
        format!("B^3 = {}", b*b*b),
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

/// 和・積から対称式の値（中級）
/// x+y=A, xy=B のとき x^2-xy+y^2 の値
fn medium_symmetric_value(rng: &mut impl Rng) -> Problem {
    let x = pick(rng, &[3i32, 4, 5, 6, 7]);
    let y = pick(rng, &[1i32, 2, 3, 4]);
    let sum = x + y;
    let prod = x * y;
    // x^2 - xy + y^2 = (x+y)^2 - 3xy
    let answer_val = sum * sum - 3 * prod;

    let instruction = format!("x + y = {}, xy = {} のとき, 次の式の値を求めなさい。", sum, prod);
    let question = "x^2 - xy + y^2".to_string();
    let answer = format!("{}", answer_val);

    let steps = vec![
        format!("x^2 - xy + y^2 = (x+y)^2 - 3xy と変形する"),
        format!("= {}^2 - 3 \\times {}", sum, prod),
        format!("= {} - {}", sum*sum, 3*prod),
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

// ============================================================================
// Hard から降格した Medium パターン
// ============================================================================

/// (2x-y+3)^2 のような3変数平方展開
fn medium_three_var_square(rng: &mut impl Rng) -> Problem {
    // (ax + by + c)^2 = a^2x^2 + b^2y^2 + c^2 + 2abxy + 2acx + 2bcy
    let a = pick(rng, &[1i32, 2, 3]);
    let b_sign = if rng.gen_bool(0.5) { 1i32 } else { -1 };
    let b = b_sign * pick(rng, &[1i32, 2, 3]);
    let c = pick(rng, &[-4i32, -3, -2, -1, 1, 2, 3, 4]);

    let c_x2 = a * a;
    let c_y2 = b * b;
    let c_c2 = c * c;
    let c_xy = 2 * a * b;
    let c_x = 2 * a * c;
    let c_y = 2 * b * c;

    let sign_b = if b >= 0 { "+" } else { "-" };
    let sign_c = if c >= 0 { "+" } else { "-" };
    let inner = format!("{}x {} {}y {} {}", a, sign_b, b.abs(), sign_c, c.abs());
    let instruction = "次の計算をしなさい。".to_string();
    let question = format!("({})^2", inner);

    // 答えを構築
    let answer = {
        let mut parts = Vec::new();
        let push = |parts: &mut Vec<String>, coef: i32, var: &str| {
            if coef == 0 { return; }
            let abs = coef.abs();
            let s = if var.is_empty() { format!("{}", abs) }
                    else if abs == 1 { var.to_string() }
                    else { format!("{}{}", abs, var) };
            if parts.is_empty() {
                parts.push(if coef < 0 { format!("-{}", s) } else { s });
            } else {
                parts.push(if coef < 0 { format!("- {}", s) } else { format!("+ {}", s) });
            }
        };
        push(&mut parts, c_x2, "x^2");
        push(&mut parts, c_y2, "y^2");
        push(&mut parts, c_c2, "");
        push(&mut parts, c_xy, "xy");
        push(&mut parts, c_x, "x");
        push(&mut parts, c_y, "y");
        if parts.is_empty() { "0".to_string() } else { parts.join(" ") }
    };

    let steps = vec![
        "(A + B + C)^2 = A^2 + B^2 + C^2 + 2AB + 2BC + 2CA を利用する".to_string(),
        format!("A={}x, B={}y, C={}", a, b, c),
        format!("A^2={}, B^2={}, C^2={}", coeff_str(c_x2, "x^2"), coeff_str(c_y2, "y^2"), c_c2),
        format!("2AB={}, 2AC={}, 2BC={}", coeff_str(c_xy, "xy"), coeff_str(c_x, "x"), coeff_str(c_y, "y")),
        format!("答え: {}", answer),
    ];

    Problem { difficulty: Difficulty::Medium, instruction, question_latex: question, answer_latex: answer, steps }
}

/// (√A+√B)^2 + (√A-√B)^2 の値
fn medium_sqrt_sum_squares(rng: &mut impl Rng) -> Problem {
    let a = pick(rng, &[2u32, 3, 5, 6, 7, 10]);
    let b = pick(rng, &[2u32, 3, 5, 6, 7, 10]);
    // (√A+√B)^2 = A + 2√AB + B
    // (√A-√B)^2 = A - 2√AB + B
    // 合計 = 2A + 2B
    let answer = 2 * (a + b);

    let instruction = "次の計算をしなさい。".to_string();
    let question = format!("(\\sqrt{{{}}} + \\sqrt{{{}}})^2 + (\\sqrt{{{}}} - \\sqrt{{{}}})^2", a, b, a, b);

    let steps = vec![
        format!("(\\sqrt{{{}}} + \\sqrt{{{}}})^2 = {} + 2\\sqrt{{{}}} + {}", a, b, a, a*b, b),
        format!("(\\sqrt{{{}}} - \\sqrt{{{}}})^2 = {} - 2\\sqrt{{{}}} + {}", a, b, a, a*b, b),
        "足すと \\sqrt{AB} の項が消える".to_string(),
        format!("= 2 \\times {} + 2 \\times {} = {}", a, b, answer),
    ];
    Problem { difficulty: Difficulty::Medium, instruction, question_latex: question, answer_latex: format!("{}", answer), steps }
}

/// (3a-2b)^2 - (2a+3b)^2 の展開と整理
fn medium_diff_two_squares_expand(rng: &mut impl Rng) -> Problem {
    let p = pick(rng, &[2i32, 3, 4]);
    let q = pick(rng, &[1i32, 2, 3]);
    let r = pick(rng, &[1i32, 2, 3]);
    let s = pick(rng, &[2i32, 3, 4]);

    // (pa-qb)^2 - (ra+sb)^2
    // = p^2a^2 - 2pqab + q^2b^2 - r^2a^2 - 2rsab - s^2b^2
    // = (p^2-r^2)a^2 - 2(pq+rs)ab + (q^2-s^2)b^2
    let ca = p*p - r*r;
    let cab = -2*(p*q + r*s);
    let cb = q*q - s*s;

    let instruction = "次の計算をしなさい。".to_string();
    let question = format!("({}a - {}b)^2 - ({}a + {}b)^2", p, q, r, s);

    let answer = {
        let mut parts = Vec::new();
        if ca != 0 { parts.push(format!("{}a^2", ca)); }
        if cab != 0 {
            if parts.is_empty() { parts.push(format!("{}ab", cab)); }
            else if cab > 0 { parts.push(format!("+ {}ab", cab)); }
            else { parts.push(format!("- {}ab", cab.abs())); }
        }
        if cb != 0 {
            if parts.is_empty() { parts.push(format!("{}b^2", cb)); }
            else if cb > 0 { parts.push(format!("+ {}b^2", cb)); }
            else { parts.push(format!("- {}b^2", cb.abs())); }
        }
        if parts.is_empty() { "0".to_string() } else { parts.join(" ") }
    };

    let steps = vec![
        format!("({}a - {}b)^2 = {}a^2 - {}ab + {}b^2", p, q, p*p, 2*p*q, q*q),
        format!("({}a + {}b)^2 = {}a^2 + {}ab + {}b^2", r, s, r*r, 2*r*s, s*s),
        format!("a^2の係数: {} - {} = {}", p*p, r*r, ca),
        format!("abの係数: -{} - {} = {}", 2*p*q, 2*r*s, cab),
        format!("b^2の係数: {} - {} = {}", q*q, s*s, cb),
        format!("答え: {}", answer),
    ];
    Problem { difficulty: Difficulty::Medium, instruction, question_latex: question, answer_latex: answer, steps }
}

/// 大きな数の計算工夫: N^2 - (N-1)(N+1) のバリエーション
fn medium_unit_difference(rng: &mut impl Rng) -> Problem {
    let n = pick(rng, &[1000i32, 2000, 3000, 5000, 9999]);
    let k = pick(rng, &[2i32, 3, 4, 5]);
    // n^2 - (n-k)(n+k) = n^2 - (n^2 - k^2) = k^2
    let answer = k * k;

    let instruction = "次の計算をしなさい。".to_string();
    let question = format!("{}^2 - ({} - {})({} + {})", n, n, k, n, k);

    let steps = vec![
        format!("({} - {})({} + {}) = {}^2 - {}^2 の形を見抜く", n, k, n, k, n, k),
        format!("{}^2 - ({}^2 - {}^2)", n, n, k),
        format!("= {}^2 = {}", k, answer),
        format!("答え: {}", answer),
    ];
    Problem { difficulty: Difficulty::Medium, instruction, question_latex: question, answer_latex: format!("{}", answer), steps }
}

/// 連続した数の積の工夫: n(n+1)(n+2)(n+3) + 1 = (n^2+3n+1)^2 の一部
/// ここでは n(n+3) と (n+1)(n+2) を利用する練習
fn medium_consecutive_product(rng: &mut impl Rng) -> Problem {
    let n = pick(rng, &[10i32, 20, 30, 50, 100]);
    // n(n+3) = n^2 + 3n
    // (n+1)(n+2) = n^2 + 3n + 2
    // (n+1)(n+2) - n(n+3) = 2
    let answer = 2i32;

    let instruction = "次の計算をしなさい。".to_string();
    let question = format!("({} + 1)({} + 2) - {} \\times ({} + 3)", n, n, n, n);

    let steps = vec![
        format!("t = n^2 + 3n と置く（n={}）", n),
        format!("(n+1)(n+2) = n^2 + 3n + 2 = t + 2"),
        format!("n(n+3) = n^2 + 3n = t"),
        format!("(t+2) - t = 2"),
        format!("答え: {}", answer),
    ];
    Problem { difficulty: Difficulty::Medium, instruction, question_latex: question, answer_latex: format!("{}", answer), steps }
}

/// 分数を使った式の値: x = p + 1/p のとき x^2 - 2 (具体的な無理数)
fn medium_fraction_expression(rng: &mut impl Rng) -> Problem {
    // (√a - 1/√a)^2 = a - 2 + 1/a
    // 代わりに: (√a + √b)(1/√a + 1/√b) = (√(b/a) + √(a/b) + 2)
    // シンプルに: x = (√5+1)/√5 = 1 + 1/√5 → x^2 = 1 + 2/√5 + 1/5
    // もっとシンプル: x=A/B+B/A のとき x^2-2 を求める
    let a = pick(rng, &[2i32, 3, 4, 5]);
    let b = pick(rng, &[2i32, 3, 4, 5]);
    if a == b { return medium_fraction_expression(rng); }

    // x = a/b + b/a のとき x^2 - 2 = (a/b - b/a)^2 = (a^2-b^2)^2 / (ab)^2
    // でも複雑。代わりにシンプルな問題: x=a, y=1/a のとき (x+y)^2 - (x-y)^2
    // = 4xy = 4 * a * (1/a) = 4 (常に4)
    // これは面白い！
    let n = pick(rng, &[3i32, 5, 7, 11, 13]);

    let instruction = format!("x = {} とする。xy = 1 のとき, 次の式の値を求めなさい。", n);
    let question = "(x + y)^2 - (x - y)^2".to_string();
    let answer = "4".to_string();

    let steps = vec![
        "(x+y)^2 - (x-y)^2 = 4xy を利用する".to_string(),
        "= 4 \\times 1 = 4".to_string(),
        "答え: 4（x の値によらない）".to_string(),
    ];
    Problem { difficulty: Difficulty::Medium, instruction, question_latex: question, answer_latex: answer, steps }
}

/// 置き換えで定数化（バリエーション）: (2x+y)^2 - (2x-y)^2 = 8xy
fn medium_substitution_constant2(rng: &mut impl Rng) -> Problem {
    let x = pick(rng, &[15i32, 20, 25, 30, 50]);
    let y = pick(rng, &[7i32, 11, 13, 17, 19]);
    let answer = 8 * x * y;

    let instruction = format!("x={}, y={} のとき, 次の式の値を求めなさい。", x, y);
    let question = "(2x+y)^2 - (2x-y)^2".to_string();

    let steps = vec![
        "t = 2x と置くと (t+y)^2 - (t-y)^2 の形".to_string(),
        "(t+y)^2 - (t-y)^2 = 4ty を利用する".to_string(),
        format!("= 4 \\times 2x \\times y = 8xy"),
        format!("= 8 \\times {} \\times {} = {}", x, y, answer),
    ];
    Problem { difficulty: Difficulty::Medium, instruction, question_latex: question, answer_latex: format!("{}", answer), steps }
}

/// (a+b)^2 = c^2 の形で a, b が無理数
fn medium_irrational_simplify(rng: &mut impl Rng) -> Problem {
    // (√m + √n)^2 = m + n + 2√(mn) の形の整理
    let m = pick(rng, &[2u32, 3, 5, 6, 7]);
    let n = pick(rng, &[2u32, 3, 5, 6, 7]);
    if m == n { return medium_irrational_simplify(rng); }

    let outer = m + n;
    let inner = m * n;

    let sign = if rng.gen_bool(0.5) { 1i32 } else { -1 };
    let sign_str_str = if sign > 0 { "+" } else { "-" };
    let answer_sign = if sign > 0 { "+" } else { "-" };

    let instruction = "次の式を計算しなさい。".to_string();
    let question = format!("(\\sqrt{{{}}} {} \\sqrt{{{}}})^2", m, sign_str_str, n);

    let sqrt_part = if inner == 1 { "2".to_string() } else { format!("2\\sqrt{{{}}}", inner) };
    let answer = format!("{} {} {}", outer, answer_sign, sqrt_part);

    let steps = vec![
        "(A ± B)^2 = A^2 ± 2AB + B^2 を利用する".to_string(),
        format!("(\\sqrt{{{}}})^2 = {}", m, m),
        format!("2 \\times \\sqrt{{{}}} \\times \\sqrt{{{}}} = 2\\sqrt{{{}}}", m, n, inner),
        format!("(\\sqrt{{{}}})^2 = {}", n, n),
        format!("答え: {}", answer),
    ];
    Problem { difficulty: Difficulty::Medium, instruction, question_latex: question, answer_latex: answer, steps }
}
