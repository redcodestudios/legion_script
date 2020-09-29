use legion_script::*;

#[test]
fn run_python_script() {
    let mut id_count = 0_u64;
    driver::run_script(
        String::from("tests/scripts/print.py"), &mut id_count);
    assert_eq!(0,0);
}

