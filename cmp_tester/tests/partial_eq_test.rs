mod common;

extern crate cmp_tester;

use cmp_tester::partial_eq_tester as tester;
use common::panic_wrapper::assert_panic;
use mockall::mock;

mock! {
	Sample{}

	impl PartialEq for Sample {
		fn eq(&self, other: &Self) -> bool;
	}
}

#[test]
fn not_equal() {
	let mut mock_x = MockSample::new();
	mock_x.expect_eq().times(2).returning(|_| false);

	let mut mock_y = MockSample::new();
	mock_y.expect_eq().times(2).returning(|_| false);

	tester::assert_not_equal(mock_x, mock_y);
}

#[test]
fn reflexivity_test() {
	let mut mock = MockSample::new();
	mock.expect_eq().times(1).returning(|_| true);

	tester::assert_reflexivity(mock);

	assert_panic(|| {
		let mut mock = MockSample::new();
		mock.expect_eq().times(1).returning(|_| false);
		tester::assert_reflexivity(mock);
	});
}

#[test]
fn transitive_test() {
	let mut mock_x = MockSample::new();
	mock_x.expect_eq().returning(|_| true);

	let mut mock_y = MockSample::new();
	mock_y.expect_eq().returning(|_| true);

	let mut mock_z = MockSample::new();
	mock_z.expect_eq().returning(|_| true);

	tester::assert_transitive(mock_x, mock_y, mock_z);

	assert_panic(|| {
		let mut mock_x = MockSample::new();
		mock_x.expect_eq().returning(|_| false);

		let mut mock_y = MockSample::new();
		mock_y.expect_eq().returning(|_| true);

		let mut mock_z = MockSample::new();
		mock_z.expect_eq().returning(|_| true);

		tester::assert_transitive(mock_x, mock_y, mock_z);
	});

	assert_panic(|| {
		let mut mock_x = MockSample::new();
		mock_x.expect_eq().returning(|_| true);

		let mut mock_y = MockSample::new();
		mock_y.expect_eq().returning(|_| false);

		let mut mock_z = MockSample::new();
		mock_z.expect_eq().returning(|_| true);

		tester::assert_transitive(mock_x, mock_y, mock_z);
	});

	assert_panic(|| {
		let mut mock_x = MockSample::new();
		mock_x.expect_eq().returning(|_| true);

		let mut mock_y = MockSample::new();
		mock_y.expect_eq().returning(|_| true);

		let mut mock_z = MockSample::new();
		mock_z.expect_eq().returning(|_| false);

		tester::assert_transitive(mock_x, mock_y, mock_z);
	});
}

#[test]
fn symmetric_test() {
	let mut mock_x = MockSample::new();
	mock_x.expect_eq().times(1).returning(|_| true);

	let mut mock_y = MockSample::new();
	mock_y.expect_eq().times(1).returning(|_| true);

	tester::assert_symmetric(mock_x, mock_y);
}

#[test]
fn symmetric_fail_test() {
	assert_panic(|| {
		let mut mock_x = MockSample::new();
		mock_x.expect_eq().times(1).returning(|_| false);

		let mock_y = MockSample::new();

		tester::assert_symmetric(mock_x, mock_y);
	});

	assert_panic(move || {
		let mut mock_x = MockSample::new();
		mock_x.expect_eq().times(1).returning(|_| true);

		let mut mock_y = MockSample::new();
		mock_y.expect_eq().times(1).returning(|_| false);

		tester::assert_symmetric(mock_x, mock_y);
	});
}

#[test]
fn assert_eq_test() {
	tester::assert_eq(1, 1, 1, 2);

	assert_panic(|| {
		tester::assert_eq(1, 2, 3, 4);
	});

	assert_panic(|| {
		tester::assert_eq(1, 1, 1, 1);
	});

	assert_panic(|| {
		tester::assert_eq(1, 1, 2, 4);
	});
}
