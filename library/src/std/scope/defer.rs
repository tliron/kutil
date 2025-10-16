/// Defer execution to the end of the scope.
#[macro_export]
macro_rules! defer {
    ( $($code:tt)* ) => (
        let _deferred_fn_once = $crate::std::scope::DeferredFnOnce::new(
            || -> () { $($code)* }
        );
    )
}

#[allow(unused_imports)]
pub use defer;
