//! 3年生 初級問題。
//!
//! パターン: 分配法則・(x+a)(x+b)・平方・差の積・置き換え・一般二次・3因数・√

use rand::Rng;

/// 初級パターンをランダムに選んで返す。
pub fn generate_easy(rng: &mut impl Rng) -> Problem {
    match rng.gen_range(0..8u8) {
        0 => easy_distributive(rng),
        1 => easy_xaxb(rng),
        2 => easy_square(rng),
        3 => easy_difference_of_squares(rng),
        4 => easy_substitution(rng),
        5 => easy_general_quadratic(rng),
        6 => easy_triple_product(rng),
        _ => easy_sqrt_expand(rng),
    }
}
use super::helpers::{
    format_full_expansion, format_linear, format_poly3, format_poly4,
    format_poly4_full, format_quadratic, sign_str,
};
use crate::problems::{coeff_str, Difficulty, Problem, pick, random_sign};

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
        "({})(x^2 {} {}x {} {})",
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

fn easy_general_quadratic(rng: &mut impl Rng) -> Problem {
    let coeffs: &[i32] = &[1, 2, 3];
    let consts: &[i32] = &[-4, -3, -2, -1, 1, 2, 3, 4];

    let a = pick(rng, coeffs);
    let b = pick(rng, consts);
    let c = pick(rng, coeffs);
    let d = pick(rng, consts);

    // (ax+b)(cx+d) = ac x^2 + (ad+bc)x + bd
    let c2 = a * c;
    let c1 = a * d + b * c;
    let c0 = b * d;

    let instruction = "次の計算をしなさい。".to_string();
    let fmt_inner = |coeff: i32, con: i32| -> String {
        let x = if coeff == 1 { "x".to_string() } else { format!("{}x", coeff) };
        if con > 0 { format!("{} + {}", x, con) }
        else { format!("{} - {}", x, con.abs()) }
    };
    let question = format!("({})({})", fmt_inner(a, b), fmt_inner(c, d));

    let answer = format_poly3(c2, c1, c0);

    let steps = vec![
        format!("{}x \\times {}x = {}x^2", a, c, c2),
        format!("{}x \\times {} + {} \\times {}x = {}x", a, d, b, c, c1),
        format!("{} \\times {} = {}", b, d, c0),
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

/// (x+a)(x+b)(x+c) 3因数の展開
fn easy_triple_product(rng: &mut impl Rng) -> Problem {
    let vals: &[i32] = &[-3, -2, -1, 1, 2, 3];
    let a = pick(rng, vals);
    let b = pick(rng, vals);
    let c = pick(rng, vals);

    // (x+a)(x+b)(x+c) = x^3 + (a+b+c)x^2 + (ab+bc+ca)x + abc
    let p = a + b + c;
    let q = a*b + b*c + c*a;
    let r = a * b * c;

    let instruction = "次の計算をしなさい。".to_string();
    let fmt_factor = |k: i32| -> String {
        if k >= 0 { format!("x + {}", k) } else { format!("x - {}", k.abs()) }
    };
    let question = format!("({})({})({})", fmt_factor(a), fmt_factor(b), fmt_factor(c));
    let answer = format_poly4_full(1, p, q, r);

    let steps = vec![
        format!("まず (x+{})(x+{}) を展開する", a, b),
        format!("= x^2 + {}x + {}", a+b, a*b),
        format!("次に (x^2 + {}x + {})(x+{}) を展開する", a+b, a*b, c),
        format!("x^3 の係数: 1"),
        format!("x^2 の係数: {} + {} = {}", a+b, c, p),
        format!("x の係数: {} + {}\\times{} = {}", a*b, a+b, c, q),
        format!("定数: {} \\times {} = {}", a*b, c, r),
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

/// (√a + √b)(√a - √b) と平方根の展開
fn easy_sqrt_expand(rng: &mut impl Rng) -> Problem {
    // (√a + √b)(√a - √b) = a - b
    let vals: &[u32] = &[2, 3, 5, 6, 7, 10, 11, 13];
    let a = pick(rng, vals);
    let b = pick(rng, vals);
    if a == b { return easy_sqrt_expand(rng); }

    let answer_val = a as i32 - b as i32;

    let instruction = "次の計算をしなさい。".to_string();
    let question = format!(
        "(\\sqrt{{{}}} + \\sqrt{{{}}})(\\sqrt{{{}}} - \\sqrt{{{}}})",
        a, b, a, b
    );
    let answer = format!("{}", answer_val);

    let steps = vec![
        format!("(x+y)(x-y) = x^2 - y^2 で x=\\sqrt{{{}}}, y=\\sqrt{{{}}}", a, b),
        format!("= (\\sqrt{{{}}})^2 - (\\sqrt{{{}}})^2", a, b),
        format!("= {} - {}", a, b),
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
