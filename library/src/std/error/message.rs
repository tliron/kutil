/// Define a message error.
#[macro_export]
macro_rules! message_error {
    ( $type:ident, $message:literal $(,)? ) => {
        /// $type.
        #[derive(Clone, Debug, Default)]
        pub struct $type(::std::option::Option<::std::string::String>);

        impl $type {
            /// Constructor.
            pub fn new_from<DisplayT>(display: DisplayT) -> Self
            where
                DisplayT: ::std::fmt::Display,
            {
                display.to_string().into()
            }
        }

        impl ::std::convert::From<::std::string::String> for $type {
            fn from(message: ::std::string::String) -> Self {
                Self(::std::option::Option::Some(message))
            }
        }

        impl ::std::convert::From<&str> for $type {
            fn from(message: &str) -> Self {
                Self(::std::option::Option::Some(message.into()))
            }
        }

        impl ::std::fmt::Display for $type {
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                match &self.0 {
                    ::std::option::Option::Some(message) => ::std::write!(formatter, "{}: {}", $message, message),
                    ::std::option::Option::None => ::std::fmt::Display::fmt($message, formatter),
                }
            }
        }

        impl ::std::error::Error for $type {}
    };
}
