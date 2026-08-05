use dice_rs::{
    Chain, DiceResult, DiceThreadId, events::*,
    subscribe, thread::*, tls_key,
};
use lazy_static::lazy_static;
use serde_json::Result;
use std::{
    fs::File,
};
use std::collections::LinkedList;
use std::error::Error;
use std::io::BufWriter;
use std::io::Write;
use std::sync::{LazyLock, Mutex};
use craps_record::{OpenatRecord, RecordEnum, ToRecordEnum};

#[derive(Debug, Default)]
struct Trace {
    initd: bool,
    thread_id: Option<DiceThreadId>,
    records: LinkedList<RecordEnum>,
    writer: Option<BufWriter<File>>,
}

tls_key!(TRACE: Trace);

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

const TRACE_THRESHOLD: usize = 1;

static RETIRED_FILES: LazyLock<Mutex<Vec<File>>> = LazyLock::new(|| Mutex::new(Vec::new()));

impl Trace {
    pub fn initialize(&mut self, thread_id: DiceThreadId) {
        if self.initd || !*ENABLED {
            return;
        }
        self.initd = true;
        self.thread_id = Some(thread_id);

        self.writer = Some(BufWriter::new(File::create(format!("records/craps_{thread_id}.txt")).expect("file creation should succeed")));
    }

    pub fn end(&mut self) -> std::result::Result<(), Box<dyn Error>> {
        self.dump_events()?;
        self.writer
            .as_mut()
            .map(|file| file.flush().expect("should be able to flush file"));
        RETIRED_FILES
            .lock()
            .expect("mutex must not be poisoned")
            .push(self.writer.take().expect("trace writer should be initialized by init and end is only called once").into_inner().expect("file must be opened before end"));
        self.writer = None;
        Ok(())
    }

    pub fn record_event<T: ToRecordEnum>(&mut self, event: &T) {
        if !self.initd {
            return;
        }
        // assert!(self.initd);
        // safe as long as all implementations of to_record are safe
        if let Some(record) = unsafe { event.to_record() } {
            self.records.push_back(record);

            if self.records.len() >= TRACE_THRESHOLD {
                self.dump_events()
                    .expect("event dump should succeed");
            }
        }
    }

    fn dump_events(&mut self) -> std::result::Result<(), Box<dyn Error>> {
        if !self.initd {
            return Ok(());
        }
        if let Some(tid) = self.thread_id {
            match &mut self.writer {
                None => {
                }
                Some(writer) => {
                    while let Some(record) = self.records.pop_front() {
                        serde_json::to_writer(&mut *writer, &record)?;
                    }
                    writer.flush()?;
                }
            }
        }
        Ok(())
    }
}

subscribe!(Chain::CaptureEvent, CRAPS_PRIORITY, |_event: Option<&mut SelfInitEvent>, meta| {
    if !*ENABLED {
        return DiceResult::Ok
    }
    let thread_id = self_id(meta);
    TRACE.with(meta, |trace| {
        trace.initialize(thread_id);
    });

    DiceResult::Ok
});

subscribe!(Chain::CaptureEvent, CRAPS_PRIORITY, |_event: Option<&mut SelfFiniEvent>, meta| {
    if !*ENABLED {
        return DiceResult::Ok
    }
    TRACE.with(meta, |trace| {
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
                  |event: Option<&mut $event>, meta| {
                          if !*ENABLED {
                              return DiceResult::Ok
                          }
                          TRACE.with(meta, |trace| {
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

pub fn use_craps_recorder() {
}
