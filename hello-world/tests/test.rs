use gtest::{Log, Program, System};

// #[test]
// fn hello_test() {
//     let sys = System::new();
//     sys.init_logger();
//     let program = Program::current(&sys);
//     program.send(2, String::from("INIT MESSAGE"));
//     let res = program.send(2, String::from("你好啊"));
//     let expected_log = Log::builder()
//     .dest(2)
//     .payload(String::from("Hello"));
//     assert!(res.contains(&expected_log));

use hello_world::InputMessages;
// }
#[test]
fn hello_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    let res = program.send(2, String::from("Hello2"));
    assert!(!res.main_failed());
    assert!(res.log().is_empty());

    // test `SendHelloTo`
    let hello_recipient: u64 = 666;
    let res = program.send(2, InputMessages::SendHelloTo(hello_recipient.into()));
    let expected_log = Log::builder()
        .dest(hello_recipient)
        .payload(String::from("Hello2"));
    println!("请显示结果：{:?}", expected_log);
    println!("请显示res：{:?}", res);
    assert!(res.contains(&expected_log))

}
