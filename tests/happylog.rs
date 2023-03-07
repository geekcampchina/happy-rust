use happy_rust::happylog::HappyLog;

#[test]
fn test_all_log() {
    // 警告：在初始化logtest之前，不能初始化log4rs（log4rs::init_config等等）
    let mut logger = logtest::Logger::start();

    let foo = "1";
    let fn_name = "test_all_log";

    HappyLog::enter_fn(fn_name);
    assert_eq!(logger.pop().unwrap().args(), "Enter function: test_all_log");

    HappyLog::input("foo", foo);
    assert_eq!(logger.pop().unwrap().args(), "input->foo=1");

    HappyLog::error("error");
    assert_eq!(logger.pop().unwrap().args(), "error");

    HappyLog::warn("warn");
    assert_eq!(logger.pop().unwrap().args(), "warn");

    HappyLog::info("info");
    assert_eq!(logger.pop().unwrap().args(), "info");

    HappyLog::debug("debug");
    assert_eq!(logger.pop().unwrap().args(), "debug");

    HappyLog::trace("trace");
    assert_eq!(logger.pop().unwrap().args(), "trace");

    HappyLog::var("foo", foo);
    assert_eq!(logger.pop().unwrap().args(), "var->foo=1");

    HappyLog::output("foo", foo);
    assert_eq!(logger.pop().unwrap().args(), "output->foo=1");

    HappyLog::exit_fn(fn_name);
    assert_eq!(logger.pop().unwrap().args(), "Exit function: test_all_log");
}