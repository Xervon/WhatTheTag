use std::process::Termination;
use std::ops::Try;

pub enum WTTResult {
    Ok,
    Err(crate::WTTError),
}

impl<T> From<crate::Result<T>> for WTTResult {
    fn from(result: crate::Result<T>) -> WTTResult {
        match result {
            crate::Result::Ok(_)  => WTTResult::Ok,
            crate::Result::Err(e) => WTTResult::Err(e),
        }
    }
}

impl Termination for WTTResult {
    fn report(self) -> i32 {
        match self {
            WTTResult::Ok     => 0,
            WTTResult::Err(e) => e.report(),
        }
    }
}

impl Try for WTTResult {
    type Ok = ();
    type Error = crate::WTTError;

    fn into_result(self) -> Result<<Self as Try>::Ok, <Self as Try>::Error> {
        match self {
            WTTResult::Ok     => Ok(()),
            WTTResult::Err(e) => Err(e),
        }
    }

    fn from_error(e: <Self as Try>::Error) -> Self {
        WTTResult::Err(e)
    }

    fn from_ok(_: <Self as Try>::Ok) -> Self {
        WTTResult::Ok
    }
}
