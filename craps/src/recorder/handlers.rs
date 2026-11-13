use dice_rs::{
    Chain, DiceResult, events::*,
    thread::*,
};

use super::recorder::*;

use crate::{
    RuntimeMode::Record,
    RUNTIME_MODE,
};

macro_rules! subscribe {
    ($chain:expr, |$e:ident: Option<&mut $t:ty>, $m:ident| $body:block) => {
        crate::subscribe!(
            $chain,
            (*RUNTIME_MODE == Record),
            |$e: Option<&mut $t>, $m| $body
        );
    };
}

macro_rules! subscribe_init {
    ($chain:expr, |$e:ident: Option<&mut $t:ty>, $m:ident| $body:block) => {
        crate::subscribe_init!(
            $chain,
            (*RUNTIME_MODE == Record),
            |$e: Option<&mut $t>, $m| $body
        );
    };
}

subscribe_init!(
    Chain::CaptureEvent,
    |_event: Option<&mut SelfInitEvent>, meta| {
        let thread_id = self_id(meta);

        with_recorder(meta, |recorder| {
            recorder.get_or_insert_with(|| {
                Recorder::new(thread_id)
            });
        });

        DiceResult::Ok
    }
);

subscribe!(
    Chain::CaptureEvent,
    |_event: Option<&mut SelfFiniEvent>, meta| {
        with_recorder(meta, |recorder| {
            recorder
                .take()
                .expect("recorder should be initialized")
                .end()
                .expect("trace finalization should succeed");
        });

        DiceResult::Ok
    }
);

macro_rules! define_simple_handlers {
    ($($event:ty),* $(,)?) => {
        $(
            subscribe!(
                Chain::CaptureAfter,
                |event: Option<&mut $event>, meta| {
                    with_recorder(meta, |recorder| {
                        if let Some(recorder) = recorder.as_mut() {
                            recorder
                                .record_event(
                                    event.expect(
                                        "event should be available"
                                    ),
                                )
                                .expect(
                                    "recording should succeed"
                                );
                        }
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
    FeofEvent,
    FerrorEvent,
    FgetcEvent,
    FgetposEvent,
    FgetsEvent,
    FputcEvent,
    FputsEvent,
    FreadEvent,
    FseekEvent,
    FsetposEvent,
    FtellEvent,
    FwriteEvent,
);
