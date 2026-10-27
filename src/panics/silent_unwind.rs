mod thread_local_panic_hook;

use std::panic;

pub(crate) fn catch<F: FnOnce() -> R + panic::UnwindSafe, R>(f: F) -> std::thread::Result<R> {
    thread_local_panic_hook::set(Box::new(|_| {}));
    let result = panic::catch_unwind(f);
    thread_local_panic_hook::take();
    result
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use std::sync::atomic::*;
    use std::sync::*;
    use std::thread;

    #[test]
    fn catch_DoesNotUseOrReplaceOtherThreadsPanicHook() {
        // Arrange
        let hook_calls = Arc::new(AtomicUsize::new(0));
        let hook_calls_for_hook = hook_calls.clone();
        let catch_started = Barrier::new(2);
        let other_thread_panicked = Barrier::new(2);
        thread_local_panic_hook::set(Box::new(move |_| {
            hook_calls_for_hook.fetch_add(1, Ordering::SeqCst);
        }));

        // Act
        let (silent_result, other_result) = thread::scope(|scope| {
            let silent_thread = scope.spawn(|| {
                catch(|| {
                    catch_started.wait();
                    other_thread_panicked.wait();
                    panic!("silent thread panic");
                })
            });

            catch_started.wait();
            let other_result = panic::catch_unwind(|| panic!("other thread panic"));
            other_thread_panicked.wait();
            let silent_result = silent_thread.join();
            (silent_result, other_result)
        });
        thread_local_panic_hook::take();

        // Assert
        let silent_panic = silent_result
            .flatten()
            .expect_err("Silent thread must catch its panic");
        let other_panic = other_result.expect_err("Other thread must panic");
        assert_eq!(
            silent_panic.downcast_ref::<&str>(),
            Some(&"silent thread panic")
        );
        assert_eq!(
            other_panic.downcast_ref::<&str>(),
            Some(&"other thread panic")
        );
        assert_eq!(hook_calls.load(Ordering::SeqCst), 1);
    }
}
