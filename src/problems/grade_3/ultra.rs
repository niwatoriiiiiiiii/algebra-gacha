//! 3年生 超級問題（全28パターン）。
//!
//! 正答率1%未満を目標とした難関校・数検レベル。
//! 置き換え・対称式・無理数・高次展開・2段階置き換えなど。
//! 係数は大きめに設定してごり押し計算を封じている。

use rand::Rng;

/// 超級パターンをランダムに選んで返す（全28パターン）。
pub fn generate_ultra(rng: &mut impl Rng) -> Problem {
    match rng.gen_range(0..32u8) {
        0  => ultra_01(rng),
        1  => ultra_02(rng),
        2  => ultra_03(rng),
        3  => ultra_04(rng),
        4  => ultra_05(rng),
        5  => ultra_06(rng),
        6  => ultra_07(rng),
        7  => ultra_08(rng),
        8  => ultra_09(rng),
        9  => ultra_10(rng),
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
        24 => ultra_25(rng),
        25 => ultra_26(rng),
        26 => ultra_27(rng),
        27 => ultra_28(rng),
        28 => hard_reciprocal_square(rng),
        29 => hard_cube_difference_factor(rng),
        30 => hard_sum_square_relation(rng),
        _  => hard_symmetric_hard(rng),
    }
}

use super::helpers::{gcd_i, ultra_problem};
use crate::problems::{Problem, pick};

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
    // 2x+y が大きな値になるよう設定（ごり押し計算不可）
    let x_val = pick(rng, &[30i32, 40, 50, 60]);
    let y_val = pick(rng, &[-19i32, -29, -39, -49]);

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
    // 答えは定数1。大きな数を設定して「直接計算しようとしても無理」と気づかせる
    let a_val = pick(rng, &[51i32, 73, 112, 134]);
    let b_val = pick(rng, &[31i32, 42, 53, 64]);

    let instruction = format!("a={}, b={} のとき, 次の式の値を求めなさい。", a_val, b_val);
    let question = "(a + 2b - 1)^2 - (a + 2b)(a + 2b - 2)".to_string();

    let steps = vec![
        "t = a + 2b と置く".to_string(),
        "(t-1)^2 - t(t-2)".to_string(),
        "t^2 - 2t + 1 - t^2 + 2t".to_string(),
        "答え: 1（a, b の値によらず常に 1）".to_string(),
    ];

    ultra_problem(instruction, question, "1".to_string(), steps)
}

/// 超級-04: `(x-y+3)^2 - (x-y)^2 - 6(x-y)` = 9（定数化）
fn ultra_04(rng: &mut impl Rng) -> Problem {
    // 答えは定数9。大きな数でごり押しを封じる
    let x_val = pick(rng, &[100i32, 200, 300, 500]);
    let y_val = pick(rng, &[71i32, 131, 231, 431]);

    let instruction = format!("x={}, y={} のとき, 次の式の値を求めなさい。", x_val, y_val);
    let question = "(x - y + 3)^2 - (x-y)^2 - 6(x-y)".to_string();

    let steps = vec![
        "t = x - y と置く".to_string(),
        "(t+3)^2 - t^2 - 6t".to_string(),
        "t^2 + 6t + 9 - t^2 - 6t".to_string(),
        "答え: 9（x, y の値によらず常に 9）".to_string(),
    ];

    ultra_problem(instruction, question, "9".to_string(), steps)
}

