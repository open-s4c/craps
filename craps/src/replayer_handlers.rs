use dice_rs::{
    Chain, DiceResult, events::*,
    subscribe, thread::*, tls_key,
};
use libc::c_void;
use lazy_static::lazy_static;
use std::{mem, ptr};
use crate::record::{ReplayRecord, ToRecordEnum};
use crate::replayer::*;

const CRAPS_PRIORITY: i32 = match i32::from_str_radix(env!("CRAPS_PRIORITY", "CRAPS_PRIORITY must be set"), 10) {
    Ok(val) => val,
    Err(_) => panic!("Failed to parse CRAPS_PRIORITY as i32")
};

lazy_static! {
    static ref ENABLED: bool = {
        match std::env::var("RUNTIME_MODE").as_deref() {
            Ok("RECORD") => false,
            Ok("REPLAY") => true,
            _ => panic!("RUNTIME_MODE must be either RECORD or REPLAY"),
        }
    };
}

pub extern "C" fn return_zero() -> i32 {
    0
}

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

subscribe!(Chain::CaptureEvent, CRAPS_PRIORITY, |_event: Option<&mut SelfInitEvent>, meta| {
    if !*ENABLED {
        return DiceResult::Ok
    }
    let thread_id = self_id(meta);

    with_replayer(meta, |trace| {
        trace.initialize(thread_id);
    });

    DiceResult::Ok
});

subscribe!(Chain::CaptureEvent, CRAPS_PRIORITY, |_event: Option<&mut SelfFiniEvent>, meta| {
    if !*ENABLED {
        return DiceResult::Ok
    }
    let thread_id = self_id(meta);
    with_replayer(meta, |replayer| {
        replayer.end().expect("replayer finalization should succeed");
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
                          if !*ENABLED {
                              return DiceResult::Ok
                          }
                          with_replayer(meta, |replayer| {
                              let event = event.expect("event should be present");
                              event.set_func(return_zero as *const c_void);
                          });
                          DiceResult::Ok
                  }
              );
          )*
      };
}

define_simple_before_handlers!(
    //SocketEvent,
    //SocketpairEvent,
    //GetsockoptEvent,
    //SetsockoptEvent,
    //BindEvent,
    //ConnectEvent,
    //ShutdownEvent,
    //CloseEvent,
    ReadEvent,
    WriteEvent,
    ReadvEvent,
    WritevEvent,
    RecvmsgEvent,
    SendmsgEvent,
    RecvfromEvent,
    SendtoEvent,
    //EpollCreate1Event,
    //EpollCtlEvent,
    EpollWaitEvent,
    //EventfdEvent,
    PollEvent,
    GetrandomEvent,
    RecvEvent,
    SendEvent,
    PpollEvent,
    TimeEvent,
);

