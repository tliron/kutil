use pyo3::prelude::*;

/// Register submodules.
pub fn register_submodules(module: &Bound<'_, PyModule>, module_name: &str, submodule_names: &[&str]) -> PyResult<()> {
    let modules = module.py().import("sys")?.getattr("modules")?;
    for submodule_name in submodule_names {
        let submodule = module.getattr(submodule_name)?;
        modules.set_item(format!("{}.{}", module_name, submodule_name), submodule)?;
    }
    Ok(())
}
