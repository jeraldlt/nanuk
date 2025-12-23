use crate::stats::t_distribution::t_distribution_cdf;
use polars::prelude::*;

pub enum Alternative {
    TwoSided,
    Greater,
    Less,
}

// pub fn t_test(data: &DataFrame, dv: Vec<&str>, between: Option<&str>, alt: Alternative) {}

pub fn one_sample_t_test(x: &Series, mu: f64, alt: Alternative) -> DataFrame {
    let dof = x.len() - 1;
    let std_err = x.std(1).unwrap() / (x.len() as f64).sqrt();
    let t = (x.mean().unwrap() - mu) / std_err;
    let d = t / (x.len() as f64).sqrt();

    // let t_critical = t_distribution_cdf(0.975, dof as f64);
    // let ci_lower = x.mean().unwrap() + t_critical * std_err;
    // let ci_lower = (t - t_critical) * std_err;

    let (alternative, p) = match alt {
        Alternative::TwoSided => ("two-sided", t_distribution_cdf(t, dof as f64) * 2.0),
        _ => ("one-sided", t_distribution_cdf(t, dof as f64)),
    };

    df!(
        "T" => [t],
        "dof" => [dof as i64],
        "alternative" => [alternative],
        "p-val" => [p],
        "CI95%" => [f64::NAN],
        "cohen-d" => [d],
        "BF10" => [f64::NAN],
        "power" => [f64::NAN],
    )
    .unwrap()
}

pub fn paired_t_test(x: &Series, y: &Series, alt: Alternative) -> DataFrame {
    let diff = (x - y).unwrap();

    one_sample_t_test(&diff, 0.0, alt)
}
