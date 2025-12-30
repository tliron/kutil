use super::super::message::*;

// use std::sync::*;

//
// ConcurrencyError
//

message_error!(ConcurrencyError, "concurrency");

// impl<DataT> From<PoisonError<DataT>> for ConcurrencyError {
//     fn from(error: PoisonError<DataT>) -> Self {
//         Self::from(error.to_string())
//     }
// }
