use super::{context::*, depict::*};

use std::io::*;

impl<BoxedT> Depict for Box<BoxedT>
where
    BoxedT: Depict,
{
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> Result<()>
    where
        WriteT: Write,
    {
        self.as_ref().depict(writer, context)
    }
}
