use super::{context::*, depict::*};

use std::io::*;

//
// DynDepict
//

/// A reduced `dyn`-compatible version of [Depict].
pub trait DynDepict {
    /// See [Depict::depict].
    fn dyn_depict(&self, writer: Box<&mut dyn Write>, context: &DepictionContext) -> Result<()>;
}

impl<DepictT> DynDepict for DepictT
where
    DepictT: Depict,
{
    fn dyn_depict(&self, mut writer: Box<&mut dyn Write>, context: &DepictionContext) -> Result<()> {
        self.depict(writer.as_mut(), context)
    }
}