/// 超級-05: `x+y=A, xy=B のとき x^2+y^2 の値`
fn ultra_05(rng: &mut impl Rng) -> Problem {
    // x^2+y^2 が大きくなる数値を選ぶ（(x+y)^2-2xy の公式を使わないと無理）
    let x = pick(rng, &[20i32, 30, 40, 50]);
    let y = pick(rng, &[13i32, 17, 23, 31]);
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
    // (x-y)^2+2xy の公式を使わないと計算が大変になる数値
    let x = pick(rng, &[30i32, 40, 50, 60]);
    let y = pick(rng, &[13i32, 17, 23, 31]);
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
    // 2(x^2+y^2)-(x+y)^2 の公式を使わないと解けない規模の数値
    let x = pick(rng, &[20i32, 30, 40, 50]);
    let y = pick(rng, &[13i32, 17, 23, 31]);
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

/// 超級-15: `A(A+1) - (A-1)(A+2)` の値（答えは常に 2）
///
/// A が 4桁のため直接計算は事実上不可能。
/// A(A+1) - (A-1)(A+2) = A²+A - A²-A+2 = 2 という恒等式を使う。
fn ultra_15(rng: &mut impl Rng) -> Problem {
    // A を4桁の数に設定。A² は約400万 → 筆算でも計算不可能
    let a = pick(rng, &[2031i32, 3141, 1999, 2500, 1234, 3000]);

    let instruction = "次の計算をしなさい。".to_string();
    let question = format!(
        "{} \\times {} - {} \\times {}",
        a, a + 1, a - 1, a + 2
    );

    let steps = vec![
        format!("t = {} と置く", a),
        "t(t+1) - (t-1)(t+2) と読み替える".to_string(),
        "= t² + t - (t² + t - 2)".to_string(),
        "= t² + t - t² - t + 2".to_string(),
        "答え: 2（A の値によらず常に 2）".to_string(),
    ];

    ultra_problem(instruction, question, "2".to_string(), steps)
}


/// 超級-16: `N²(N+1) - N(N+1)²` の値
///
/// N が 3桁のため N²(N+1) は7桁 → 直接計算は不可能。
/// N²(N+1) - N(N+1)² = N(N+1)(N-(N+1)) = -N(N+1)
fn ultra_16(rng: &mut impl Rng) -> Problem {
    // N を3桁に設定。N²(N+1) は最大約10億 → ごり押し不可
    let n = pick(rng, &[300i32, 400, 500, 999]);
    let answer = -n * (n + 1);

    let instruction = "次の計算をしなさい。".to_string();
    let question = format!(
        "{}^2 \\times {} - {} \\times {}^2",
        n, n + 1, n, n + 1
    );

    let steps = vec![
        format!("N = {} と置く", n),
        "N²(N+1) - N(N+1)² = N(N+1)(N - (N+1)) と因数分解する".to_string(),
        "N - (N+1) = -1".to_string(),
        format!("= N(N+1) × (-1) = -{} × {} = {}", n, n + 1, answer),
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
    // 2(ab+bc+ca) の公式を使わないと手計算が困難な数値
    let a = pick(rng, &[20i32, 30, 40, 50]);
    let b = pick(rng, &[13i32, 17, 23, 31]);
    let c = pick(rng, &[7i32, 11, 13, 19]);
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
    // 2(a^2+b^2+c^2-ab-bc-ca) の公式を使わないと手計算が困難
    let a = pick(rng, &[30i32, 40, 50, 60]);
    let b = pick(rng, &[13i32, 17, 23, 31]);
    let c = pick(rng, &[7i32, 11, 13, 17]);
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
    // t=x^2-x-1 が大きくなりごり押し計算が不可能になる数値
    let x_val = pick(rng, &[20i32, 30, 40, 50]);
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
    // t=a^2-3a が大きくなりごり押し計算が不可能になる数値
    let a_val = pick(rng, &[20i32, 30, 40, 50]);
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
    // nを大きくして直接計算を封じる
    let n = pick(rng, &[10i32, 12, 15, 20]);
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
    // x - 1/x = n のとき x^2 + 1/x^2 = n^2 + 2
    // nを大きくして直接計算を封じる
    let n = pick(rng, &[10i32, 12, 15, 20]);
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


fn ultra_26(rng: &mut impl Rng) -> Problem {
    let x = pick(rng, &[20i32, 30, 40, 50]);
    let y = pick(rng, &[13i32, 17, 23, 31]);
    let sum = x + y;
    let prod = x * y;
    // x^3+y^3 = (x+y)((x+y)^2 - 3xy)
    let answer_val = sum * (sum*sum - 3*prod);

    let instruction = format!(
        "x + y = {}, xy = {} のとき, 次の式の値を求めなさい。",
        sum, prod
    );
    let question = "x^3 + y^3".to_string();
    let answer = format!("{}", answer_val);

    let steps = vec![
        "x^3 + y^3 = (x+y)(x^2-xy+y^2) と因数分解する".to_string(),
        format!("x^2 - xy + y^2 = (x+y)^2 - 3xy = {}^2 - 3\\times{} = {}",
            sum, prod, sum*sum - 3*prod),
        format!("x^3 + y^3 = {} \\times {}", sum, sum*sum - 3*prod),
        format!("答え: {}", answer),
    ];

    ultra_problem(instruction, question, answer, steps)
}

/// 超級-27: (a^2-b)(a^2+b) - a^2(a^2-b^2+b) 型
fn ultra_27(rng: &mut impl Rng) -> Problem {
    // a=M, b=N として計算
    // (a^2-b)(a^2+b) - a^2(a^2-b^2+b)
    // = a^4 - b^2 - a^4 + a^2 b^2 - a^2 b
    // = a^2 b^2 - a^2 b - b^2
    // = b(a^2 b - a^2 - b)
    // = b(a^2(b-1) - b)
    // 整数になる a,b を先に決める
    let a = pick(rng, &[5i32, 7, 10, 13]);
    let b = pick(rng, &[3i32, 4, 5, 6]);

    let a2 = a * a;
    let answer_val = a2*b*b - a2*b - b*b;

    // a=M として具体値を代入する問題
    let inst_concrete = format!(
        "a = {} のとき, 次の式の値を求めなさい。",
        a
    );
    let q_concrete = format!(
        "(a^2 - {})(a^2 + {}) - a^2(a^2 - {}a + {})",
        b, b, b, b
    );

    let steps = vec![
        format!("(a^2)^2 - {}^2 - a^2(a^2 - {}a + {})", b, b, b),
        format!("= a^4 - {} - a^4 + {}a^3 - {}a^2", b*b, b, b),
        format!("= {}a^3 - {}a^2 - {}", b, b, b*b),
        format!("a = {} を代入: {}\\times{}^3 - {}\\times{}^2 - {}",
            a, b, a, b, a, b*b),
        format!("= {} - {} - {} = {}", b*a*a*a, b*a*a, b*b, answer_val),
    ];

    ultra_problem(inst_concrete, q_concrete, format!("{}", answer_val), steps)
}

/// 超級-28: x+y=A, x^3+y^3=B のとき xy の値を求める
fn ultra_28(rng: &mut impl Rng) -> Problem {
    // x+y=S, xy=P として x^3+y^3 = S(S^2-3P)
    // → P = (S^3 - (x^3+y^3)) / (3S)
    // 整数になる S, P を先に決める
    let x = pick(rng, &[20i32, 30, 40, 50]);
    let y = pick(rng, &[13i32, 17, 23, 29]);
    let sum_val = x + y;
    let prod_val = x * y;
    let cube_sum = sum_val * (sum_val*sum_val - 3*prod_val);

    let instruction = format!(
        "x + y = {}, x^3 + y^3 = {} のとき, xy の値を求めなさい。",
        sum_val, cube_sum
    );
    let question = "xy".to_string();
    let answer = format!("{}", prod_val);

    let steps = vec![
        "x^3 + y^3 = (x+y)(x^2-xy+y^2) = (x+y)((x+y)^2 - 3xy) を利用する".to_string(),
        format!("{} = {}\\times({}^2 - 3xy)", cube_sum, sum_val, sum_val),
        format!("{} = {}\\times({} - 3xy)", cube_sum, sum_val, sum_val*sum_val),
        format!("{} - 3xy \\times {} = {}", sum_val*sum_val, sum_val, cube_sum),
        format!("3xy = {} - {} = {}", sum_val*sum_val*sum_val, cube_sum, sum_val*sum_val*sum_val - cube_sum),
        format!("xy = {} ÷ 3 = {}", sum_val*sum_val*sum_val - cube_sum, prod_val),
        format!("答え: {}", answer),
    ];

    ultra_problem(instruction, question, answer, steps)
}

// ============================================================================
// Hard から昇格した Ultra パターン
// ============================================================================

/// x + 1/x = n (整数) のとき (x - 1/x)^2 の値
fn hard_reciprocal_square(rng: &mut impl Rng) -> Problem {
    // x + 1/x = n → x^2 + 2 + 1/x^2 = n^2 → x^2 + 1/x^2 = n^2-2
    // (x - 1/x)^2 = x^2 - 2 + 1/x^2 = n^2 - 4
    let n = pick(rng, &[3i32, 4, 5, 6, 7]);
    let answer = n * n - 4;

    let instruction = format!("x + \\dfrac{{1}}{{x}} = {} のとき, 次の式の値を求めなさい。", n);
    let question = "\\left(x - \\dfrac{1}{x}\\right)^2".to_string();

    let steps = vec![
        format!("\\left(x - \\frac{{1}}{{x}}\\right)^2 = \\left(x + \\frac{{1}}{{x}}\\right)^2 - 4 を利用する"),
        format!("= {}^2 - 4 = {} - 4", n, n*n),
        format!("答え: {}", answer),
    ];

    ultra_problem(instruction, question, format!("{}", answer), steps)
}

/// 3乗の差の積: (a-b)(a^2+ab+b^2) = a^3 - b^3 の利用
fn hard_cube_difference_factor(rng: &mut impl Rng) -> Problem {
    let a = pick(rng, &[2i32, 3, 4, 5, 10]);
    let b = pick(rng, &[1i32, 2, 3]);
    if a == b { return hard_cube_difference_factor(rng); }

    // (a-b)(a^2+ab+b^2) = a^3 - b^3
    let answer = a*a*a - b*b*b;

    let instruction = "次の計算をしなさい。".to_string();
    let question = format!("({} - {})({} + {} \\times {} + {})", a, b, a*a, a, b, b*b);

    let steps = vec![
        "(a-b)(a^2+ab+b^2) = a^3 - b^3 の公式を利用する".to_string(),
        format!("a = {}, b = {}", a, b),
        format!("= {}^3 - {}^3 = {} - {} = {}", a, b, a*a*a, b*b*b, answer),
    ];

    ultra_problem(instruction, question, format!("{}", answer), steps)
}

/// x^2 + y^2 と (x+y)^2 の関係を使った問題
fn hard_sum_square_relation(rng: &mut impl Rng) -> Problem {
    // x^2 + y^2 = A, (x+y)^2 = B のとき xy の値を求める
    // (x+y)^2 = x^2 + 2xy + y^2 → xy = (B-A)/2
    let x = pick(rng, &[10i32, 15, 20, 25, 30]);
    let y = pick(rng, &[7i32, 11, 13, 17]);
    let sum_sq = (x + y) * (x + y);
    let sq_sum = x*x + y*y;
    let answer = x * y;

    let instruction = format!(
        "x^2 + y^2 = {}, (x+y)^2 = {} のとき, xy の値を求めなさい。",
        sq_sum, sum_sq
    );
    let question = "xy".to_string();

    let steps = vec![
        "(x+y)^2 = x^2 + 2xy + y^2 を利用する".to_string(),
        format!("{} = {} + 2xy", sum_sq, sq_sum),
        format!("2xy = {} - {} = {}", sum_sq, sq_sum, sum_sq - sq_sum),
        format!("xy = {} ÷ 2 = {}", sum_sq - sq_sum, answer),
    ];

    ultra_problem(instruction, question, format!("{}", answer), steps)
}

/// 2変数の対称式: a^2 + b^2 を (a+b) と ab から求める（やや複雑な値）
fn hard_symmetric_hard(rng: &mut impl Rng) -> Problem {
    // a+b = p, ab = q のとき a^3 - b^3 を求める
    // a^3 - b^3 = (a-b)(a^2+ab+b^2) = (a-b)((a+b)^2 - 3ab)
    // (a-b)^2 = (a+b)^2 - 4ab = p^2 - 4q
    // (a-b) = √(p^2-4q) となり無理数になるケースが多い
    // 代わりに a^2 + b^2 を求める問題で大きな数
    let a = pick(rng, &[20i32, 25, 30, 40]);
    let b = pick(rng, &[13i32, 17, 19, 23]);
    let sum = a + b;
    let prod = a * b;

    let instruction = format!("a + b = {}, ab = {} のとき, 次の式の値を求めなさい。", sum, prod);
    let question = "a^2 + b^2 - ab".to_string();
    // a^2 + b^2 - ab = (a+b)^2 - 3ab
    let answer_val = sum*sum - 3*prod;

    let steps = vec![
        "a^2 + b^2 - ab = (a+b)^2 - 3ab と変形する".to_string(),
        format!("= {}^2 - 3 \\times {}", sum, prod),
        format!("= {} - {} = {}", sum*sum, 3*prod, answer_val),
    ];

    ultra_problem(instruction, question, format!("{}", answer_val), steps)
}
