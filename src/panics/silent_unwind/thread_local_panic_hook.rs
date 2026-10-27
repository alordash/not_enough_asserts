use std::panic;

type Hook = Box<dyn Fn(&panic::PanicHookInfo<'_>) + 'static + Sync + Send>;
pub fn set(hook: Hook) {
    internal::ensure_default_hook();
    internal::LOCAL_HOOK.set(Some(hook));
}

pub fn take() -> Option<Hook> {
    internal::ensure_default_hook();
    internal::LOCAL_HOOK.take()
}

mod internal {
    use super::*;
    use std::cell::RefCell;
    use std::sync::LazyLock;
    use std::thread;

    static DEFAULT_HOOK: LazyLock<Hook> = LazyLock::new(init_default_hook);

    fn init_default_hook() -> Hook {
        let default_hook = panic::take_hook();
        panic::set_hook(Box::new(global_hook));
        return default_hook;
    }

    fn global_hook(p: &panic::PanicHookInfo) {
        LOCAL_HOOK.with(|x| {
            let maybe_local_hook = x.borrow();
            maybe_local_hook.as_ref().unwrap_or(&*DEFAULT_HOOK)(p);
        });
    }

    thread_local! {
        pub static LOCAL_HOOK: RefCell<Option<Hook>> = RefCell::new(None);
    }

    #[inline]
    pub fn ensure_default_hook() {
        if thread::panicking() {
            panic!("Cannot modify panic hook from panicking thread.");
        }

        _ = *DEFAULT_HOOK;
    }
}
