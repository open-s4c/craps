use dice_rs::{
    Chain, DiceEvent, DiceResult, DiceThreadId, Metadata, TypeId, events::*, init_dice_state,
    subscribe, thread::*, tls_key,
};
use serde_json::Result;
use std::{
    error::Error,
    fs::File,
};
use std::io::Write;

use craps_record::{Record, RecordEnum, ToRecordEnum};

#[derive(Debug, Default)]
struct Trace {
    initd: bool,
    thread_id: Option<DiceThreadId>,
    records: Vec<RecordEnum>,
    file: Option<File>,
}

init_dice_state!();

tls_key!(TRACE: Trace);

const TRACE_THRESHOLD: usize = 0;

impl Trace {
    pub fn initialize(&mut self, thread_id: DiceThreadId) {
        if self.initd {
            return;
        }
        self.initd = true;
        self.thread_id = Some(thread_id);

        self.file = Some(File::create(format!("records/craps_{thread_id}.txt")).unwrap());
    }

    pub fn end(&mut self) -> Result<()> {
        self.dump_events()?;
        self.file = None;
        Ok(())
    }

    pub fn record_event<T: ToRecordEnum>(&mut self, event: &T) {
        if !self.initd {
            return;
        }
        // assert!(self.initd);
        // safe as long as all implementations of to_record are safe
        let record = unsafe { event.to_record() };
        self.records.push(record);
        //println!("[{}] {} {}", self.thread_id.unwrap(), record.global_index, record.event);
        if self.records.len() == TRACE_THRESHOLD {
            self.dump_events().unwrap()
        }
    }

    fn dump_events(&mut self) -> Result<()> {
        if !self.initd || self.records.is_empty() {
            return Ok(());
        }
        if let Some(tid) = self.thread_id {
            match &mut self.file {
                None => {
                }
                Some(file) => {
                    serde_json::to_writer(file, &self.records)?;
                }
            }
        }
        Ok(())
    }
}

subscribe!(Chain::CaptureEvent, 9999, |_event: Option<&mut SelfInitEvent>, meta| {
    let thread_id = self_id(meta);

    TRACE.with(meta, |trace| {
        trace.initialize(thread_id);
    });

    DiceResult::Ok
});

subscribe!(Chain::CaptureEvent, 9999, |_event: Option<&mut SelfFiniEvent>, meta| {
    let thread_id = self_id(meta);
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
                  9999,
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
    PollEvent,
    GetrandomEvent,
);
