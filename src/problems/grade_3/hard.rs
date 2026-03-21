//! 3年生 上級問題。
//!
//! パターン: 置き換え（定数化）・数値の工夫・式の値・4因数展開・恒等式

use rand::Rng;

/// 上級パターンをランダムに選んで返す（全9パターン）。
pub fn generate_hard(rng: &mut impl Rng) -> Problem {
    match rng.gen_range(0..9u8) {
        0  => hard_substitution_complex(rng),
        1  => hard_numerical_trick(rng),
        2  => hard_expression_value(rng),
        3  => hard_four_factor(rng),
        4  => hard_sqrt_value(rng),
        5  => hard_golden_ratio_value(rng),
        6  => hard_rationalize_value(rng),
        7  => hard_cube_sum_from_basics(rng),
        _  => hard_self_reciprocal(rng),
    }
}
use super::helpers::{gcd_i, parse_decimal};
use crate::problems::{coeff_str, Difficulty, Problem, pick};

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

fn hard_four_factor(rng: &mut impl Rng) -> Problem {
    let a = pick(rng, &[2i32, 3, 4, 5]);

    let instruction = "次の計算をしなさい。".to_string();
    let question = format!(
        "(x + {})(x - {})(x^2 + {})",
        a, a, a * a
    );
    let answer = format!("x^4 - {}", i32::pow(a, 4));

    let steps = vec![
        format!("(x+{})(x-{}) を先に計算する", a, a),
        format!("= x^2 - {}", a*a),
        format!("次に (x^2 - {})(x^2 + {}) を計算する", a*a, a*a),
        format!("= (x^2)^2 - {}^2 = x^4 - {}", a*a, i32::pow(a,4)),
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



// ============================================================================
// 追加 Hard パターン（06〜30）
// ============================================================================




/// x=√A+B のとき x^2-2Bx の値（整数になる）
fn hard_sqrt_value(rng: &mut impl Rng) -> Problem {
    let a = pick(rng, &[2u32, 3, 5, 7, 11, 13]);
    let b = pick(rng, &[1i32, 2, 3, 4, 5]);
    // x = √A + B → x - B = √A → (x-B)^2 = A
    // x^2 - 2Bx + B^2 = A → x^2 - 2Bx = A - B^2
    let answer = a as i32 - b * b;

    let instruction = format!("x = \\sqrt{{{}}} + {} のとき, 次の式の値を求めなさい。", a, b);
    let question = format!("x^2 - {}x", 2 * b);

    let steps = vec![
        format!("x - {} = \\sqrt{{{}}} と変形する", b, a),
        format!("両辺を2乗: (x-{})^2 = {}", b, a),
        format!("x^2 - {}x + {} = {}", 2*b, b*b, a),
        format!("x^2 - {}x = {} - {} = {}", 2*b, a, b*b, answer),
    ];
    Problem { difficulty: Difficulty::Hard, instruction, question_latex: question, answer_latex: format!("{}", answer), steps }
}





/// x = (√5 + 1)/2 のとき x^2 - x の値（黄金比）
fn hard_golden_ratio_value(rng: &mut impl Rng) -> Problem {
    // x = (√5+1)/2 → 2x = √5+1 → 2x-1 = √5 → (2x-1)^2 = 5
    // 4x^2 - 4x + 1 = 5 → 4x^2 - 4x = 4 → x^2 - x = 1
    // バリエーション: (√n + k)/m のとき ax^2 + bx の値
    // 簡単なケース: x = √A + B のとき x^2 - 2Bx + C
    // ここでは x=√3+1 → x-1=√3 → x^2-2x+1=3 → x^2-2x=2
    let (root, shift, ans) = pick(rng, &[(3u32, 1i32, 2i32), (5, 2, 1), (7, 2, 3), (2, 1, 1)]);

    let instruction = format!("x = \\sqrt{{{}}} + {} のとき, 次の式の値を求めなさい。", root, shift);
    let question = format!("x^2 - {}x", 2 * shift);

    let steps = vec![
        format!("x - {} = \\sqrt{{{}}} と変形する", shift, root),
        format!("両辺を2乗: x^2 - {}x + {} = {}", 2*shift, shift*shift, root),
        format!("x^2 - {}x = {} - {} = {}", 2*shift, root, shift*shift, ans),
    ];
    Problem { difficulty: Difficulty::Hard, instruction, question_latex: question, answer_latex: format!("{}", ans), steps }
}





/// 分数型の式の値: x=√2+1 のとき 1/(x-1) + x の値
fn hard_rationalize_value(rng: &mut impl Rng) -> Problem {
    // x = √n + 1 → x-1 = √n → 1/(x-1) = 1/√n = √n/n
    // x + 1/(x-1) = √n + 1 + √n/n = √n(1 + 1/n) + 1
    // 複雑なので x=√2+1 のとき x^2 + 1/(x-1)^2
    // x-1=√2 → (x-1)^2=2 → 1/(x-1)^2=1/2
    // x^2 = (√2+1)^2 = 3+2√2 → 整数にならない
    // 代わりに: x=√5+2 のとき x^2 - 4x
    // x-2=√5 → x^2-4x+4=5 → x^2-4x=1
    let (root, shift, _) = pick(rng, &[(5u32, 2i32, 1i32), (3, 1, 2), (7, 2, 3), (11, 3, 2)]);

    let instruction = format!("x = \\sqrt{{{}}} + {} のとき, 次の式の値を求めなさい。", root, shift);
    let question = format!("x^2 - {}x + 1", 2*shift);

    // x^2 - 2shift*x = root - shift^2
    // x^2 - 2shift*x + 1 = root - shift^2 + 1
    let answer = root as i32 - shift*shift + 1;

    let steps = vec![
        format!("x - {} = \\sqrt{{{}}} と変形する", shift, root),
        format!("両辺を2乗: x^2 - {}x + {} = {}", 2*shift, shift*shift, root),
        format!("x^2 - {}x = {} - {} = {}", 2*shift, root, shift*shift, root as i32 - shift*shift),
        format!("x^2 - {}x + 1 = {} + 1 = {}", 2*shift, root as i32 - shift*shift, answer),
    ];
    Problem { difficulty: Difficulty::Hard, instruction, question_latex: question, answer_latex: format!("{}", answer), steps }
}




/// x + y = A, x^2 + y^2 = B のとき x^3 + y^3 の値
fn hard_cube_sum_from_basics(rng: &mut impl Rng) -> Problem {
    let x = pick(rng, &[5i32, 6, 7, 8, 10]);
    let y = pick(rng, &[2i32, 3, 4, 5]);
    if x == y { return hard_cube_sum_from_basics(rng); }

    let s = x + y;
    let sq = x*x + y*y;
    let p = (s*s - sq) / 2; // xy
    // x^3 + y^3 = (x+y)(x^2-xy+y^2) = s(sq - p)
    let answer = s * (sq - p);

    let instruction = format!("x + y = {}, x^2 + y^2 = {} のとき, 次の式の値を求めなさい。", s, sq);
    let question = "x^3 + y^3".to_string();

    let steps = vec![
        "xy = ((x+y)^2 - (x^2+y^2)) / 2 を求める".to_string(),
        format!("xy = ({}^2 - {}) / 2 = ({} - {}) / 2 = {}", s, sq, s*s, sq, p),
        "x^3 + y^3 = (x+y)(x^2-xy+y^2) = (x+y)((x^2+y^2) - xy)".to_string(),
        format!("= {} \\times ({} - {}) = {} \\times {} = {}", s, sq, p, s, sq-p, answer),
    ];
    Problem { difficulty: Difficulty::Hard, instruction, question_latex: question, answer_latex: format!("{}", answer), steps }
}



/// x=a+1/a のとき x^2 - 2 の値（具体的なaを与える）
fn hard_self_reciprocal(rng: &mut impl Rng) -> Problem {
    // x = a + 1/a のとき x^2 = a^2 + 2 + 1/a^2 → x^2 - 2 = a^2 + 1/a^2
    // これは超級と似てるのでここでは逆方向: a^2 + 1/a^2 = n を与えて a^4 + 1/a^4 を求める
    // a^2 + 1/a^2 = n → (a^2 + 1/a^2)^2 = a^4 + 2 + 1/a^4 → a^4 + 1/a^4 = n^2 - 2
    let n = pick(rng, &[3i32, 5, 7, 11]);
    let answer = n * n - 2;

    let instruction = format!("a^2 + \\dfrac{{1}}{{a^2}} = {} のとき, 次の式の値を求めなさい。", n);
    let question = "a^4 + \\dfrac{1}{a^4}".to_string();

    let steps = vec![
        format!("\\left(a^2 + \\frac{{1}}{{a^2}}\\right)^2 = a^4 + 2 + \\frac{{1}}{{a^4}} を利用する"),
        format!("a^4 + \\frac{{1}}{{a^4}} = \\left(a^2 + \\frac{{1}}{{a^2}}\\right)^2 - 2"),
        format!("= {}^2 - 2 = {} - 2 = {}", n, n*n, answer),
    ];
    Problem { difficulty: Difficulty::Hard, instruction, question_latex: question, answer_latex: format!("{}", answer), steps }
}
