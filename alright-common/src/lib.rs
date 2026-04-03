use alright::{
    types::exception::BaseException,
    traits::{ExceptionUtils, Transform}
};
use std::error::Error;
use std::fmt::Debug;

mod modules;
pub use modules::*;

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
