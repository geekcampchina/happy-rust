use happy_rust::{hldebug, hlenter_fn, hlerror, hlexit_fn, hlinfo, hlinput, hloutput, hltrace, hlvar, hlwarn};

#[test]
fn test_all_log() {
    // 警告：在初始化logtest之前，不能初始化log4rs（log4rs::init_config等等）
    let mut logger = logtest::Logger::start();

    let foo = "1";
    let fn_name = "test_all_log";

    hlenter_fn!(fn_name);
    assert_eq!(logger.pop().unwrap().args(), "Enter function: test_all_log");

    hlinput!("foo", foo);
    assert_eq!(logger.pop().unwrap().args(), "input->foo=1");

    hlerror!("error");
    assert_eq!(logger.pop().unwrap().args(), "error");

    hlwarn!("warn");
    assert_eq!(logger.pop().unwrap().args(), "warn");

    hlinfo!("info");
    assert_eq!(logger.pop().unwrap().args(), "info");

    hldebug!("debug");
    assert_eq!(logger.pop().unwrap().args(), "debug");

    hltrace!("trace");
    assert_eq!(logger.pop().unwrap().args(), "trace");

    hlvar!("foo", foo);
    assert_eq!(logger.pop().unwrap().args(), "var->foo=1");

    hloutput!("foo", foo);
    assert_eq!(logger.pop().unwrap().args(), "output->foo=1");

    hlexit_fn!(fn_name);
    assert_eq!(logger.pop().unwrap().args(), "Exit function: test_all_log");
}