use std::fs;
use std::sync::Once;
use log::{debug, error, info, LevelFilter, trace, warn};
use log4rs::append::console::ConsoleAppender;
use log4rs::Config;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;

static INIT: Once = Once::new();

pub struct HappyLog {
}

impl HappyLog {
    pub fn init_default_log(level: LevelFilter) {
        INIT.call_once(|| {
            HappyLog::_init_default_log(level);
        });
    }

    pub fn init(path: &str) {
        INIT.call_once(|| {
            HappyLog::_init(path);
        });
    }

    fn _init_default_log(level: LevelFilter) {
        let stdout = ConsoleAppender::builder()
            .encoder(Box::new(
                PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)} {P} [{l}] {m}{n}")
            )).build();

        let default_log_config = Config::builder()
            .appender(
                Appender::builder().build("stdout", Box::new(stdout)))
            .build(
                Root::builder().appender("stdout").build(level),
            ).unwrap();

        match log4rs::init_config(default_log_config) {
            Ok(_) => {
                info!("HappyLog->未启用日志配置文件，加载默认设置。当前运行在【控制台输出】模式下......");
            }
            Err(e) => {
                error!("HappyLog->未启用日志配置文件，加载默认设置出现错误：{}", e);
            }
        }
    }

    fn _init(path: &str) {
        if fs::metadata(path).is_ok() {
            match log4rs::init_file(path, Default::default()) {
                Ok(_) => {
                    info!("HappyLog->加载日志配置文件成功：{}", path);
                }
                Err(e) => {
                    error!("HappyLog->加载日志配置文件出现错误：{}", e);
                }
            }
        } else {
            Self::_init_default_log(LevelFilter::Info);
        }
    }

    pub fn enter_fn(fn_name: &str) {
        trace!("Enter function: {}", fn_name);
    }

    pub fn exit_fn(fn_name: &str) {
        trace!("Exit function: {}", fn_name);
    }

    pub fn error(s: &str) {
        error!("{}", s);
    }

    pub fn warn(s: &str) {
        warn!("{}", s);
    }

    pub fn info(s: &str) {
        info!("{}", s);
    }

    pub fn debug(s: &str) {
        debug!("{}", s);
    }

    pub fn trace(s: &str) {
        trace!("{}", s);
    }

    pub fn input(name: &str, value: &str) {
        trace!("input->{}={}", name, value);
    }

    pub fn output(name: &str, value: &str) {
        trace!("output->{}={}", name, value);
    }

    pub fn var(name: &str, value: &str) {
        trace!("var->{}={}", name, value);
    }
}

#[macro_export]
macro_rules! hlenter_fn {
    ($fn_name:tt) => {
        HappyLog::enter_fn($fn_name);
    };
}

#[macro_export]
macro_rules! hlinput {
    ($name:tt, $value:tt) => {
        HappyLog::input($name, $value);
    };
}

#[macro_export]
macro_rules! hlvar {
    ($name:tt, $value:tt) => {
        HappyLog::var($name, $value);
    };
}

#[macro_export]
macro_rules! hlerror {
    ($s:tt) => {
        HappyLog::error($s);
    };
}

#[macro_export]
macro_rules! hlwarn {
    ($s:tt) => {
        HappyLog::warn($s);
    };
}

#[macro_export]
macro_rules! hlinfo {
    ($s:tt) => {
        HappyLog::info($s);
    };
}

#[macro_export]
macro_rules! hldebug {
    ($s:tt) => {
        HappyLog::debug($s);
    };
}

#[macro_export]
macro_rules! hltrace {
    ($s:tt) => {
        HappyLog::trace($s);
    };
}

#[macro_export]
macro_rules! hloutput {
    ($name:tt, $value:tt) => {
        HappyLog::output($name, $value);
    };
}

#[macro_export]
macro_rules! hlexit_fn {
    ($fn_name:tt) => {
        HappyLog::exit_fn($fn_name);
    };
}