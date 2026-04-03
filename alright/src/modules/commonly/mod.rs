use std::error::Error;
use std::fmt::Debug;
use crate::{
    traits::{
        ExceptionUtils,
        Transform,
        TemplateDisplay
    },
    exceptions,
    BaseException,
    Property,

};

pub trait PromiseErr: Debug + Error {}

impl<T: Transform> PromiseErr for BaseException<T> {}

pub type AlrightBox = Box<dyn PromiseErr>;

pub trait AlrightError: Error + Sized {
    type PromiseErr: PromiseErr;
    fn into_exception(self) -> BaseException<Self::PromiseErr> where <Self as AlrightError>::PromiseErr: Transform;
}

impl<T: Transform + Error + PromiseErr + ExceptionUtils<T>> AlrightError for T {
    type PromiseErr = T;
    fn into_exception(self) -> BaseException<Self::PromiseErr> {
        self.into()
    }
}

exceptions!(
    Exception,
    JustException,
    GeneratorExit,
    KeyboardInterrupt,
    SystemExit,
    ArithmeticError,
    AssertionError,
    AttributeError,
    BufferError,
    EOFError,
    MemoryError,
    NameError,
    ReferenceError,
    RuntimeError,
    StopAsyncIteration,
    StopIteration,
    SystemError,
    TypeError,
    ValueError,
    FloatingPointError,
    OverflowError,
    ZeroDivisionError,
    BlockingIOError,
    ChildProcessError,
    ConnectionError,
    FileExistsError,
    FileNotFoundError,
    InterruptedError,
    IsADirectoryError,
    NotADirectoryError,
    PermissionError,
    ProcessLookupError,
    TimeoutError,
    IndexError,
    KeyError,
    NotImplementedError,
    RecursionError,
    UnicodeError,
    BrokenPipeError,
    ConnectionAbortedError,
    ConnectionRefusedError,
    ConnectionResetError,
    UnicodeDecodeError,
    UnicodeEncodeError,
    UnicodeTranslateError,
    ExceptionGroup,
    OSError,
);

