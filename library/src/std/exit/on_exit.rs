use std::{process::*, sync::*};

/// Call a hook on exit.
///
/// Hooks will be called in the reverse order in which they are added.
pub fn on_exit<HookT>(hook: HookT)
where
    HookT: FnMut() + Send + 'static,
{
    let hook = Box::new(hook);

    let mut hooks = EXIT_HOOKS.lock().expect("EXIT_HOOKS");
    match hooks.as_mut() {
        Some(hooks) => {
            hooks.push(hook);
        }

        None => {
            let mut new_hooks = Vec::<ExitHook>::default();
            new_hooks.push(hook);
            *hooks = Some(new_hooks);
            set_ctrlc_handler();
        }
    }
}

//
// ExitHook
//

type ExitHook = Box<dyn FnMut() + Send>;

type Static<StaticT> = LazyLock<Mutex<Option<StaticT>>>;

static EXIT_HOOKS: Static<Vec<ExitHook>> = LazyLock::new(|| Default::default());

// ctrlc

// Can only be called once!
fn set_ctrlc_handler() {
    ctrlc::set_handler(ctrlc_handler).expect("ctrlc::set_handler");
}

fn ctrlc_handler() {
    if let Some(hooks) = EXIT_HOOKS.lock().expect("EXIT_HOOKS").take() {
        for mut hook in hooks.into_iter().rev() {
            hook();
        }
    }

    exit(130);
}
