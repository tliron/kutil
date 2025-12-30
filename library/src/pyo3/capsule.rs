use {
    pyo3::{
        intern,
        {prelude::*, types::*},
    },
    std::ffi::*,
};

/// Clone capsule.
pub fn clone_capsule<CloneT>(any: &Bound<'_, PyAny>, name: &CStr) -> PyResult<CloneT>
where
    CloneT: Clone,
{
    let any = any.cast::<PyCapsule>()?;
    let any = any.pointer_checked(Some(name))?;
    let any: *mut CloneT = any.cast().as_ptr();

    // SAFETY: because we are only cloning there is no possibility of a mutation or a leak
    Ok(unsafe { (*any).clone() })
}

/// Clone capsule from attribute.
pub fn clone_capsule_from_attr<CloneT>(
    any: &Bound<'_, PyAny>,
    attr: &Bound<'_, PyString>,
    name: &CStr,
) -> PyResult<CloneT>
where
    CloneT: Clone,
{
    clone_capsule(&any.getattr(attr)?, name)
}

/// Clone capsule from "capsule" attribute.
pub fn clone_capsule_attr<CloneT>(any: &Bound<'_, PyAny>, name: &CStr) -> PyResult<CloneT>
where
    CloneT: Clone,
{
    clone_capsule_from_attr(any, intern!(any.py(), "capsule"), name)
}
