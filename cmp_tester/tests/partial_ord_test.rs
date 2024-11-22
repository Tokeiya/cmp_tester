mod common;
extern crate cmp_tester;

use cmp_tester::partial_ord_tester as tester;
use common::panic_wrapper::assert_panic;
#[test]
fn assert_greater_test() {
	tester::assert_greater(1, 2);

	assert_panic(|| {
		tester::assert_greater(2, 1);
	});

	assert_panic(|| {
		tester::assert_greater(1, 1);
	});
}

#[test]
fn assert_less_test() {
	tester::assert_less(1, 0);

	assert_panic(|| {
		tester::assert_less(0, 1);
	});

	assert_panic(|| {
		tester::assert_less(1, 1);
	});
}

#[test]
fn assert_greater_or_eq_test() {
	tester::assert_greater_or_eq(1, 2, 1);

	assert_panic(|| {
		tester::assert_greater_or_eq(2, 1, 1);
	});

	assert_panic(|| {
		tester::assert_greater_or_eq(1, 1, 2);
	});
}

#[test]
fn assert_less_or_eq_test() {
	tester::assert_less_or_eq(1, 0, 1);

	assert_panic(|| {
		tester::assert_less_or_eq(0, 1, 1);
	});

	assert_panic(|| {
		tester::assert_less_or_eq(1, 2, 1);
	});
}

#[test]
fn assert_ord_test() {
	tester::assert_ord(1, 2, 0, 1);

	assert_panic(|| {
		tester::assert_ord(2, 1, 0, 1);
	});

	assert_panic(|| {
		tester::assert_ord(1, 0, 2, 1);
	});

	assert_panic(|| {
		tester::assert_ord(1, 2, 1, 0);
	});
}
