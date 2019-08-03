extern crate lexical;

#[test]
fn vector_issue() {
    let mut results = Vec::<f32>::new();
    let line_split_vec = vec!["1", "2", "3"];
    results.extend({
          line_split_vec.iter().map(|x| {
                lexical::parse::<f32, _>(x.trim()).unwrap()
           })
     });
    assert_eq!(results, vec![1f32, 2f32, 3f32]);
}

#[test]
fn float_test() {
    let num: f64 = lexical::parse(b"5.002868148396374").unwrap();
    assert_eq!(num, 5.002868148396374);
}
