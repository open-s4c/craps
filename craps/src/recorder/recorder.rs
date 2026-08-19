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

#[derive(Default)]
pub(crate) struct Recorder {
    initd: bool,
    thread_id: Option<DiceThreadId>,
    writer: Option<RecordWriter>,
}

tls_key!(RECORDER: Recorder);

impl Recorder {
    pub fn initialize(
        &mut self,
        thread_id: DiceThreadId,
    ) {
        if self.initd {
            return;
        }

        let path = format!(
            "records/craps_{thread_id}.txt"
        );

        let writer =
            writer::from_path::<RecordEnum>(&path)
                .expect(
                    "trace file creation should succeed"
                );

        self.writer = Some(Box::new(writer));
        self.thread_id = Some(thread_id);
        self.initd = true;
    }

    pub fn end(
        &mut self,
    ) -> Result<(), WriterError> {
        let mut writer = self
            .writer
            .take()
            .expect(
                "trace writer should be initialized \
                 and end should only be called once",
            );

        writer.flush()?;

        // Deliberately prevent the writer from being dropped.
        //
        // The underlying File therefore remains alive and
        // its file descriptor remains open.
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
        if !self.initd {
            return Ok(());
        }
        if let Some(record) = event.to_record() {
            let writer = self.writer
                .as_mut()
                .expect("writer must have been initialized");
            writer.write(&record)?;
            return writer.flush()
        }
        Ok(())
    }
}

pub(crate) fn with_recorder<R>(
    meta: &mut Metadata,
    f: impl FnOnce(&mut Recorder) -> R,
) -> R {
    RECORDER.with(meta, f)
}
