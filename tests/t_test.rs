use nanuk::t_test::*;
use polars::prelude::*;
use std::fs::File;

#[test]
fn one_sample_t_test_weight() {
    let data_file = File::open("data/Weight Gain.csv").expect("unable to open data file");

    let pn_result_file =
        File::open("pytests/one_sample_t_test_weight.csv").expect("unable to open comparison file");

    let df = CsvReadOptions::default()
        .with_has_header(true)
        .into_reader_with_file_handle(data_file)
        .finish()
        .unwrap();

    println!("{:?}", df);

    let result = one_sample_t_test(
        df["Difference"].as_materialized_series(),
        0.0,
        Alternative::TwoSided,
    );
    println!("{:?}", result);

    let pn_result = CsvReadOptions::default()
        .with_has_header(true)
        .into_reader_with_file_handle(pn_result_file)
        .finish()
        .unwrap();
    println!("{:?}", pn_result);

    for col in vec!["T", "dof", "p-val", "cohen-d"] {
        let val_result = result[col].get(0).unwrap().try_extract::<f64>().unwrap();
        let val_pn_result = pn_result[col].get(0).unwrap().try_extract::<f64>().unwrap();

        assert!(f64::abs(val_result - val_pn_result) < 100.0 * f64::EPSILON);
    }
}

#[test]
fn paired_t_test_moon() {
    let data_file = File::open("data/Moon and Aggression.csv").expect("unable to open data file");

    let pn_result_file =
        File::open("pytests/paired_t_test_moon.csv").expect("unable to open comparison file");

    let df = CsvReadOptions::default()
        .with_has_header(true)
        .into_reader_with_file_handle(data_file)
        .finish()
        .unwrap();

    println!("{:?}", df);

    let result = paired_t_test(
        df["Moon"].as_materialized_series(),
        df["Other"].as_materialized_series(),
        Alternative::TwoSided,
    );
    println!("{:?}", result);

    let pn_result = CsvReadOptions::default()
        .with_has_header(true)
        .into_reader_with_file_handle(pn_result_file)
        .finish()
        .unwrap();
    println!("{:?}", pn_result);

    for col in vec!["T", "dof", "p-val", "cohen-d"] {
        let val_result = result[col].get(0).unwrap().try_extract::<f64>().unwrap();
        let val_pn_result = pn_result[col].get(0).unwrap().try_extract::<f64>().unwrap();

        assert!(f64::abs(val_result - val_pn_result) < 100.0 * f64::EPSILON);
    }
}
