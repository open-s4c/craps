use std::{
    mem,
    thread,
};

use dice_rs::{
    DiceThreadId,
    Metadata,
    tls_key,
    thread::TlsKey,
};

use crate::record::{
    RecordEnum,
    ReplayRecord,
};

use felt::reader::{
    self,
    Reader,
    ReaderError,
};

type RecordReader =
Box<dyn Reader<ReplayRecord>>;

pub(crate) struct Replayer {
    thread_id: DiceThreadId,
    records: RecordReader,
}

tls_key!(REPLAYER: Option<Replayer>);

impl Replayer {
    pub(crate) fn new(
        thread_id: DiceThreadId,
    ) -> Self {
        let path =
            format!("records/craps_{thread_id}.txt");

        let reader =
            reader::from_path::<RecordEnum>(&path)
                .expect(
                    "should be able to open the trace"
                );

        Self {
            thread_id,
            records: Box::new(reader),
        }
    }

    pub fn end(
        mut self,
    ) -> Result<(), ReaderError> {
        if self.records.peek()?.is_some() {
            panic!("finish before finishing replay!");
        }

        // Keep the reader alive so its underlying File is not
        // dropped, and therefore its file descriptor remains open.
        let records = self.records;
        mem::forget(records);

        Ok(())
    }

    pub fn dequeue_record(
        &mut self,
    ) -> &ReplayRecord {
        match self.records.next() {
            Ok(Some(record)) => record,

            Err(err) => {
                panic!(
                    "record should be deserialized: {err}"
                );
            }

            Ok(None) => {
                loop {
                    thread::park();
                }
            }
        }
    }
}

pub(crate) fn with_replayer<R>(
    meta: &mut Metadata,
    f: impl FnOnce(&mut Option<Replayer>) -> R,
) -> R {
    REPLAYER.with(meta, f)
}
