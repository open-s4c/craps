use dice_rs::{
    Chain, DiceResult, DiceThreadId, events::*, init_dice_state,
    subscribe, thread::*, tls_key,
};
use serde_json::Result;
use std::{
    fs::File,
};
use std::collections::LinkedList;
use std::io::BufWriter;
use std::io::Write;
use craps_record::{RecordEnum, ToRecordEnum};

#[derive(Debug, Default)]
struct Trace {
    initd: bool,
    thread_id: Option<DiceThreadId>,
    records: LinkedList<RecordEnum>,
    writer: Option<BufWriter<File>>,
}

init_dice_state!();

tls_key!(TRACE: Trace);

const CRAPS_PRIORITY: i32 = match i32::from_str_radix(env!("CRAPS_PRIORITY", "CRAPS_PRIORITY must be set"), 10) {
    Ok(val) => val,
    Err(_) => panic!("Failed to parse CRAPS_PRIORITY as i32")
};
const TRACE_THRESHOLD: usize = 1;

impl Trace {
    pub fn initialize(&mut self, thread_id: DiceThreadId) {
        if self.initd {
            return;
        }
        self.initd = true;
        self.thread_id = Some(thread_id);

        self.writer = Some(BufWriter::new(File::create(format!("records/craps_{thread_id}.txt")).unwrap()));
        println!("initialized craps trace for thread {}", thread_id);
    }

    pub fn end(&mut self) -> Result<()> {
        self.dump_events()?;
        self.writer
            .as_mut()
            .map(|file| file.flush().expect("should be able to flush file"));
        self.writer = None;
        Ok(())
    }

    pub fn record_event<T: ToRecordEnum>(&mut self, event: &T) {
        if !self.initd {
            return;
        }
        // assert!(self.initd);
        // safe as long as all implementations of to_record are safe
        let record = unsafe { event.to_record() };
        self.records.push_back(record);
        //println!("[{}] {} {}", self.thread_id.unwrap(), record.global_index, record.event);
        if self.records.len() == TRACE_THRESHOLD {
            self.dump_events().unwrap()
        }
    }

    fn dump_events(&mut self) -> Result<()> {
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
                }
            }
        }
        Ok(())
    }
}

subscribe!(Chain::CaptureEvent, CRAPS_PRIORITY, |_event: Option<&mut SelfInitEvent>, meta| {
    let thread_id = self_id(meta);
    TRACE.with(meta, |trace| {
        trace.initialize(thread_id);
    });

    DiceResult::Ok
});

subscribe!(Chain::CaptureEvent, CRAPS_PRIORITY, |_event: Option<&mut SelfFiniEvent>, meta| {
    TRACE.with(meta, |trace| {
        trace.end().unwrap();
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
                          TRACE.with(meta, |trace| {
                              trace.record_event(event.unwrap())
                          });
                          DiceResult::Ok
                  }
              );
          )*
      };
}

define_simple_handlers!(
    GetrandomEvent,
    GetrandomEvent,
    EpollCreate1Event,
    EpollCtlEvent,
    EpollWaitEvent,
    PollEvent,
    EventfdEvent,
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
);

pub fn use_craps() {
}
