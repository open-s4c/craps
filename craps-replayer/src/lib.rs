use dice_rs::{
    Chain, DiceEvent, DiceResult, DiceThreadId, Metadata, TypeId, events::*, init_dice_state,
    subscribe, thread::*, tls_key,
};
use libc::c_void;
use serde_json::Result;
use std::{
    error::Error,
    mem,
    fs::File,
    ptr,
};
use std::io::Write;

use craps_record::{Record, RecordEnum, ToRecordEnum};

pub extern "C" fn return_zero() -> i32 {
    0
}

#[derive(Debug, Default)]
struct Replayer {
    initd: bool,
    thread_id: Option<DiceThreadId>,
    records: Vec<RecordEnum>,
}

init_dice_state!();

tls_key!(REPLAYER: Replayer);

pub trait SetFunc<T> {
    fn set_func(&mut self, func: *const c_void);
}

macro_rules! define_set_func {
    ($($event:ty),* $(,)?) => {
        $(
            impl SetFunc<$event> for $event {
                fn set_func(&mut self, func: *const c_void) {
                    self.func = Some(unsafe { mem::transmute(func) })
                }
            }
        )*
    };
}

impl Replayer {
    pub fn initialize(&mut self, thread_id: DiceThreadId) {
        if self.initd {
            return;
        }
        self.initd = true;
        self.thread_id = Some(thread_id);

        let file = File::open(format!("records/craps_{thread_id}.txt")).unwrap();
        self.records = serde_json::from_reader(file).unwrap();
    }

    pub fn end(&mut self) -> Result<()> {
        if !self.records.is_empty() {
            panic!("finish before finishing replay!")
        }
        Ok(())
    }

    pub fn dequeue_record(&mut self) -> RecordEnum {
        self.records.remove(0)
    }
}

subscribe!(Chain::CaptureEvent, 9999, |_event: Option<&mut SelfInitEvent>, meta| {
    let thread_id = self_id(meta);

    REPLAYER.with(meta, |trace| {
        trace.initialize(thread_id);
    });

    DiceResult::Ok
});

subscribe!(Chain::CaptureEvent, 9999, |_event: Option<&mut SelfFiniEvent>, meta| {
    let thread_id = self_id(meta);
    REPLAYER.with(meta, |replayer| {
        replayer.end().unwrap();
    });
    DiceResult::Ok
});

macro_rules! define_simple_before_handlers {
    ($($event:ty),* $(,)?) => {
          $(
              define_set_func!($event);
              subscribe!(
                  Chain::CaptureAfter,
                  9999,
                  |event: Option<&mut $event>, meta| {
                          REPLAYER.with(meta, |replayer| {
                              let event = event.unwrap();
                              event.set_func(return_zero as *const c_void);
                          });
                          DiceResult::Ok
                  }
              );
          )*
      };
}

define_simple_before_handlers!(
    PollEvent,
    GetrandomEvent,
);

subscribe!(Chain::CaptureAfter, 9999, |_event: Option<&mut PollEvent>, meta| {
    let thread_id = self_id(meta);
    REPLAYER.with(meta, |replayer| {
        let event = _event.unwrap();
        let record = replayer.dequeue_record();
        match record {
            RecordEnum::PollRecord(poll_record) => {
                event.ret = poll_record.ret;
                if poll_record.ret > 0 {
                    poll_record.fds.into_iter()
                        .enumerate()
                        .filter(|index_and_fd| index_and_fd.1.revents != 0)
                        .for_each(|index_and_fd| unsafe { (*event.fds.add(index_and_fd.0 as usize)).revents = index_and_fd.1.revents });
                }
            },
            _ => panic!("replay mismatch!")
        }
    });
    DiceResult::Ok
});

subscribe!(Chain::CaptureAfter, 9999, |_event: Option<&mut GetrandomEvent>, meta| {
    let thread_id = self_id(meta);
    REPLAYER.with(meta, |replayer| {
        let event = _event.unwrap();
        let record = replayer.dequeue_record();
        match record {
            RecordEnum::GetrandomRecord(getrandom_record) => {
                event.ret = getrandom_record.ret;
                // safe as long as the size of buf is the same as on record
                unsafe { ptr::copy_nonoverlapping(
                    getrandom_record.buf.as_ptr(),
                    event.buf as *mut u8,
                    getrandom_record.ret as usize
                ); }
            },
            _ => panic!("replay mismatch!")
        }
    });
    DiceResult::Ok
});
