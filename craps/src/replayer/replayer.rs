use std::thread;

use dice_rs::{
    DiceThreadId,
    Metadata,
    tls_key,
    thread::TlsKey
};

use crate::{
    record::ReplayRecord,
};

use felt::reader::{
    Reader,
    ReaderError,
    self,
};
use crate::record::RecordEnum;

type RecordReader = Box<dyn Reader<ReplayRecord>>;

#[derive(Default)]
pub(crate) struct Replayer {
    initd: bool,
    thread_id: Option<DiceThreadId>,
    records: Option<RecordReader>,
}

tls_key!(REPLAYER: Replayer);

impl Replayer {
    pub fn initialize(&mut self, thread_id: DiceThreadId) {
        if self.initd {
            return;
        }

        let path = format!("records/craps_{thread_id}.txt");

        let reader = reader::from_path::<RecordEnum>(&path)
            .expect("should be able to open the trace");

        self.records = Some(Box::new(reader));
        self.thread_id = Some(thread_id);
        self.initd = true;
    }

    pub fn end(&mut self) -> Result<(), ReaderError> {
        let records = self
            .records
            .as_mut()
            .expect("records iterator should be available");

        if records.peek()?.is_some() {
            panic!("finish before finishing replay!");
        }

        // Keep the reader alive so its underlying File is not dropped,
        // and therefore its file descriptor remains open.
        let records = self
            .records
            .take()
            .expect("records iterator should be available");

        // Avoid dropping reader
        std::mem::forget(records);

        Ok(())
    }

    pub fn dequeue_record(&mut self) -> &ReplayRecord {
        let records = self
            .records
            .as_mut()
            .expect("records iterator should be available");

        match records.next() {
            Ok(Some(record)) => record,

            Err(err) => {
                panic!("record should be deserialized: {err}");
            }

            Ok(None) => {
                println!(
                    "craps replay: {:?} ran out of records",
                    self.thread_id
                );

                loop {
                    thread::park();
                }
            }
        }
    }
}

pub(crate) fn with_replayer<R>(
    meta: &mut Metadata,
    f: impl FnOnce(&mut Replayer) -> R,
) -> R {
    REPLAYER.with(meta, f)
}
