use arslib::{sort, sort_stable};
use rand::Rng;

#[test]
fn test_empty_and_single() {
    let mut empty: Vec<i32> = vec![];
    sort(&mut empty);
    assert!(empty.is_empty());

    let mut single = vec![42];
    sort(&mut single);
    assert_eq!(single, vec![42]);
}

#[test]
fn test_small_sorting_i32() {
    let mut data = vec![5, 2, 9, 1, 5, 6];
    let mut expected = data.clone();
    expected.sort();
    sort(&mut data);
    assert_eq!(data, expected);
}

#[test]
fn test_small_sorting_stable_i32() {
    let mut data = vec![5, 2, 9, 1, 5, 6];
    let mut expected = data.clone();
    expected.sort();
    sort_stable(&mut data);
    assert_eq!(data, expected);
}

#[test]
fn test_large_sorting_i32() {
    let mut rng = rand::thread_rng();
    let mut data: Vec<i32> = (0..5000).map(|_| rng.gen_range(-10000..10000)).collect();
    let mut expected = data.clone();
    expected.sort();
    sort(&mut data);
    assert_eq!(data, expected);
}

#[test]
fn test_large_sorting_stable_i32() {
    let mut rng = rand::thread_rng();
    let mut data: Vec<i32> = (0..5000).map(|_| rng.gen_range(-10000..10000)).collect();
    let mut expected = data.clone();
    expected.sort();
    sort_stable(&mut data);
    assert_eq!(data, expected);
}

#[test]
fn test_already_sorted() {
    let mut data: Vec<i32> = (0..3000).collect();
    let expected = data.clone();
    sort(&mut data);
    assert_eq!(data, expected);
}

#[test]
fn test_reverse_sorted() {
    let mut data: Vec<i32> = (0..3000).rev().collect();
    let mut expected = data.clone();
    expected.sort();
    sort(&mut data);
    assert_eq!(data, expected);
}

#[test]
fn test_duplicates() {
    let mut data = vec![10; 3000];
    let expected = data.clone();
    sort(&mut data);
    assert_eq!(data, expected);
}

#[test]
fn test_sorting_strings() {
    let mut data = vec![
        "banana".to_string(),
        "apple".to_string(),
        "cherry".to_string(),
        "date".to_string(),
    ];
    let mut expected = data.clone();
    expected.sort();
    sort(&mut data);
    assert_eq!(data, expected);
}

#[test]
fn test_sorting_f64() {
    let mut data = vec![3.15, 1.59, 2.65, 3.58, 9.79];
    let mut expected = data.clone();
    expected.sort_by(|a, b| a.partial_cmp(b).unwrap());
    sort(&mut data);
    assert_eq!(data, expected);
}
