use legion_script::*;

#[test]
fn run_python_script() {
    driver::run_script(
        String::from("/home/shammyz/Documents/repos/legion_script/tests/scripts/print.py")
    );
    assert_eq!(0,0);
}
