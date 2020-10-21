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
    FFTOperation,
    IO(std::io::Error),
    MPSCRecv(std::sync::mpsc::RecvError),
    Crossterm(crossterm::ErrorKind),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::NoCpalDevice => None,
            Error::BuildStream(err) => Some(err),
            Error::PlayStream(err) => Some(err),
            Error::PauseStream(err) => Some(err),
            Error::StreamOperation(_) => None,
            Error::BufferOperation(_) => None,
            Error::FFTOperation => None,
            Error::IO(err) => Some(err),
            Error::MPSCRecv(err) => Some(err),
            Error::Crossterm(err) => Some(err),
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
            Error::FFTOperation=> write!(f, "The input data was greater than the sampling rate, probably CPAL hiccup - ignoring"),
            Error::IO(_) => write!(f, "Could not create terminal backend!"),
            Error::MPSCRecv(_) => write!(f, "The receiver queue is empty!"),
            Error::Crossterm(_) => write!(f, "Could not create TUI"),
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

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IO(err)
    }
}

impl From<std::sync::mpsc::RecvError> for Error {
    fn from(err: std::sync::mpsc::RecvError) -> Error {
        Error::MPSCRecv(err)
    }
}

impl From<crossterm::ErrorKind> for Error {
    fn from(err: crossterm::ErrorKind) -> Error {
        Error::Crossterm(err)
    }
}
