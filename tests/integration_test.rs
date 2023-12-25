use vectors::tests_package::tests_module::adder;


#[test]
fn it_adds_two(){
    assert_eq!(4, adder(2));
}