macro_rules! define_after_replay_handler {
    ($event_ty:ty, $record_variant:ident, |$event:ident, $record:ident| $body:block) => {
        subscribe!(Chain::CaptureAfter, CRAPS_PRIORITY, |event: Option<&mut $event_ty>, meta| {
            if !*ENABLED {
                return DiceResult::Ok
            }
            let $event = event.expect("event should be present");
            if unsafe { $event.to_record() }.is_none() {
                return DiceResult::Ok
            }
            with_replayer(meta, |replayer| {
                //println!("craps replay: {:?}:{}", replayer.thread_id, stringify!($record_variant));
                match replayer.dequeue_record() {
                    ReplayRecord::$record_variant($record) => $body,
                    other => panic!(
                            "replay mismatch: expected {}, got {:?}",
                            stringify!($record_variant),
                            other
                    ),
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
            event.errno_ = record.errno;
        });
    };
}

//define_ret_only_handler!(SocketEvent, SocketRecord);
//define_ret_only_handler!(ConnectEvent, ConnectRecord);
//define_ret_only_handler!(BindEvent, BindRecord);
//define_ret_only_handler!(ShutdownEvent, ShutdownRecord);
//define_ret_only_handler!(CloseEvent, CloseRecord);
//define_ret_only_handler!(SetsockoptEvent, SetsockoptRecord);
define_ret_only_handler!(WriteEvent, WriteRecord);
define_ret_only_handler!(WritevEvent, WritevRecord);
define_ret_only_handler!(SendtoEvent, SendtoRecord);
define_ret_only_handler!(SendmsgEvent, SendmsgRecord);
//define_ret_only_handler!(EventfdEvent, EventfdRecord);
//define_ret_only_handler!(EpollCreate1Event, EpollCreate1Record);
//define_ret_only_handler!(EpollCtlEvent, EpollCtlRecord);
//define_ret_only_handler!(OpenatEvent, OpenatRecord);

define_after_replay_handler!(PollEvent, PollRecord, |event, record| {
        event.ret = record.ret;
        if record.ret > 0 {
            record.fds.iter()
                .enumerate()
                .for_each(|index_and_fd| unsafe { (*event.fds.add(index_and_fd.0 as usize)).revents = index_and_fd.1.revents });
        }
});

define_after_replay_handler!(GetrandomEvent, GetrandomRecord, |event, record| {
    event.ret = record.ret;
    // safe as long as the size of buf is the same as on record
    unsafe { ptr::copy_nonoverlapping(
        record.buf.as_ptr(),
        event.buf as *mut u8,
        record.ret as usize
    ); }
});

define_after_replay_handler!(ReadEvent, ReadRecord, |event, record| {
    event.ret = record.ret;
    event.errno_ = record.errno;
    if record.ret > 0 {
        unsafe { ptr::copy_nonoverlapping(record.buf.as_ptr(), event.buf as *mut u8, record.ret as usize); }
    }
});

define_after_replay_handler!(ReadvEvent, ReadvRecord, |event, record| {
    event.ret = record.ret;
    event.errno_ = record.errno;
    if record.ret > 0 && !event.iov.is_null() && event.iovcnt > 0 {
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
    event.errno_ = record.errno;
    if record.ret > 0 {
        unsafe { ptr::copy_nonoverlapping(record.buf.as_ptr(), event.buf as *mut u8, record.ret as usize); }
        if !event.src_addr.is_null() && record.addrlen > 0 {
            unsafe { ptr::copy_nonoverlapping(record.src_addr.as_ptr(), event.src_addr as *mut u8, record.addrlen as usize); }
        }
    }
});

define_after_replay_handler!(RecvmsgEvent, RecvmsgRecord, |event, record| {
    event.ret = record.ret;
    event.errno_ = record.errno;

    if record.ret < 0 || event.msg.is_null() {
        return;
    }

    let Some(recorded_msg) = record.msg.as_ref() else {
        return;
    };

    unsafe {
        let msg = &mut *event.msg;

        // These are input capacities supplied by the caller.
        let name_capacity = msg.msg_namelen as usize;
        let control_capacity = msg.msg_controllen;
        let iov_count = msg.msg_iovlen;

        // Restore the peer address.
        if !msg.msg_name.is_null() && name_capacity > 0 {
            let copy_len = recorded_msg.name.len().min(name_capacity);

            ptr::copy_nonoverlapping(
                recorded_msg.name.as_ptr(),
                msg.msg_name.cast::<u8>(),
                copy_len,
            );
        }

        // Restore received payload across the caller's iovecs.
        if !msg.msg_iov.is_null() {
            let mut remaining = record.ret as usize;

            for (index, recorded_iov) in recorded_msg
                .iov
                .iter()
                .take(iov_count)
                .enumerate()
            {
                if remaining == 0 {
                    break;
                }

                let destination_iov = &mut *msg.msg_iov.add(index);

                if destination_iov.iov_base.is_null()
                    || destination_iov.iov_len == 0
                {
                    continue;
                }

                let copy_len = recorded_iov
                    .len()
                    .min(destination_iov.iov_len)
                    .min(remaining);

                ptr::copy_nonoverlapping(
                    recorded_iov.as_ptr(),
                    destination_iov.iov_base.cast::<u8>(),
                    copy_len,
                );

                remaining -= copy_len;
            }
        }

        // Restore ancillary data.
        if !msg.msg_control.is_null() && control_capacity > 0 {
            let copy_len = recorded_msg.control.len().min(control_capacity);

            ptr::copy_nonoverlapping(
                recorded_msg.control.as_ptr(),
                msg.msg_control.cast::<u8>(),
                copy_len,
            );
        }

        // Restore output metadata after using the original capacities.
        msg.msg_namelen = recorded_msg.namelen;
        msg.msg_controllen = recorded_msg.controllen;
        msg.msg_flags = recorded_msg.flags;
    }
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
/*
define_after_replay_handler!(SocketpairEvent, SocketpairRecord, |event, record| {
    event.ret = record.ret;
    event.errno_ = record.errno;
    if let Some(sv) = record.sv {
        unsafe {
            *event.sv.offset(0) = sv[0];
            *event.sv.offset(1) = sv[1];
        }
    }
});
*/
define_after_replay_handler!(GettimeofdayEvent, GettimeofdayRecord, |event, record| {
    event.ret = record.ret;

    if record.ret == 0 {
        if let Some(tv) = &record.tv {
            if !event.tv.is_null() {
                unsafe {
                    (*event.tv).tv_sec = tv.tv_sec;
                    (*event.tv).tv_usec = tv.tv_usec;
                }
            }
        }
    }
});

define_after_replay_handler!(ClockGettimeEvent, ClockGettimeRecord, |event, record| {
    event.ret = record.ret;

    if record.ret == 0 {
        if let Some(tp) = &record.tp {
            if !event.tp.is_null() {
                unsafe {
                    (*event.tp).tv_sec = tp.tv_sec;
                    (*event.tp).tv_nsec = tp.tv_nsec;
                }
            }
        }
    }
});

define_after_replay_handler!(RecvEvent, RecvRecord, |event, record| {
    event.ret = record.ret;
    event.errno_ = record.errno;

    if record.ret > 0 && !event.buf.is_null() {
        unsafe {
            ptr::copy_nonoverlapping(
                record.buf.as_ptr(),
                event.buf as *mut u8,
                record.ret as usize,
            );
        }
    }
});

define_after_replay_handler!(SendEvent, SendRecord, |event, record| {
    event.ret = record.ret;
    event.errno_ = record.errno;
});

define_after_replay_handler!(PpollEvent, PpollRecord, |event, record| {
    event.ret = record.ret;
    event.errno_ = record.errno;

    if record.ret > 0 && !event.fds.is_null() {
        record.fds
            .iter()
            .enumerate()
            .for_each(|index_and_fd| unsafe {
                (*event.fds.add(index_and_fd.0)).revents =
                    index_and_fd.1.revents;
            });
    }
});

define_after_replay_handler!(
    SyscallEvent,
    SyscallGetrandomRecord,
    |event, record| {
        event.ret = record.ret;
        event.errno_ = record.errno;

        if record.ret > 0 {
            let buf = event.args[0] as usize as *mut u8;

            assert!(
                !buf.is_null(),
                "getrandom output buffer must not be null",
            );

            assert_eq!(
                record.buf.len(),
                record.ret as usize,
                "recorded getrandom buffer length is inconsistent",
            );

            unsafe {
                ptr::copy_nonoverlapping(
                    record.buf.as_ptr(),
                    buf,
                    record.buf.len(),
                );
            }
        }
    }
);

define_after_replay_handler!(TimeEvent, TimeRecord, |event, record| {
    event.ret = record.ret;

    if record.ret != -1 && !event.tloc.is_null() {
        unsafe {
            ptr::write(event.tloc, record.ret);
        }
    }
});
