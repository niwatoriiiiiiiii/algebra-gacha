//! 3年生の問題生成モジュール。
//!
//! 出題範囲: 乗法公式・計算の工夫・式の値
//! 難易度: 初級 / 中級 / 上級 / 超級（全25パターン）

use rand::Rng;

use super::{Difficulty, Problem, coeff_str, pick, random_sign};

// ---- 公開 API ----------------------------------------------------------------

/// 指定難易度の3年生問題を生成して返す。
pub fn generate(rng: &mut impl Rng, difficulty: &Difficulty) -> Problem {
    match difficulty {
        Difficulty::Easy => generate_easy(rng),
        Difficulty::Medium => generate_medium(rng),
        Difficulty::Hard => generate_hard(rng),
        Difficulty::Ultra => generate_ultra(rng),
    }
}

// ============================================================================
// 初級（Easy）
// ============================================================================

fn generate_easy(rng: &mut impl Rng) -> Problem {
    // 初級パターンを均等に選ぶ
    match rng.gen_range(0..5u8) {
        0 => easy_distributive(rng),
        1 => easy_xaxb(rng),
        2 => easy_square(rng),
        3 => easy_difference_of_squares(rng),
        _ => easy_substitution(rng),
    }
}

/// 分配法則（多項式×多項式）: `(Ax + B)(x^2 + Cx + D)`
fn easy_distributive(rng: &mut impl Rng) -> Problem {
    let a = pick(rng, &[1i32, 2, 3]);
    let b = pick(rng, &[-3i32, -2, -1, 1, 2, 3]);
    let c = pick(rng, &[-3i32, -2, -1, 1, 2, 3]);
    let d = pick(rng, &[-3i32, -2, -1, 1, 2, 3]);

    // (ax + b)(x^2 + cx + d)
    // = ax^3 + acx^2 + adx + bx^2 + bcx + bd
    // = ax^3 + (ac+b)x^2 + (ad+bc)x + bd
    let c3 = a;
    let c2 = a * c + b;
    let c1 = a * d + b * c;
    let c0 = b * d;

    let instruction = "次の計算をしなさい。".to_string();
    let question = format!(
        "次の計算をしなさい。\\\\ ({})(x^2 {} {}x {} {})",
        format_linear(a, b, "x"),
        sign_str(c),
        c.abs(),
        sign_str(d),
        d.abs()
    );

    let answer = format_poly4(c3, c2, c1, c0);

    let steps = vec![
        format!(
            "{}を x^2 + {}x + {} の各項にかける",
            format_linear(a, b, "x"),
            c,
            d
        ),
        format!("x^3 の係数: {}", c3),
        format!("x^2 の係数: {} \\times {} + {} = {}", a, c, b, c2),
        format!(
            "x の係数: {} \\times {} + {} \\times {} = {}",
            a, d, b, c, c1
        ),
        format!("定数項: {} \\times {} = {}", b, d, c0),
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

/// `(x+a)(x+b)` 型の展開: `(x + Ay)(x + By)`
fn easy_xaxb(rng: &mut impl Rng) -> Problem {
    let coeffs: &[i32] = &[-4, -3, -2, -1, 1, 2, 3, 4];
    let a = pick(rng, coeffs);
    let b = pick(rng, coeffs);

    // (x + ay)(x + by) = x^2 + (a+b)xy + ab y^2
    let sum = a + b;
    let prod = a * b;

    let instruction = "次の計算をしなさい。".to_string();
    let question = format!(
        "(x {} {})(x {} {})",
        sign_str(a),
        coeff_str(a.abs(), "y"),
        sign_str(b),
        coeff_str(b.abs(), "y")
    );

    let xy_term = match sum.cmp(&0) {
        std::cmp::Ordering::Equal => String::new(),
        std::cmp::Ordering::Greater => format!(" + {}xy", sum),
        std::cmp::Ordering::Less => format!(" - {}xy", sum.abs()),
    };
    let y2_term = match prod.cmp(&0) {
        std::cmp::Ordering::Equal => String::new(),
        std::cmp::Ordering::Greater => format!(" + {}y^2", prod),
        std::cmp::Ordering::Less => format!(" - {}y^2", prod.abs()),
    };
    let answer = format!("x^2{}{}", xy_term, y2_term);

    let steps = vec![
        format!("(x+a)(x+b) = x^2 + (a+b)x + ab を利用する"),
        format!("a={}, b={} とおく", a, b),
        format!("a+b = {} + ({}) = {}", a, b, sum),
        format!("ab = {} \\times {} = {}", a, b, prod),
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

/// `(x±a)^2` 型の展開（平方根含む）
fn easy_square(rng: &mut impl Rng) -> Problem {
    // 平方根パターン or 通常パターンを選ぶ
    if rng.gen_bool(0.4) {
        // sqrt パターン: (\sqrt{A} - \sqrt{B})^2
        let a = pick(rng, &[2u32, 3, 5, 6, 7]);
        let b = pick(rng, &[2u32, 3, 5, 6, 7]);
        let sign = random_sign(rng);

        // (\sqrt{a} ± \sqrt{b})^2 = a ± 2\sqrt{ab} + b = (a+b) ± 2\sqrt{ab}
        let outer = a + b;
        let inner = a * b;

        let sign_str_pm = if sign > 0 { "+" } else { "-" };
        let instruction = "次の計算をしなさい。".to_string();
        let question = format!("(\\sqrt{{{}}} {} \\sqrt{{{}}})^2", a, sign_str_pm, b);
        let sqrt_part = if inner == 1 {
            "2".to_string()
        } else {
            format!("2\\sqrt{{{}}}", inner)
        };
        let answer = format!(
            "{} {} {}",
            outer,
            if sign > 0 { "+" } else { "-" },
            sqrt_part
        );

        let steps = vec![
            format!("(x \\pm y)^2 = x^2 \\pm 2xy + y^2 を利用する"),
            format!("(\\sqrt{{{}}})^2 = {}", a, a),
            format!(
                "2 \\times \\sqrt{{{}}} \\times \\sqrt{{{}}} = 2\\sqrt{{{}}}",
                a, b, inner
            ),
            format!("(\\sqrt{{{}}})^2 = {}", b, b),
            format!("答え: {}", answer),
        ];

        return Problem {
            difficulty: Difficulty::Easy,
            instruction,
            question_latex: question,
            answer_latex: answer,
            steps,
        };
    }

    // 通常パターン: (Ax ± B)^2
    let a = pick(rng, &[1i32, 2, 3]);
    let b = pick(rng, &[1i32, 2, 3, 4, 5]);
    let sign = random_sign(rng);

    // (ax ± b)^2 = a^2 x^2 ± 2ab x + b^2
    let c2 = a * a;
    let c1 = 2 * a * b * sign;
    let c0 = b * b;

    let q_inner = if sign > 0 {
        format!("{} + {}", coeff_str(a, "x"), b)
    } else {
        format!("{} - {}", coeff_str(a, "x"), b)
    };
    let instruction = "次の計算をしなさい。".to_string();
    let question = format!("({})^2", q_inner);

    let answer = format_quadratic(c2, "x", c1, "x", c0);

    let steps = vec![
        format!("(x \\pm a)^2 = x^2 \\pm 2ax + a^2 を利用する"),
        format!("({})^2 = {}", coeff_str(a, "x"), coeff_str(c2, "x^2")),
        format!(
            "2 \\times {} \\times {} = {}",
            coeff_str(a, "x"),
            b,
            coeff_str(c1, "x")
        ),
        format!("{}^2 = {}", b, c0),
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

/// `(x+a)(x-a)` 型の展開: `(Ax - By)(Ax + By)`
fn easy_difference_of_squares(rng: &mut impl Rng) -> Problem {
    let a = pick(rng, &[1i32, 2, 3]);
    let b = pick(rng, &[1i32, 2, 3]);

    // (ax - by)(ax + by) = a^2 x^2 - b^2 y^2
    let c2x = a * a;
    let c2y = b * b;

    let instruction = "次の計算をしなさい。".to_string();
    let question = format!(
        "({} - {})({} + {})",
        coeff_str(a, "x"),
        coeff_str(b, "y"),
        coeff_str(a, "x"),
        coeff_str(b, "y")
    );
    let answer = format!("{} - {}", coeff_str(c2x, "x^2"), coeff_str(c2y, "y^2"));

    let steps = vec![
        format!("(x+a)(x-a) = x^2 - a^2 を利用する"),
        format!("({})^2 = {}", coeff_str(a, "x"), coeff_str(c2x, "x^2")),
        format!("({})^2 = {}", coeff_str(b, "y"), coeff_str(c2y, "y^2")),
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

/// 置き換えによる展開（初級）: `(x - Ay + B)(x - Ay + C)`
fn easy_substitution(rng: &mut impl Rng) -> Problem {
    let a = pick(rng, &[1i32, 2, 3]);
    let b = pick(rng, &[-4i32, -3, -2, -1, 1, 2, 3, 4]);
    let c = pick(rng, &[-4i32, -3, -2, -1, 1, 2, 3, 4]);

    // t = x - ay と置くと (t + b)(t + c) = t^2 + (b+c)t + bc
    // = (x-ay)^2 + (b+c)(x-ay) + bc
    let sum = b + c;
    let prod = b * c;

    let q_str1 = format!("x {} + {}", coeff_str(-a, "y"), b);
    let q_str2 = format!("x {} + {}", coeff_str(-a, "y"), c);
    let instruction = "次の計算をしなさい。".to_string();
    let question = format!("({})({}) ", q_str1, q_str2);

    // 展開: (x-ay)^2 + (b+c)(x-ay) + bc
    // = x^2 - 2axy + a^2 y^2 + (b+c)x - a(b+c)y + bc
    let c_x2 = 1i32;
    let c_xy = -2 * a;
    let c_y2 = a * a;
    let c_x = sum;
    let c_y = -a * sum;
    let c_0 = prod;

    let answer = format_full_expansion(c_x2, c_xy, c_y2, c_x, c_y, c_0);

    let steps = vec![
        format!("t = x {} と置く", coeff_str(-a, "y")),
        format!(
            "(t {} {})(t {} {}) = t^2 + {}t + {}",
            sign_str(b),
            b.abs(),
            sign_str(c),
            c.abs(),
            sum,
            prod
        ),
        format!(
            "t を戻す: (x {})^2 + {}(x {}) + {}",
            coeff_str(-a, "y"),
            sum,
            coeff_str(-a, "y"),
            prod
        ),
        format!("展開して答え: {}", answer),
    ];

    Problem {
        difficulty: Difficulty::Easy,
        instruction,
        question_latex: question,
        answer_latex: answer,
        steps,
    }
}

// ============================================================================
// 中級（Medium）
// ============================================================================

fn generate_medium(rng: &mut impl Rng) -> Problem {
    match rng.gen_range(0..6u8) {
        0 => medium_xaxb_coeff(rng),
        1 => medium_square_frac(rng),
        2 => medium_diff_squares_frac(rng),
        3 => medium_substitution(),
        4 => medium_numerical_trick(rng),
        _ => medium_expression_value(rng),
    }
}

/// `(Aa - b)(Aa + Bb)` 型
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
// ============================================================================

fn generate_hard(rng: &mut impl Rng) -> Problem {
    match rng.gen_range(0..3u8) {
        0 => hard_substitution_complex(rng),
        1 => hard_numerical_trick(rng),
        _ => hard_expression_value(rng),
    }
}

/// 置き換えによる展開（上級）: `(x+Ay-1)^2 - (x+Ay)(x+Ay-2)` = 1
fn hard_substitution_complex(rng: &mut impl Rng) -> Problem {
    let a = pick(rng, &[1i32, 2, 3]);

    // t = x + Ay と置く
    // (t-1)^2 - t(t-2)
    // = t^2 - 2t + 1 - t^2 + 2t = 1
    let instruction = "次の計算をしなさい。".to_string();
    let question = format!(
        "(x + {} - 1)^2 - (x + {})(x + {} - 2)",
        coeff_str(a, "y"),
        coeff_str(a, "y"),
        coeff_str(a, "y")
    );

    let steps = vec![
        format!("t = x + {} と置く", coeff_str(a, "y")),
        format!("(t-1)^2 - t(t-2)"),
        format!("t^2 - 2t + 1 - t^2 + 2t"),
        format!("答え: 1（定数x, y の値によらない）"),
    ];

    Problem {
        difficulty: Difficulty::Hard,
        instruction,
        question_latex: question,
        answer_latex: "1".to_string(),
        steps,
    }
}

/// 計算の工夫（上級）
fn hard_numerical_trick(rng: &mut impl Rng) -> Problem {
    match rng.gen_range(0..2u8) {
        0 => {
            // A^2 - B^2 型（差が大きい）
            let base = pick(rng, &[2031i32, 1025, 3000, 1999]);
            let delta = pick(rng, &[8i32, 4, 10, 6]);
            // base^2 - (base+delta)^2 = -(2*base + delta)*delta
            let a = base;
            let b = base + delta;
            let answer = (a + b) * (a - b); // 負になる

            let instruction = "次の計算をしなさい。".to_string();
            let question = format!("{}^2 - {}^2", a, b);
            let steps = vec![
                format!("a^2 - b^2 = (a+b)(a-b) を利用する"),
                format!("a + b = {} + {} = {}", a, b, a + b),
                format!("a - b = {} - {} = {}", a, b, a - b),
                format!("答え: {} \\times {} = {}", a + b, a - b, answer),
            ];

            Problem {
                difficulty: Difficulty::Hard,
                instruction,
                question_latex: question,
                answer_latex: format!("{}", answer),
                steps,
            }
        }
        _ => {
            // 分数型: (A^2 - B^2) / (C^2 - D^2)
            // = (A+B)(A-B) / (C+D)(C-D)
            let a = pick(rng, &[201i32, 301, 401]);
            let b = a - 2; // A-2
            let c = a * 2 - 1; // 2A-1
            let d = c - 2;

            // (a^2 - b^2) / (c^2 - d^2)
            // = (a+b)(a-b) / (c+d)(c-d)
            // = (2a-2)*2 / (2c-2)*2 = (2(a-1)) / (2(c-1))
            // aを具体的に計算
            let num = (a + b) * (a - b);
            let den = (c + d) * (c - d);
            // 約分
            let g = gcd_i(num, den);
            let num_r = num / g;
            let den_r = den / g;

            let instruction = "次の計算をしなさい。".to_string();
            let question = format!("\\frac{{{}^2 - {}^2}}{{{}^2 - {}^2}}", a, b, c, d);
            let answer = if den_r == 1 {
                format!("{}", num_r)
            } else {
                format!("\\frac{{{}}}{{{}}}", num_r, den_r)
            };

            let steps = vec![
                format!(
                    "分子: ({}+{})({}−{}) = {} \\times {} = {}",
                    a,
                    b,
                    a,
                    b,
                    a + b,
                    a - b,
                    num
                ),
                format!(
                    "分母: ({}+{})({}−{}) = {} \\times {} = {}",
                    c,
                    d,
                    c,
                    d,
                    c + d,
                    c - d,
                    den
                ),
                format!("約分: \\frac{{{}}}{{{}}}", num, den),
                format!("答え: {}", answer),
            ];

            Problem {
                difficulty: Difficulty::Hard,
                instruction,
                question_latex: question,
                answer_latex: answer,
                steps,
            }
        }
    }
}

/// 式の値（上級）
fn hard_expression_value(rng: &mut impl Rng) -> Problem {
    let x_str = pick(rng, &["0.4", "0.3", "0.6", "0.2"]);
    let y_str = pick(rng, &["0.3", "0.2", "0.1", "0.4"]);

    // 有理数に変換
    let (x_num, x_den) = parse_decimal(x_str);
    let (y_num, y_den) = parse_decimal(y_str);

    // x + 2y = x_num/x_den + 2*y_num/y_den = (x_num*y_den + 2*y_num*x_den) / (x_den*y_den)
    let combined_num = x_num * y_den + 2 * y_num * x_den;
    let combined_den = x_den * y_den;
    let answer_val_num = combined_num * combined_num;
    let answer_val_den = combined_den * combined_den;
    let g = gcd_i(answer_val_num, answer_val_den);
    let ans_n = answer_val_num / g;
    let ans_d = answer_val_den / g;

    let answer = if ans_d == 1 {
        format!("{}", ans_n)
    } else {
        format!("\\frac{{{}}}{{{}}}", ans_n, ans_d)
    };

    let instruction = format!("x={}, y={} のとき, 次の式の値を求めなさい。", x_str, y_str);
    let question = "5(2x^2 + 2xy + y^2) - (3x + y)^2".to_string();

    let steps = vec![
        "展開する: 10x^2 + 10xy + 5y^2 - 9x^2 - 6xy - y^2".to_string(),
        "整理: x^2 + 4xy + 4y^2".to_string(),
        "因数分解: (x + 2y)^2".to_string(),
        format!(
            "x + 2y = {} + 2 \\times {} = \\frac{{{}}}{{{}}}",
            x_str, y_str, combined_num, combined_den
        ),
        format!(
            "答え: \\left(\\frac{{{}}}{{{}}}\\right)^2 = {}",
            combined_num, combined_den, answer
        ),
    ];

    Problem {
        difficulty: Difficulty::Hard,
        instruction,
        question_latex: question,
        answer_latex: answer,
        steps,
    }
}

// ============================================================================
// 超級（Ultra）- 全25パターン
// ============================================================================

fn generate_ultra(rng: &mut impl Rng) -> Problem {
    // 25パターンを均等にランダム選択
    match rng.gen_range(0..25u8) {
        0 => ultra_01(rng),
        1 => ultra_02(rng),
        2 => ultra_03(rng),
        3 => ultra_04(rng),
        4 => ultra_05(rng),
        5 => ultra_06(rng),
        6 => ultra_07(rng),
        7 => ultra_08(rng),
        8 => ultra_09(rng),
        9 => ultra_10(rng),
        10 => ultra_11(rng),
        11 => ultra_12(rng),
        12 => ultra_13(rng),
        13 => ultra_14(rng),
        14 => ultra_15(rng),
        15 => ultra_16(rng),
        16 => ultra_17(),
        17 => ultra_18(),
        18 => ultra_19(),
        19 => ultra_20(rng),
        20 => ultra_21(rng),
        21 => ultra_22(rng),
        22 => ultra_23(rng),
        23 => ultra_24(rng),
        _ => ultra_25(rng),
    }
}

/// 超級-01: `(a-Cb)^2 - D(a-Cb) - E の値`
fn ultra_01(rng: &mut impl Rng) -> Problem {
    let a_val = pick(rng, &[30i32, 40, 50, 25]);
    let b_val = pick(rng, &[-23i32, -15, -18, -12]);
    let cap_c = pick(rng, &[2i32, 3]);
    let cap_d = pick(rng, &[2i32, 3, 4, 5, 6]);
    let cap_e = pick(rng, &[8i32, 12, 15, 24]);

    let t = a_val - cap_c * b_val;
    let answer = t * t - cap_d * t - cap_e;

    let instruction = format!("a={}, b={} のとき, 次の式の値を求めなさい。", a_val, b_val);
    let question = format!("(a - {}b)^2 - {}(a - {}b) - {}", cap_c, cap_d, cap_c, cap_e);

    let steps = vec![
        format!("t = a - {}b と置く", cap_c),
        format!("t = {} - {} \\times ({}) = {}", a_val, cap_c, b_val, t),
        format!(
            "t^2 - {}t - {} = {}^2 - {} \\times {} - {}",
            cap_d, cap_e, t, cap_d, t, cap_e
        ),
        format!("= {} - {} - {}", t * t, cap_d * t, cap_e),
        format!("答え: {}", answer),
    ];

    ultra_problem(instruction, question, format!("{}", answer), steps)
}

/// 超級-02: `(2x+y)^2 - 3(2x+y) + 2 の値`
fn ultra_02(rng: &mut impl Rng) -> Problem {
    let x_val = pick(rng, &[3i32, 4, 5, 7]);
    let y_val = pick(rng, &[-4i32, -5, -6, -7, -8]);

    let t = 2 * x_val + y_val;
    let answer = t * t - 3 * t + 2;

    let instruction = format!("x={}, y={} のとき, 次の式の値を求めなさい。", x_val, y_val);
    let question = "(2x+y)^2 - 3(2x+y) + 2".to_string();

    let steps = vec![
        "t = 2x + y と置く".to_string(),
        format!("t = 2 \\times {} + ({}) = {}", x_val, y_val, t),
        format!("t^2 - 3t + 2 = (t-1)(t-2)（因数分解）"),
        format!(
            "t = {} を代入: ({} - 1)({} - 2) = {} \\times {}",
            t,
            t,
            t,
            t - 1,
            t - 2
        ),
        format!("答え: {}", answer),
    ];

    ultra_problem(instruction, question, format!("{}", answer), steps)
}

/// 超級-03: `(a+2b-1)^2 - (a+2b)(a+2b-2)` = 1（定数化）
fn ultra_03(rng: &mut impl Rng) -> Problem {
    let a_val = pick(rng, &[5i32, 7, 11, 13]);
    let b_val = pick(rng, &[3i32, 4, 5, 6]);

    let instruction = format!("a={}, b={} のとき, 次の式の値を求めなさい。", a_val, b_val);
    let question = "(a + 2b - 1)^2 - (a + 2b)(a + 2b - 2)".to_string();

    let steps = vec![
        "t = a + 2b と置く".to_string(),
        "(t-1)^2 - t(t-2)".to_string(),
        "t^2 - 2t + 1 - t^2 + 2t".to_string(),
        "答え: 1（a, b の値によらず常に 1！）".to_string(),
    ];

    ultra_problem(instruction, question, "1".to_string(), steps)
}

/// 超級-04: `(x-y+3)^2 - (x-y)^2 - 6(x-y)` = 9（定数化）
fn ultra_04(rng: &mut impl Rng) -> Problem {
    let x_val = pick(rng, &[10i32, 20, 30, 50]);
    let y_val = pick(rng, &[7i32, 13, 23, 43]);

    let instruction = format!("x={}, y={} のとき, 次の式の値を求めなさい。", x_val, y_val);
    let question = "(x - y + 3)^2 - (x-y)^2 - 6(x-y)".to_string();

    let steps = vec![
        "t = x - y と置く".to_string(),
        "(t+3)^2 - t^2 - 6t".to_string(),
        "t^2 + 6t + 9 - t^2 - 6t".to_string(),
        "答え: 9（x, y の値によらず常に 9！）".to_string(),
    ];

    ultra_problem(instruction, question, "9".to_string(), steps)
}

/// 超級-05: `x+y=A, xy=B のとき x^2+y^2 の値`
fn ultra_05(rng: &mut impl Rng) -> Problem {
    // x, y を整数で先に決めてから A, B を計算する
    let x = pick(rng, &[2i32, 3, 4, 5, 6]);
    let y = pick(rng, &[1i32, 2, 3, 4, 5]);
    let sum = x + y;
    let prod = x * y;
    let answer = x * x + y * y;

    let instruction = format!(
        "x + y = {}, xy = {} のとき, 次の式の値を求めなさい。",
        sum, prod
    );
    let question = "x^2 + y^2".to_string();

    let steps = vec![
        "(x+y)^2 = x^2 + 2xy + y^2 を変形する".to_string(),
        format!("x^2 + y^2 = (x+y)^2 - 2xy = {}^2 - 2 \\times {}", sum, prod),
        format!("= {} - {}", sum * sum, 2 * prod),
        format!("答え: {}", answer),
    ];

    ultra_problem(instruction, question, format!("{}", answer), steps)
}

/// 超級-06: `x-y=A, xy=B のとき x^2+y^2 の値`
fn ultra_06(rng: &mut impl Rng) -> Problem {
    let x = pick(rng, &[5i32, 6, 7, 8]);
    let y = pick(rng, &[1i32, 2, 3, 4]);
    let diff = x - y;
    let prod = x * y;
    let answer = x * x + y * y;

    let instruction = format!(
        "x - y = {}, xy = {} のとき, 次の式の値を求めなさい。",
        diff, prod
    );
    let question = "x^2 + y^2".to_string();

    let steps = vec![
        "(x-y)^2 = x^2 - 2xy + y^2 を変形する".to_string(),
        format!(
            "x^2 + y^2 = (x-y)^2 + 2xy = {}^2 + 2 \\times {}",
            diff, prod
        ),
        format!("= {} + {}", diff * diff, 2 * prod),
        format!("答え: {}", answer),
    ];

    ultra_problem(instruction, question, format!("{}", answer), steps)
}

/// 超級-07: `x+y=A, x^2+y^2=B のとき (x-y)^2 の値`
fn ultra_07(rng: &mut impl Rng) -> Problem {
    let x = pick(rng, &[4i32, 5, 6, 7]);
    let y = pick(rng, &[1i32, 2, 3]);
    let sum = x + y;
    let sum_sq = x * x + y * y;
    let diff_sq = (x - y) * (x - y);

    let instruction = format!(
        "x + y = {}, x^2 + y^2 = {} のとき, 次の式の値を求めなさい。",
        sum, sum_sq
    );
    let question = "(x-y)^2".to_string();

    let steps = vec![
        "(x-y)^2 = 2(x^2+y^2) - (x+y)^2 を利用する".to_string(),
        format!("= 2 \\times {} - {}^2", sum_sq, sum),
        format!("= {} - {}", 2 * sum_sq, sum * sum),
        format!("答え: {}", diff_sq),
    ];

    ultra_problem(instruction, question, format!("{}", diff_sq), steps)
}

/// 超級-08: `a=√P+√Q, b=√P-√Q のとき a^2+b^2 の値`
fn ultra_08(rng: &mut impl Rng) -> Problem {
    let p = pick(rng, &[2u32, 3, 5, 6]);
    let q = pick(rng, &[2u32, 3, 5, 7]);
    let answer = 2 * (p + q);

    let instruction = format!(
        "a = \\sqrt{{{}}} + \\sqrt{{{}}}, b = \\sqrt{{{}}} - \\sqrt{{{}}} のとき, 次の式の値を求めなさい。",
        p, q, p, q
    );
    let question = "a^2 + b^2".to_string();

    let steps = vec![
        "a^2 = (\\sqrt{P}+\\sqrt{Q})^2 = P + 2\\sqrt{PQ} + Q と展開".to_string(),
        "b^2 = (\\sqrt{P}-\\sqrt{Q})^2 = P - 2\\sqrt{PQ} + Q と展開".to_string(),
        "a^2 + b^2 = 2P + 2Q（\\sqrt{PQ} が消える）".to_string(),
        format!("= 2({} + {}) = {}", p, q, answer),
        format!("答え: {}", answer),
    ];

    ultra_problem(instruction, question, format!("{}", answer), steps)
}

/// 超級-09: `m=√A+√B, n=√A-√B のとき (m^2-n^2)^2 の値`
fn ultra_09(rng: &mut impl Rng) -> Problem {
    let a = pick(rng, &[3u32, 5, 7, 2]);
    let b = pick(rng, &[2u32, 3, 5, 6]);
    // m^2 - n^2 = (m+n)(m-n) = 2√A × 2√B = 4√(AB)
    // (m^2-n^2)^2 = 16AB
    let answer = 16 * a * b;

    let instruction = format!(
        "m = \\sqrt{{{}}} + \\sqrt{{{}}}, n = \\sqrt{{{}}} - \\sqrt{{{}}} のとき, 次の式の値を求めなさい。",
        a, b, a, b
    );
    let question = "(m^2 - n^2)^2".to_string();

    let steps = vec![
        "m^2 - n^2 = (m+n)(m-n) と因数分解する".to_string(),
        format!("m + n = 2\\sqrt{{{}}}, m - n = 2\\sqrt{{{}}}", a, b),
        format!(
            "(m+n)(m-n) = 2\\sqrt{{{}}} \\times 2\\sqrt{{{}}} = 4\\sqrt{{{}}}",
            a,
            b,
            a * b
        ),
        format!(
            "(4\\sqrt{{{}}})^2 = 16 \\times {} = {}",
            a * b,
            a * b,
            answer
        ),
        format!("答え: {}", answer),
    ];

    ultra_problem(instruction, question, format!("{}", answer), steps)
}

/// 超級-10: `x=√A + B のとき x^2 - 2Bx + C の値`
fn ultra_10(rng: &mut impl Rng) -> Problem {
    let a = pick(rng, &[2u32, 3, 5, 7]);
    let b = pick(rng, &[1i32, 2, 3, 4]);
    let c = pick(rng, &[1i32, 2, 3, 4, 5]);

    // x = √A + B → x - B = √A → (x-B)^2 = A
    // x^2 - 2Bx + B^2 = A → x^2 - 2Bx = A - B^2
    let answer = a as i32 - b * b + c;

    let instruction = format!(
        "x = \\sqrt{{{}}} + {} のとき, 次の式の値を求めなさい。",
        a, b
    );
    let question = format!("x^2 - {}x + {}", 2 * b, c);

    let steps = vec![
        format!("x - {} = \\sqrt{{{}}} と変形する", b, a),
        format!("両辺を2乗: (x-{})^2 = {}", b, a),
        format!("x^2 - {}x + {} = {}", 2 * b, b * b, a),
        format!(
            "x^2 - {}x = {} - {} = {}",
            2 * b,
            a,
            b * b,
            a as i32 - b * b
        ),
        format!(
            "x^2 - {}x + {} = {} + {} = {}",
            2 * b,
            c,
            a as i32 - b * b,
            c,
            answer
        ),
    ];

    ultra_problem(instruction, question, format!("{}", answer), steps)
}

/// 超級-11: `x = 1/(√A+B) のとき x^2 + 2Bx の値`
fn ultra_11(rng: &mut impl Rng) -> Problem {
    let b = pick(rng, &[1i32, 2]);

    // a=2→a_simple=2, a=3→a_simple=2(b=1)or5(b=2), a=5→a_simple=2or5
    // b=1のとき a_simple = 2, b=2のとき a_simple = 5
    // b=1,2 いずれも a^2 - b^2 ≠ 0 なので常に有効
    let a_simple = (b * b + 1) as u32;
    // x = √(B^2+1) - B
    // x + B = √(B^2+1)
    // (x+B)^2 = B^2+1
    // x^2 + 2Bx + B^2 = B^2 + 1
    // x^2 + 2Bx = 1
    let instruction = format!(
        "x = \\dfrac{{1}}{{\\sqrt{{{}}} + {}}} のとき, 次の式の値を求めなさい。",
        a_simple, b
    );
    let question = format!("x^2 + {}x", 2 * b);

    let steps = vec![
        format!(
            "有理化: x = \\dfrac{{\\sqrt{{{}}} - {}}}{{({})^2 - ({})^2}} = \\sqrt{{{}}} - {}",
            a_simple, b, a_simple, b, a_simple, b
        ),
        format!("x + {} = \\sqrt{{{}}}", b, a_simple),
        format!("両辺を2乗: (x+{})^2 = {}", b, a_simple),
        format!("x^2 + {}x + {} = {}", 2 * b, b * b, a_simple),
        format!("x^2 + {}x = {} - {} = 1", 2 * b, a_simple, b * b),
    ];

    ultra_problem(instruction, question, "1".to_string(), steps)
}

/// 超級-12: `x = (√A-√B)/(√A+√B) のとき x^2 + 2x の値`
fn ultra_12(rng: &mut impl Rng) -> Problem {
    let a = pick(rng, &[3u32, 5, 7]);
    let b = pick(rng, &[2u32, 3]);
    if a == b {
        return ultra_12(rng);
    }

    // x = (√A - √B)/(√A + √B)
    // 有理化: x = (√A-√B)^2/(A-B) = (A + B - 2√AB)/(A-B)
    // x + 1 = (A+B-2√AB + A-B)/(A-B) = (2A-2√AB)/(A-B) = 2√A(√A-√B)/(A-B) = 2√A/(√A+√B)
    // 複雑なので具体値で答えを示す

    // x(x+2) = x^2 + 2x を求める
    // x^2 + 2x + 1 = (x+1)^2
    // x + 1 = (√A-√B)/(√A+√B) + 1 = (√A-√B + √A+√B)/(√A+√B) = 2√A/(√A+√B)
    // (x+1)^2 = 4A/(√A+√B)^2 = 4A/(A+2√AB+B)
    // 計算が複雑なので、整数になるよう A=B+1 のケースは避け、
    // 具体的に (√3-√2)/(√3+√2) = (√3-√2)^2/(3-2) = 5-2√6 を利用

    // x = (5 - 2√6), x+1 = 6 - 2√6 = 2(3-√6)... まだ複雑
    // シンプルに: x^2 + 2x = (x+1)^2 - 1
    // (x+1) = 2√A/(√A+√B) として (x+1)^2 = 4A/(A+B+2√AB)

    // よりシンプルな問い: x + 1/x を使った問題に変更
    // x = (√A-√B)/(√A+√B), 1/x = (√A+√B)/(√A-√B)
    // x + 1/x = (√A-√B)^2 + (√A+√B)^2 / ((√A)^2-(√B)^2) = 2(A+B)/(A-B)

    let numerator = 2 * (a + b) as i32;
    let denominator = (a as i32 - b as i32).abs();
    let g = gcd_i(numerator, denominator);
    let ans_n = numerator / g;
    let ans_d = denominator / g;
    let answer = if ans_d == 1 {
        format!("{}", ans_n)
    } else {
        format!("\\frac{{{}}}{{{}}}", ans_n, ans_d)
    };

    let instruction = format!(
        "x = \\dfrac{{\\sqrt{{{}}} - \\sqrt{{{}}}}}{{\\sqrt{{{}}} + \\sqrt{{{}}}}} のとき, 次の式の値を求めなさい。",
        a, b, a, b
    );
    let question = "x + \\dfrac{1}{x}".to_string();

    let steps = vec![
        format!(
            "\\frac{{1}}{{x}} = \\frac{{\\sqrt{{{}}} + \\sqrt{{{}}}}}{{\\sqrt{{{}}} - \\sqrt{{{}}}}}",
            a, b, a, b
        ),
        "x + 1/x の分子: (√A-√B)^2 + (√A+√B)^2 = 2(A+B)".to_string(),
        "分母: (√A+√B)(√A-√B) = A - B".to_string(),
        format!(
            "x + 1/x = \\frac{{2({} + {})}}{{{} - {}}} = \\frac{{{}}}{{{}}}",
            a,
            b,
            a,
            b,
            2 * (a + b),
            (a as i32 - b as i32).abs()
        ),
        format!("答え: {}", answer),
    ];

    ultra_problem(instruction, question, answer, steps)
}

/// 超級-13: `a=√P+√Q, b=√P-√Q のとき a^2b+ab^2 の値`
fn ultra_13(rng: &mut impl Rng) -> Problem {
    let p = pick(rng, &[3u32, 5, 7]);
    let q = pick(rng, &[2u32, 3, 5]);
    if p == q {
        return ultra_13(rng);
    }

    // a^2 b + ab^2 = ab(a+b)
    // ab = (√P+√Q)(√P-√Q) = P - Q
    // a + b = 2√P
    // → ab(a+b) = (P-Q) × 2√P = 2(P-Q)√P
    let coeff = 2 * (p as i32 - q as i32);
    let answer = if coeff == 2 {
        format!("2\\sqrt{{{}}}", p)
    } else {
        format!("{}\\sqrt{{{}}}", coeff, p)
    };

    let instruction = format!(
        "a = \\sqrt{{{}}} + \\sqrt{{{}}}, b = \\sqrt{{{}}} - \\sqrt{{{}}} のとき, 次の式の値を求めなさい。",
        p, q, p, q
    );
    let question = "a^2 b + ab^2".to_string();

    let steps = vec![
        "a^2 b + ab^2 = ab(a+b) と因数分解する".to_string(),
        format!(
            "ab = (\\sqrt{{{}}} + \\sqrt{{{}}})(\\sqrt{{{}}} - \\sqrt{{{}}}) = {} - {} = {}",
            p,
            q,
            p,
            q,
            p,
            q,
            p as i32 - q as i32
        ),
        format!("a + b = 2\\sqrt{{{}}}", p),
        format!(
            "ab(a+b) = {} \\times 2\\sqrt{{{}}} = {}",
            p as i32 - q as i32,
            p,
            answer
        ),
        format!("答え: {}", answer),
    ];

    ultra_problem(instruction, question, answer, steps)
}

/// 超級-14: `((N+1)^2-(N-1)^2) / ((N+2)^2-(N-2)^2)` = 1/2（定数）
fn ultra_14(rng: &mut impl Rng) -> Problem {
    let n = pick(rng, &[100i32, 200, 500, 1000, 777]);

    let instruction = "次の式の値を求めなさい。".to_string();
    let question = format!(
        "\\dfrac{{({} + 1)^2 - ({} - 1)^2}}{{({} + 2)^2 - ({} - 2)^2}}",
        n, n, n, n
    );

    let steps = vec![
        "a^2 - b^2 = (a+b)(a-b) を利用する".to_string(),
        format!("分子: (N+1+N-1)(N+1-N+1) = 2N \\times 2 = 4N  (N={})", n),
        format!("分母: (N+2+N-2)(N+2-N+2) = 2N \\times 4 = 8N  (N={})", n),
        "分子/分母 = 4N / 8N".to_string(),
        "答え: \\frac{1}{2}（N の値によらず常に \\frac{1}{2}）".to_string(),
    ];

    ultra_problem(instruction, question, "\\frac{1}{2}".to_string(), steps)
}

/// 超級-15: `A×B + A^2 - C×D - D^2` の値
fn ultra_15(rng: &mut impl Rng) -> Problem {
    // A(A+B) - D(D+C) = A^2 + AB - D^2 - DC
    // 逆: A×B + A^2 - C×D - D^2 = A(B+A) - D(C+D)
    let a = pick(rng, &[25i32, 30, 40, 50]);
    let b = pick(rng, &[3i32, 4, 5, 6]);
    let d = pick(rng, &[20i32, 15, 10, 35]);
    let c = pick(rng, &[3i32, 4, 5, 6]);

    let answer = a * (a + b) - d * (c + d);

    let instruction = "次の計算をしなさい。".to_string();
    let question = format!(
        "{} \\times {} + {}^2 - {} \\times {} - {}^2",
        a, b, a, c, d, d
    );

    let steps = vec![
        format!("A \\times B + A^2 = A(A+B) と変形 (A={})", a),
        format!("C \\times D + D^2 = D(C+D) と変形 (D={})", d),
        format!(
            "{} \\times ({} + {}) - {} \\times ({} + {})",
            a, a, b, d, c, d
        ),
        format!("= {} \\times {} - {} \\times {}", a, a + b, d, c + d),
        format!("答え: {} - {} = {}", a * (a + b), d * (c + d), answer),
    ];

    ultra_problem(instruction, question, format!("{}", answer), steps)
}

/// 超級-16: `N^2(N+1) - N(N+1)^2` = -N(N+1)
fn ultra_16(rng: &mut impl Rng) -> Problem {
    let n = pick(rng, &[10i32, 15, 20, 25, 99]);
    let answer = -n * (n + 1);

    let instruction = "次の計算をしなさい。".to_string();
    let question = format!("{}^2 \\times {} - {} \\times {}^2", n, n + 1, n, n + 1);

    let steps = vec![
        format!(
            "N^2(N+1) - N(N+1)^2 = N(N+1)(N - (N+1)) と因数分解 (N={})",
            n
        ),
        format!("N - (N+1) = -1"),
        format!("= N(N+1) \\times (-1) = -N(N+1)"),
        format!("= -{} \\times {} = {}", n, n + 1, answer),
        format!("答え: {}", answer),
    ];

    ultra_problem(instruction, question, format!("{}", answer), steps)
}

/// 超級-17: `1001×999 + 1001^2 - 1002×998 - 998^2` の値
fn ultra_17() -> Problem {
    // a = 1000 と置く
    // 1001×999 = (a+1)(a-1) = a^2 - 1
    // 1001^2 = (a+1)^2 = a^2 + 2a + 1
    // 1002×998 = (a+2)(a-2) = a^2 - 4
    // 998^2 = (a-2)^2 = a^2 - 4a + 4
    // 合計: (a^2-1) + (a^2+2a+1) - (a^2-4) - (a^2-4a+4)
    // = a^2-1 + a^2+2a+1 - a^2+4 - a^2+4a-4
    // = 6a = 6000
    let answer = 6000i32;

    let instruction = "次の計算をしなさい。".to_string();
    let question = "1001 \\times 999 + 1001^2 - 1002 \\times 998 - 998^2".to_string();

    let steps = vec![
        "a = 1000 と置く".to_string(),
        "1001×999 = (a+1)(a-1) = a^2 - 1".to_string(),
        "1001^2 = (a+1)^2 = a^2 + 2a + 1".to_string(),
        "1002×998 = (a+2)(a-2) = a^2 - 4".to_string(),
        "998^2 = (a-2)^2 = a^2 - 4a + 4".to_string(),
        "全部足す: (a^2-1) + (a^2+2a+1) - (a^2-4) - (a^2-4a+4) = 6a".to_string(),
        "答え: 6 \\times 1000 = 6000".to_string(),
    ];

    ultra_problem(instruction, question, format!("{}", answer), steps)
}

/// 超級-18: `(x+1)(x-1)(x^2+1)` の展開 = x^4 - 1
fn ultra_18() -> Problem {
    let instruction = "次の計算をしなさい。".to_string();
    let question = "(x+1)(x-1)(x^2+1)".to_string();
    let answer = "x^4 - 1".to_string();

    let steps = vec![
        "(x+1)(x-1) を先に計算する".to_string(),
        "= (x^2 - 1)(x^2 + 1)（差の積の公式）".to_string(),
        "さらに (x^2-1)(x^2+1) = (x^2)^2 - 1^2".to_string(),
        "答え: x^4 - 1".to_string(),
    ];

    ultra_problem(instruction, question, answer, steps)
}

/// 超級-19: `(x^2+x+1)(x^2-x+1)` の展開 = x^4 + x^2 + 1
fn ultra_19() -> Problem {
    let instruction = "次の計算をしなさい。".to_string();
    let question = "(x^2 + x + 1)(x^2 - x + 1)".to_string();
    let answer = "x^4 + x^2 + 1".to_string();

    let steps = vec![
        "A = x^2 + 1 と置くと (A+x)(A-x) と読み替えられる".to_string(),
        "= A^2 - x^2 = (x^2+1)^2 - x^2".to_string(),
        "(x^2+1)^2 = x^4 + 2x^2 + 1 を展開".to_string(),
        "x^4 + 2x^2 + 1 - x^2".to_string(),
        "答え: x^4 + x^2 + 1".to_string(),
    ];

    ultra_problem(instruction, question, answer, steps)
}

/// 超級-20: `(a+b+c)^2 - (a^2+b^2+c^2)` の値
fn ultra_20(rng: &mut impl Rng) -> Problem {
    let a = pick(rng, &[2i32, 3, 4, 5]);
    let b = pick(rng, &[3i32, 4, 5, 6]);
    let c = pick(rng, &[1i32, 2, 3, 4]);
    let answer = 2 * (a * b + b * c + c * a);

    let instruction = format!("a={}, b={}, c={} のとき, 次の式の値を求めなさい。", a, b, c);
    let question = "(a+b+c)^2 - (a^2+b^2+c^2)".to_string();

    let steps = vec![
        "(a+b+c)^2 = a^2+b^2+c^2+2ab+2bc+2ca と展開する".to_string(),
        "(a+b+c)^2 - (a^2+b^2+c^2) = 2(ab+bc+ca)".to_string(),
        format!("ab = {}, bc = {}, ca = {}", a * b, b * c, c * a),
        format!(
            "2({} + {} + {}) = 2 \\times {} = {}",
            a * b,
            b * c,
            c * a,
            a * b + b * c + c * a,
            answer
        ),
        format!("答え: {}", answer),
    ];

    ultra_problem(instruction, question, format!("{}", answer), steps)
}

/// 超級-21: `(a-b)^2 + (b-c)^2 + (c-a)^2` の値
fn ultra_21(rng: &mut impl Rng) -> Problem {
    let a = pick(rng, &[5i32, 6, 7, 8]);
    let b = pick(rng, &[2i32, 3, 4, 5]);
    let c = pick(rng, &[1i32, 2, 3]);
    let answer = (a - b) * (a - b) + (b - c) * (b - c) + (c - a) * (c - a);

    let instruction = format!("a={}, b={}, c={} のとき, 次の式の値を求めなさい。", a, b, c);
    let question = "(a-b)^2 + (b-c)^2 + (c-a)^2".to_string();

    let lhs = 2 * (a * a + b * b + c * c - a * b - b * c - c * a);

    let steps = vec![
        "各項を展開する".to_string(),
        format!("(a-b)^2 + (b-c)^2 + (c-a)^2 = 2(a^2+b^2+c^2) - 2(ab+bc+ca)"),
        format!(
            "a^2+b^2+c^2 = {} + {} + {} = {}",
            a * a,
            b * b,
            c * c,
            a * a + b * b + c * c
        ),
        format!(
            "ab+bc+ca = {} + {} + {} = {}",
            a * b,
            b * c,
            c * a,
            a * b + b * c + c * a
        ),
        format!(
            "= 2 \\times {} - 2 \\times {} = {}",
            a * a + b * b + c * c,
            a * b + b * c + c * a,
            lhs
        ),
        format!("答え: {}", answer),
    ];

    ultra_problem(instruction, question, format!("{}", answer), steps)
}

/// 超級-22: `(x^2-x-1)^2 - 2(x^2-x-1) - 3 の値`（2段階置き換え）
fn ultra_22(rng: &mut impl Rng) -> Problem {
    let x_val = pick(rng, &[2i32, 3, 4, 5]);
    let t = x_val * x_val - x_val - 1;
    let answer = (t - 3) * (t + 1);

    let instruction = format!("x = {} のとき, 次の式の値を求めなさい。", x_val);
    let question = "(x^2 - x - 1)^2 - 2(x^2 - x - 1) - 3".to_string();

    let steps = vec![
        "t = x^2 - x - 1 と置く".to_string(),
        format!("t = {}^2 - {} - 1 = {}", x_val, x_val, t),
        "t^2 - 2t - 3 = (t-3)(t+1) と因数分解".to_string(),
        format!("({} - 3)({} + 1) = {} \\times {}", t, t, t - 3, t + 1),
        format!("答え: {}", answer),
    ];

    ultra_problem(instruction, question, format!("{}", answer), steps)
}

/// 超級-23: `(a^2-3a)^2 - 2(a^2-3a) - 8 の値`（2段階置き換え）
fn ultra_23(rng: &mut impl Rng) -> Problem {
    let a_val = pick(rng, &[4i32, 5, 6, 7]);
    let t = a_val * a_val - 3 * a_val;
    let answer = (t - 4) * (t + 2);

    let instruction = format!("a = {} のとき, 次の式の値を求めなさい。", a_val);
    let question = "(a^2 - 3a)^2 - 2(a^2 - 3a) - 8".to_string();

    let steps = vec![
        "t = a^2 - 3a と置く".to_string(),
        format!("t = {}^2 - 3 \\times {} = {}", a_val, a_val, t),
        "t^2 - 2t - 8 = (t-4)(t+2) と因数分解".to_string(),
        format!("({} - 4)({} + 2) = {} \\times {}", t, t, t - 4, t + 2),
        format!("答え: {}", answer),
    ];

    ultra_problem(instruction, question, format!("{}", answer), steps)
}

/// 超級-24: `x + 1/x = A のとき x^2 + 1/x^2 の値`
fn ultra_24(rng: &mut impl Rng) -> Problem {
    // x + 1/x = n のとき x^2 + 1/x^2 = n^2 - 2
    let n = pick(rng, &[3i32, 4, 5, 6]);
    let answer = n * n - 2;

    let instruction = format!(
        "x + \\dfrac{{1}}{{x}} = {} のとき, 次の式の値を求めなさい。",
        n
    );
    let question = "x^2 + \\dfrac{1}{x^2}".to_string();

    let steps = vec![
        format!("\\left(x + \\frac{{1}}{{x}}\\right)^2 = x^2 + 2 + \\frac{{1}}{{x^2}} を利用する"),
        format!("x^2 + \\frac{{1}}{{x^2}} = \\left(x + \\frac{{1}}{{x}}\\right)^2 - 2"),
        format!("= {}^2 - 2 = {} - 2", n, n * n),
        format!("答え: {}", answer),
    ];

    ultra_problem(instruction, question, format!("{}", answer), steps)
}

/// 超級-25: `x - 1/x = A のとき x^2 + 1/x^2 の値`
fn ultra_25(rng: &mut impl Rng) -> Problem {
    let n = pick(rng, &[2i32, 3, 4, 5]);
    let answer = n * n + 2;

    let instruction = format!(
        "x - \\dfrac{{1}}{{x}} = {} のとき, 次の式の値を求めなさい。",
        n
    );
    let question = "x^2 + \\dfrac{1}{x^2}".to_string();

    let steps = vec![
        format!("\\left(x - \\frac{{1}}{{x}}\\right)^2 = x^2 - 2 + \\frac{{1}}{{x^2}} を利用する"),
        format!("x^2 + \\frac{{1}}{{x^2}} = \\left(x - \\frac{{1}}{{x}}\\right)^2 + 2"),
        format!("= {}^2 + 2 = {} + 2", n, n * n),
        format!("答え: {}", answer),
    ];

    ultra_problem(instruction, question, format!("{}", answer), steps)
}

// ============================================================================
// ヘルパー関数
// ============================================================================

/// 超級問題を生成して返す（難易度を Ultra に固定する薄いラッパー）。
fn ultra_problem(
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
fn sign_str(n: i32) -> &'static str {
    if n >= 0 { "+" } else { "-" }
}

/// `ax^2 + bx + c` の LaTeX を組み立てる。
fn format_quadratic(c2: i32, v2: &str, c1: i32, _v1: &str, c0: i32) -> String {
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
fn format_poly4(c3: i32, c2: i32, c1: i32, c0: i32) -> String {
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
fn format_full_expansion(c_x2: i32, c_xy: i32, c_y2: i32, c_x: i32, c_y: i32, c0: i32) -> String {
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
fn format_linear(a: i32, b: i32, var: &str) -> String {
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
fn gcd_i(mut a: i32, mut b: i32) -> i32 {
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
fn parse_decimal(s: &str) -> (i32, i32) {
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
    fn ultra_covers_all_25_patterns() {
        // 十分な回数試行してパニックが起きないことを確認する
        let mut rng = StdRng::seed_from_u64(12345);
        for _ in 0..500 {
            let p = generate_ultra(&mut rng);
            assert!(!p.answer_latex.is_empty());
        }
    }
}
