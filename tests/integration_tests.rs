

#[test]
fn trebuchet_test() {
    // Use a multiline string as input
    let input = r#"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
    "#;

    assert_eq!(aoc::trebuchet(input), 142);
}
