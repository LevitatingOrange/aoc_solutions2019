pub fn string_to_program(s: &str) -> Vec<i32> {
    // TODO: no unwrap
    s.trim().split(',').map(|n| n.trim().parse::<i32>().unwrap()).collect()
}

#[test]
fn simple_test() {
    let s = " 1,2,0, 5135, 120, 331,  3 , 14";
    assert_eq!(string_to_program(s), vec![1,2,0,5135,120,331,3,14]);
}