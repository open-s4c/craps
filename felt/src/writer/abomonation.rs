use std::{
    any::type_name,
    io::{
        Error,
        ErrorKind,
    },
    mem::align_of,
};

use ::abomonation::Abomonation;

use super::{
    Writer,
    WriterError,
    mapped::MappedWriter,
};

fn invalid_data(
    message: impl Into<String>,
) -> WriterError {
    Box::new(
        Error::new(
            ErrorKind::InvalidData,
            message.into(),
        )
    )
}

struct AbomonationWriter {
    inner: MappedWriter,
}

impl<T> Writer<T>
for AbomonationWriter
where
    T: Abomonation,
{
    fn write(
        &mut self,
        value: &T,
    ) -> Result<(), WriterError> {
        /*
         * Determine the exact encoded payload size first so that
         * the complete frame can be made simultaneously visible
         * before serialization starts.
         */
        let payload_len =
            ::abomonation::measure(
                value
            );

        let frame =
            self.inner
                .prepare_frame(
                    payload_len
                )?;

        /*
         * Abomonation decode later interprets the beginning of the
         * payload as T, so verify that the mapped destination has
         * the alignment required by T.
         */
        let aligned = {
            let payload =
                self.inner
                    .payload_mut(
                        frame
                    );

            (payload.as_ptr() as usize)
                % align_of::<T>()
                == 0
        };

        if !aligned {
            self.inner
                .abort_frame(
                    frame
                )?;

            return Err(
                invalid_data(format!(
                    "Abomonation payload is not aligned for {}",
                    type_name::<T>(),
                ))
            );
        }

        /*
         * Encode directly into the MAP_SHARED payload.
         *
         * &mut [u8] implements std::io::Write. As bytes are
         * written, the slice advances to its unwritten remainder.
         */
        let encode_result = {
            let payload =
                self.inner
                    .payload_mut(
                        frame
                    );

            let mut destination:
                &mut [u8] =
                payload;

            let result = unsafe {
                ::abomonation::encode(
                    value,
                    &mut destination,
                )
            };

            result.map(
                |_| destination.len()
            )
        };

        match encode_result {
            Ok(0) => {}

            Ok(remaining) => {
                self.inner
                    .abort_frame(
                        frame
                    )?;

                return Err(
                    invalid_data(format!(
                        "Abomonation measure/encode mismatch: \
                         {remaining} payload bytes remained"
                    ))
                );
            }

            Err(error) => {
                /*
                 * Best effort to restore EOF to the last committed
                 * frame before returning the serialization error.
                 */
                let _ =
                    self.inner
                        .abort_frame(
                            frame
                        );

                return Err(
                    error.into()
                );
            }
        }

        /*
         * Padding and header are finalized only after encoding has
         * succeeded. The payload length is written last.
         */
        self.inner
            .commit_frame(
                frame
            )?;

        Ok(())
    }

    fn flush(
        &mut self,
    ) -> Result<(), WriterError> {
        self.inner.flush()
    }
}

pub fn from_path<T>(
    path: &str,
) -> Result<
    impl Writer<T> + Sync + 'static,
    WriterError,
>
where
    T: Abomonation + 'static,
{
    Ok(
        AbomonationWriter {
            inner:
            MappedWriter::from_path(
                path
            )?,
        }
    )
}
