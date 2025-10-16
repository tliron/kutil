// See: https://stackoverflow.com/a/29963675

//
// DeferredFnOnce
//

/// DeferredFnOnce [FnOnce]. Will call it when dropped.
pub struct DeferredFnOnce<FunctionT>
where
    FunctionT: FnOnce(),
{
    /// Function.
    pub function: Option<FunctionT>,
}

impl<FunctionT> DeferredFnOnce<FunctionT>
where
    FunctionT: FnOnce(),
{
    /// Constructor.
    pub fn new(function: FunctionT) -> Self {
        Self { function: Some(function) }
    }
}

impl<FunctionT> Drop for DeferredFnOnce<FunctionT>
where
    FunctionT: FnOnce(),
{
    fn drop(&mut self) {
        self.function.take().expect("function")();
    }
}
