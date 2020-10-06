//! Error handling

#[derive(Debug)]
pub enum StreamOp {
    Play,
    Pause,
}

#[derive(Debug)]
pub enum BufferOp {
    Push,
    Pop,
}

#[derive(Debug)]
pub enum Error {
    NoCpalDevice,
    BuildStream(cpal::BuildStreamError),
    PlayStream(cpal::PlayStreamError),
    PauseStream(cpal::PauseStreamError),
    StreamOperation(StreamOp),
    BufferOperation(BufferOp),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::NoCpalDevice => None,
            Error::BuildStream(ref err) => Some(err),
            Error::PlayStream(ref err) => Some(err),
            Error::PauseStream(ref err) => Some(err),
            Error::StreamOperation(_) => None,
            Error::BufferOperation(_) => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::NoCpalDevice => write!(f, "No cpal device available"),
            Error::BuildStream(_) => write!(f, "Could not create build stream"),
            Error::PlayStream(_) => write!(f, "Could not play stream"),
            Error::PauseStream(_) => write!(f, "Could not pause stream"),
            Error::StreamOperation(op) => match op {
                StreamOp::Play => {
                    write!(f, "Cannot play because no stream set! Run connect first!")
                }
                StreamOp::Pause => {
                    write!(f, "Cannot pause because no stream set! Run connect first!")
                }
            },
            Error::BufferOperation(op) => match op {
                BufferOp::Push => write!(f, "Push failed! The RingBuffer is full!"),
                BufferOp::Pop => write!(f, "Pop failed! The RingBuffer is empty!"),
            },
        }
    }
}

impl From<cpal::BuildStreamError> for Error {
    fn from(err: cpal::BuildStreamError) -> Error {
        Error::BuildStream(err)
    }
}

impl From<cpal::PlayStreamError> for Error {
    fn from(err: cpal::PlayStreamError) -> Error {
        Error::PlayStream(err)
    }
}

impl From<cpal::PauseStreamError> for Error {
    fn from(err: cpal::PauseStreamError) -> Error {
        Error::PauseStream(err)
    }
}
