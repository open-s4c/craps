use std::mem;

use dice_rs::{
    DiceThreadId,
    Metadata,
    tls_key,
    thread::TlsKey,
};

use crate::record::{
    RecordEnum,
    ToRecordEnum,
};

use felt::writer::{
    self,
    Writer,
    WriterError,
};

type RecordWriter =
Box<dyn Writer<RecordEnum> + Sync>;

pub(crate) struct Recorder {
    thread_id: DiceThreadId,
    writer: RecordWriter,
}

tls_key!(RECORDER: Option<Recorder>);

impl Recorder {
    pub(crate) fn new(
        thread_id: DiceThreadId,
    ) -> Self {
        let path = format!(
            "records/craps_{thread_id}.txt"
        );

        let writer =
            writer::from_path::<RecordEnum>(&path)
                .expect(
                    "trace file creation should succeed"
                );

        Self {
            thread_id,
            writer: Box::new(writer),
        }
    }

    pub fn end(
        mut self,
    ) -> Result<(), WriterError> {
        self.writer.flush()?;

        // Deliberately prevent the writer from being dropped.
        //
        // The underlying File therefore remains alive and
        // its file descriptor remains open.
        let writer = self.writer;
        mem::forget(writer);

        Ok(())
    }

    pub fn record_event<T>(
        &mut self,
        event: &T,
    ) -> Result<(), WriterError>
    where
        T: ToRecordEnum,
    {
        let Some(record) = event.to_record() else {
            return Ok(());
        };

        self.writer.write(&record)?;
        self.writer.flush()
    }
}

pub(crate) fn with_recorder<R>(
    meta: &mut Metadata,
    f: impl FnOnce(&mut Option<Recorder>) -> R,
) -> R {
    RECORDER.with(meta, f)
}
