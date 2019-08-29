fn minmax(a: i32, b: i32) -> (i32, i32) {
    if b < a {
        (b, a)
    } else {
        (a, b)
    }
}

fn gcd(a: i32, b: i32) -> i32 {
    if a == 0 || b == 0 {
        return 1;
    }

    let (mut lower, mut greater) = minmax(a, b);

    let mut modulo = greater % lower;
    while modulo != 0 {
        greater = lower;
        lower = modulo;

        modulo = greater % lower;
    }

    lower
}

pub fn lcm(a: i32, b: i32) -> i32 {
    let gcd = gcd(a, b);

    (a * b) / gcd
}

pub fn normalize_sign(n: i32, d: i32) -> (i32, i32) {
    if d < 0 {
        (-n, -d)
    } else {
        (n, d)
    }
}

pub fn reduce(a: i32, b:i32) -> (i32, i32) {
    let gcd = gcd(a.abs(), b);

    (a / gcd, b / gcd)
}
