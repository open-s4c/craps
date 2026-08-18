use dice_rs::{
    Chain, DiceResult, events::*,
    subscribe, thread::*,
};
use lazy_static::lazy_static;
use super::recorder::*;

const CRAPS_PRIORITY: i32 = match i32::from_str_radix(env!("CRAPS_PRIORITY", "CRAPS_PRIORITY must be set"), 10) {
    Ok(val) => val,
    Err(_) => panic!("Failed to parse CRAPS_PRIORITY as i32")
};

lazy_static! {
    static ref ENABLED: bool = {
        match std::env::var("RUNTIME_MODE").as_deref() {
            Ok("RECORD") => true,
            Ok("REPLAY") => false,
            _ => panic!("RUNTIME_MODE must be either RECORD or REPLAY"),
        }
    };
}

subscribe!(Chain::CaptureEvent, CRAPS_PRIORITY, *ENABLED, |_event: Option<&mut SelfInitEvent>, meta| {
    let thread_id = self_id(meta);
    with_recorder(meta, |trace| {
        trace.initialize(thread_id);
    });

    DiceResult::Ok
});

subscribe!(Chain::CaptureEvent, CRAPS_PRIORITY, *ENABLED, |_event: Option<&mut SelfFiniEvent>, meta| {
    with_recorder(meta, |trace| {
        trace.end().expect("trace finalization should succeed");
    });
    DiceResult::Ok
});

macro_rules! define_simple_handlers {
    ($($event:ty),* $(,)?) => {
          $(
              subscribe!(
                  Chain::CaptureAfter,
                  CRAPS_PRIORITY,
                  *ENABLED,
                  |event: Option<&mut $event>, meta| {
                          with_recorder(meta, |trace| {
                              trace.record_event(event.expect("event should be available"))
                          });
                          DiceResult::Ok
                  }
              );
          )*
      };
}

define_simple_handlers!(
    GetrandomEvent,
    //EpollCreate1Event,
    //EpollCtlEvent,
    EpollWaitEvent,
    PollEvent,
    //EventfdEvent,
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
    ClockGettimeEvent,
    GettimeofdayEvent,
    //OpenatEvent,
    //MmapEvent,
    RecvEvent,
    SendEvent,
    PpollEvent,
    SyscallEvent,
    TimeEvent,
);
