use gstd::String;
use gtest::{Log, Program, System};

#[test]
fn hello_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    let res = program.send(2, String::from("INIT MESSAGE"));
    assert!(!res.main_failed());
    let res = program.send(2, String::from("HANDLE MESSAGE"));
    let expected_log = Log::builder().dest(2).payload(String::from("Hello"));
    assert!(res.contains(&expected_log));
}
