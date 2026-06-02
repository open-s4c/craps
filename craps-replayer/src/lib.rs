use dice_rs::{
    Chain, DiceResult, DiceThreadId, events::*, init_dice_state,
    subscribe, thread::*, tls_key,
};
use libc::c_void;
use serde_json::Result;
use serde_json::de::IoRead;
use serde_json::Deserializer;
use serde_json::StreamDeserializer;
use std::{
    iter::Peekable,
    mem,
    fs::File,
    ptr,
};
use std::collections::LinkedList;

use craps_record::RecordEnum;

const CRAPS_PRIORITY: i32 = match i32::from_str_radix(env!("CRAPS_PRIORITY", "CRAPS_PRIORITY must be set"), 10) {
    Ok(val) => val,
    Err(_) => panic!("Failed to parse CRAPS_PRIORITY as i32")
};

pub extern "C" fn return_zero() -> i32 {
    0
}

#[derive(Default)]
struct Replayer<'a> {
    initd: bool,
    thread_id: Option<DiceThreadId>,
    records_iter: Option<Peekable<StreamDeserializer<'a, IoRead<File>, RecordEnum>>>,
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

impl Replayer<'_> {
    pub fn initialize(&mut self, thread_id: DiceThreadId) {
        if self.initd {
            return;
        }
        self.initd = true;
        self.thread_id = Some(thread_id);

        let file = File::open(format!("records/craps_{thread_id}.txt")).unwrap();
        self.records_iter = Some(Deserializer::from_reader(file).into_iter::<RecordEnum>().peekable());
    }

    pub fn end(&mut self) -> Result<()> {
        if self.records_iter.as_mut().unwrap().peek().is_some() {
            panic!("finish before finishing replay!")
        }
        Ok(())
    }

    pub fn dequeue_record(&mut self) -> RecordEnum {
        self.records_iter.as_mut().unwrap().next().unwrap().unwrap()
    }
}

subscribe!(Chain::CaptureEvent, CRAPS_PRIORITY, |_event: Option<&mut SelfInitEvent>, meta| {
    let thread_id = self_id(meta);

    REPLAYER.with(meta, |trace| {
        trace.initialize(thread_id);
    });

    DiceResult::Ok
});

subscribe!(Chain::CaptureEvent, CRAPS_PRIORITY, |_event: Option<&mut SelfFiniEvent>, meta| {
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
                  Chain::CaptureBefore,
                  CRAPS_PRIORITY,
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
    SocketEvent,
    SocketpairEvent,
    GetsockoptEvent,
    SetsockoptEvent,
    BindEvent,
    ConnectEvent,
    ShutdownEvent,
    CloseEvent,
    ReadEvent,
    WriteEvent,
    ReadvEvent,
    WritevEvent,
    RecvmsgEvent,
    SendmsgEvent,
    RecvfromEvent,
    SendtoEvent,
    EpollCreate1Event,
    EpollCtlEvent,
    EpollWaitEvent,
    EventfdEvent,
);

macro_rules! define_after_replay_handler {
    ($event_ty:ty, $record_variant:ident, |$event:ident, $record:ident| $body:block) => {
        subscribe!(Chain::CaptureAfter, CRAPS_PRIORITY, |event: Option<&mut $event_ty>, meta| {
            REPLAYER.with(meta, |replayer| {
                let $event = event.unwrap();
                match replayer.dequeue_record() {
                    RecordEnum::$record_variant($record) => $body,
                    _ => panic!("replay mismatch: expected {}", stringify!($record_variant)),
                }
            });
            DiceResult::Ok
        });
    };
}

macro_rules! define_ret_only_handler {
    ($event_ty:ty, $record_variant:ident) => {
        define_after_replay_handler!($event_ty, $record_variant, |event, record| {
            event.ret = record.ret;
        });
    };
}

define_ret_only_handler!(SocketEvent, SocketRecord);
define_ret_only_handler!(ConnectEvent, ConnectRecord);
define_ret_only_handler!(BindEvent, BindRecord);
define_ret_only_handler!(ShutdownEvent, ShutdownRecord);
define_ret_only_handler!(CloseEvent, CloseRecord);
define_ret_only_handler!(SetsockoptEvent, SetsockoptRecord);
define_ret_only_handler!(WriteEvent, WriteRecord);
define_ret_only_handler!(WritevEvent, WritevRecord);
define_ret_only_handler!(SendtoEvent, SendtoRecord);
define_ret_only_handler!(SendmsgEvent, SendmsgRecord);
define_ret_only_handler!(EventfdEvent, EventfdRecord);
define_ret_only_handler!(EpollCreate1Event, EpollCreate1Record);
define_ret_only_handler!(EpollCtlEvent, EpollCtlRecord);

subscribe!(Chain::CaptureAfter, CRAPS_PRIORITY, |_event: Option<&mut PollEvent>, meta| {
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

subscribe!(Chain::CaptureAfter, CRAPS_PRIORITY, |_event: Option<&mut GetrandomEvent>, meta| {
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

define_after_replay_handler!(ReadEvent, ReadRecord, |event, record| {
    event.ret = record.ret;
    if record.ret > 0 {
        unsafe { ptr::copy_nonoverlapping(record.buf.as_ptr(), event.buf as *mut u8, record.ret as usize); }
    }
});

define_after_replay_handler!(ReadvEvent, ReadvRecord, |event, record| {
    event.ret = record.ret;
    if record.ret > 0 && !event.iov.is_null() && event.iovcnt > 0 {
        let mut remaining = record.ret as usize;
        let iov_slice = unsafe { std::slice::from_raw_parts(event.iov, event.iovcnt as usize) };
        for (i, buf) in record.iov.iter().enumerate() {
            let iov = &iov_slice[i];

            unsafe {
                ptr::copy_nonoverlapping(
                    buf.as_ptr(),
                    iov.iov_base as *mut u8,
                    buf.len(),
                );
            }
        }
        event.ret = record.ret;
    }
});

define_after_replay_handler!(RecvfromEvent, RecvfromRecord, |event, record| {
    event.ret = record.ret;
    if record.ret > 0 {
        unsafe { ptr::copy_nonoverlapping(record.buf.as_ptr(), event.buf as *mut u8, record.ret as usize); }
        if !event.src_addr.is_null() && record.addrlen > 0 {
            unsafe { ptr::copy_nonoverlapping(record.src_addr.as_ptr(), event.src_addr as *mut u8, record.addrlen as usize); }
        }
    }
});

define_after_replay_handler!(RecvmsgEvent, RecvmsgRecord, |event, record| {
    event.ret = record.ret;
    // side-effect logic would populate msghdr fields if needed
});

define_after_replay_handler!(EpollWaitEvent, EpollWaitRecord, |event, record| {
    event.ret = record.ret;
    if record.ret > 0 && !event.events.is_null() {
        let events_slice = unsafe { std::slice::from_raw_parts_mut(event.events, record.ret as usize) };
        for (i, ev) in record.events.iter().enumerate().take(record.ret as usize) {
            events_slice[i].events = ev.events;
            events_slice[i].data.u64_ = ev.data;
        }
    }
});

define_after_replay_handler!(SocketpairEvent, SocketpairRecord, |event, record| {
    event.ret = record.ret;
    if let Some(sv) = record.sv {
        unsafe {
            *event.sv.offset(0) = sv[0];
            *event.sv.offset(1) = sv[1];
        }
    }
});

pub fn use_craps() {
}
