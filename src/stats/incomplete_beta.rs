use num_traits::{Float, NumCast};

pub fn incomplete_beta<T: Float + Copy>(a: T, b: T, x: T) -> T {
    if a <= T::zero() || b <= T::zero() {
        panic!("Bad a or b in routine betai");
    }

    if x < T::zero() || x > T::one() {
        panic!("Bad x in routine betai");
    }

    if x == T::zero() || x == T::one() {
        return x;
    }

    // ln_gamma is not implemented for the Float trait.
    // This function casts to the higest precision supported float (f64),
    // conducts the gammaln operation, and casts back to T
    //
    // Question: Is it better to cast a, b, and x to f64 and cast final result to T on return?
    fn gammln<T: Float + Copy>(x: T) -> T {
        let primative: f64 = NumCast::from(x).unwrap();
        return T::from(primative.ln_gamma().0).unwrap();
    }

    let bt = T::exp(gammln(a + b) - gammln(a) - gammln(b) + a * T::ln(x) + b * T::ln(T::one() - x));

    if x < (a + T::one()) / (a + b + T::one() + T::one()) {
        return bt * incomplete_beta_cf(a, b, x) / a;
    }

    return T::one() - bt * incomplete_beta_cf(b, a, T::one() - x) / b;
}

pub fn incomplete_beta_cf<T: Float + Copy>(a: T, b: T, x: T) -> T {
    let qab = a + b;
    let qap = a + T::one();
    let qam = a - T::one();
    let mut c = T::one();
    let mut d = T::one() - qab * x / qap;
    if T::abs(d) < T::min_positive_value() {
        d = T::min_positive_value();
    }

    d = T::one() / d;

    let mut h = d;

    for m in 1..10000 {
        let m = T::from(m).unwrap();

        let m2 = m + m;

        let aa = m * (b - m) * x / ((qam + m2) * (a + m2));

        d = T::one() + aa * d;
        if T::abs(d) < T::min_positive_value() {
            d = T::min_positive_value();
        }
        c = T::one() + aa / c;
        if T::abs(c) < T::min_positive_value() {
            c = T::min_positive_value();
        }
        d = T::one() / d;
        h = h * d * c;

        let aa = -(a + m) * (qab + m) * x / ((a + m2) * (qap + m2));

        d = T::one() + aa * d;
        if T::abs(d) < T::min_positive_value() {
            d = T::min_positive_value();
        }
        c = T::one() + aa / c;
        if T::abs(c) < T::min_positive_value() {
            c = T::min_positive_value();
        }
        d = T::one() / d;

        let del = d * c;

        h = h * del;

        if T::abs(del - T::one()) <= T::epsilon() {
            break;
        }
    }

    return h;
}

#[cfg(test)]
mod tests {
    use super::incomplete_beta;
    use std::panic;

    #[test]
    fn incomplete_beta_invalid_input() {
        // a < 0
        let result = panic::catch_unwind(|| {
            incomplete_beta(-1.0, 0.0, 0.0);
        });
        assert!(result.is_err());

        // b < 0
        let result = panic::catch_unwind(|| {
            incomplete_beta(0.0, -1.0, 0.0);
        });
        assert!(result.is_err());

        // x < 0
        let result = panic::catch_unwind(|| {
            incomplete_beta(0.0, 0.0, -1.0);
        });
        assert!(result.is_err());

        // x > 1
        let result = panic::catch_unwind(|| {
            incomplete_beta(0.0, 0.0, 2.0);
        });
        assert!(result.is_err());
    }

    #[test]
    fn incomplete_beta_properties() {
        // I_0(a, b) == 0
        let result = incomplete_beta(1.0, 1.0, 0.0);
        assert_eq!(result, 0.0);

        // I_1(a, b) == 1
        let result = incomplete_beta(1.0, 1.0, 1.0);
        assert_eq!(result, 1.0);

        // I_x(1, b) == 1 - (1 - x) ^ b
        let result = incomplete_beta(1.0, 2.0, 0.5);
        assert_eq!(result, 0.75);

        // I_x(a, 1) == x ^ a
        let result = incomplete_beta(2.0, 1.0, 0.5);
        assert_eq!(result, 0.25);

        // I_x(a, b) == I_{1-x}(b, a)
        let result_a = incomplete_beta(2.0, 1.0, 0.75);
        let result_b = 1.0 - incomplete_beta(1.0, 2.0, 0.25);
        assert_eq!(result_a, result_b);
    }
}
