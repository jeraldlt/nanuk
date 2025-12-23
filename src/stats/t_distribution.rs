use num_traits::Float;

use crate::stats::incomplete_beta::incomplete_beta;

pub fn t_distribution_cdf<T: Float + Copy>(t: T, dof: T) -> T {
    if t == T::zero() {
        return T::from(0.5).unwrap();
    } else {
        let x = dof / (t.powi(2) + dof);
        let two = T::one() + T::one();
        let val = T::one() - incomplete_beta(dof / two, T::one() / two, x) / two;

        return if t < T::zero() { val } else { T::one() - val };
    }

    // todo!("Implement t_distribution_cdf for t <= 0.0");

    // if t == T::zero() {
    //     return T::from(0.5).unwrap();
    // } else if (t < T::zero()) {
    //     return T::from(0.5).unwrap()
    //         * (T::one() - incomplete_beta(T::one(), dof, t * t / (dof + t * t)));
    // } else {
    //     return T::from(0.5).unwrap()
    //         * (T::one() + incomplete_beta(T::one(), dof, t * t / (dof + t * t)));
    // }
}

#[cfg(test)]
mod tests {
    // use super::t_distribution_cdf;

    // #[test]
    // fn t_distribution_cdf_samples() {
    //     let result = t_distribution_cdf(1.8, 20.0);
    //     println!("{}", result);
    //     assert_eq!(result, 0.43482);
    // }
}
