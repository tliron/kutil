/// If the expression is [Err], give the error and optionally use a default expression.
#[macro_export]
macro_rules! unwrap_or_give {
    ( $result:expr, $errors:expr, $default:expr $(,)? ) => {
        match $result {
            ::std::result::Result::Ok(ok) => ok,
            ::std::result::Result::Err(error) => {
                $errors.give_error(error.into())?;
                $default
            }
        }
    };

    ( $result:expr, $errors:expr $(,)? ) => {
        if let ::std::result::Result::Err(error) = $result {
            $errors.give_error(error.into())?;
        }
    };
}

/// If the expression is [Err], give the error and return an expression.
///
/// Usage is similar to the `?` operator.
#[macro_export]
macro_rules! unwrap_or_give_and_return {
    ( $result:expr, $errors:expr, $return:expr $(,)? ) => {
        match $result {
            ::std::result::Result::Ok(ok) => ok,
            ::std::result::Result::Err(error) => {
                $errors.give_error(error.into())?;
                return $return;
            }
        }
    };
}

#[allow(unused_imports)]
pub use {unwrap_or_give, unwrap_or_give_and_return};
