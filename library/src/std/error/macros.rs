/// Like [Result::ok] but gives [Err] to a receiver.
#[macro_export]
macro_rules! ok_give {
    ( $result:expr, $errors:expr $(,)? ) => {
        match $result {
            ::std::result::Result::Ok(ok) => ::std::option::Option::Some(ok),
            ::std::result::Result::Err(error) => {
                use $crate::std::error::ErrorReceiver;
                $errors.give_error(error.into())?;
                ::std::option::Option::None
            }
        }
    };
}

/// Like [Result::unwrap_or] and [Result::unwrap_or_default] but gives [Err] to a receiver.
#[macro_export]
macro_rules! unwrap_or_give {
    ( $result:expr, $errors:expr, $default:expr $(,)? ) => {
        match $result {
            ::std::result::Result::Ok(ok) => ok,
            ::std::result::Result::Err(error) => {
                use $crate::std::error::ErrorReceiver;
                $errors.give_error(error.into())?;
                $default
            }
        }
    };

    ( $result:expr, $errors:expr $(,)? ) => {
        $crate::unwrap_or_give!($result, $errors, ::std::default::Default::default())
    };
}

/// Like [Result::unwrap] but gives [Err] to a receiver and returns [Ok].
///
/// Works somewhat similarly to the `?` operator.
#[macro_export]
macro_rules! must_unwrap_give {
    ( $result:expr, $errors:expr, $default:expr $(,)? ) => {
        match $result {
            ::std::result::Result::Ok(ok) => ok,
            ::std::result::Result::Err(error) => {
                use $crate::std::error::ErrorReceiver;
                $errors.give_error(error.into())?;
                return ::std::result::Result::Ok($default);
            }
        }
    };

    ( $result:expr, $errors:expr $(,)? ) => {
        $crate::must_unwrap_give!($result, $errors, ::std::default::Default::default())
    };
}

#[allow(unused_imports)]
pub use {must_unwrap_give, ok_give, unwrap_or_give};
