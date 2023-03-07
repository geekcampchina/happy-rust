use std::fs;
use std::sync::Once;
use log4rs::append::console::ConsoleAppender;
use log4rs::Config;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log::{error, info, LevelFilter};

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
}

#[macro_export]
macro_rules! hlenter_fn {
    ($fn_name:tt) => {
        log::trace!("Enter function: {}", $fn_name);
    };
}

#[macro_export]
macro_rules! hlinput {
    ($name:tt, $value:tt) => {
        log::trace!("input->{}={}", $name, $value);
    };

    ($name:tt, $($arg:tt)+) => {
        log::trace!("input->{}={}", $name, format!("{}", format_args!($($arg)+)).as_str());
    };
}

#[macro_export]
macro_rules! hlvar {
    ($name:tt, $value:tt) => {
        log::trace!("var->{}={}", $name, $value);
    };

    ($name:tt, $($arg:tt)+) => {
        log::trace!("var->{}={}", $name, format!("{}", format_args!($($arg)+)).as_str());
    };
}

#[macro_export]
macro_rules! hlerror {
    ($($arg:tt)+) => {
        log::trace!("{}", format_args!($($arg)+));
    };
}

#[macro_export]
macro_rules! hlwarn {
    ($($arg:tt)+) => {
        log::warn!("{}", format_args!($($arg)+));
    };
}

#[macro_export]
macro_rules! hlinfo {
    ($($arg:tt)+) => {
        log::info!("{}", format_args!($($arg)+));
    };
}

#[macro_export]
macro_rules! hldebug {
    ($($arg:tt)+) => {
        log::debug!("{}", format_args!($($arg)+));
    };
}

#[macro_export]
macro_rules! hltrace {
    ($($arg:tt)+) => {
        log::trace!("{}", format_args!($($arg)+));
    };
}

#[macro_export]
macro_rules! hloutput {
    ($name:tt, $value:tt) => {
        log::trace!("output->{}={}", $name, $value);
    };

    ($name:tt, $($arg:tt)+) => {
        log::trace!("output->{}={}", $name, format!("{}", format_args!($($arg)+)).as_str());
    };
}

#[macro_export]
macro_rules! hlexit_fn {
    ($fn_name:tt) => {
        log::trace!("Exit function: {}", $fn_name);
    };
}