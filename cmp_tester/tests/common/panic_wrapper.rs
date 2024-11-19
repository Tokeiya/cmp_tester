use std::panic::{catch_unwind, UnwindSafe};

pub fn assert_panic<F: Fn() -> R + UnwindSafe, R>(f: F) {
	if let Ok(_) = catch_unwind(f) {
		unreachable!()
	}
}
