mod record;
mod recorder;
mod replayer;

use dice_rs::subscribe as dice_subscribe;
use lazy_static::lazy_static;

macro_rules! env_i32_consts {
    ($($name:ident),* $(,)?) => {
        $(
            pub(crate) const $name: i32 = match i32::from_str_radix(
                env!(stringify!($name), concat!(stringify!($name), " must be set")),
                10,
            ) {
                Ok(val) => val,
                Err(_) => panic!(concat!(
                    "Failed to parse ",
                    stringify!($name),
                    " as i32"
                )),
            };
        )*
    };
}

env_i32_consts!(
    CRAPS_PRIORITY,
    CRAPS_INIT_PRIORITY,
);

#[derive(PartialEq)]
enum RuntimeMode {
    Record,
    Replay,
}

lazy_static! {
    pub(crate) static ref RUNTIME_MODE: RuntimeMode = {
        match std::env::var("RUNTIME_MODE").as_deref() {
            Ok("RECORD") => RuntimeMode::Record,
            Ok("REPLAY") => RuntimeMode::Replay,
            _ => panic!("RUNTIME_MODE must be either RECORD or REPLAY"),
        }
    };
}

#[macro_export]
macro_rules! subscribe {
    ($chain:expr, $cond:expr, |$e:ident: Option<&mut $t:ty>, $m:ident| $body:block) => {
        $crate::dice_subscribe!($chain, $crate::CRAPS_PRIORITY, $cond, |$e: Option<&mut $t>, $m| $body);
    };
}

#[macro_export]
macro_rules! subscribe_init {
    ($chain:expr, $cond:expr, |$e:ident: Option<&mut $t:ty>, $m:ident| $body:block) => {
        $crate::dice_subscribe!($chain, $crate::CRAPS_INIT_PRIORITY, $cond, |$e: Option<&mut $t>, $m| $body);
    };
}

pub fn use_craps() {
}
