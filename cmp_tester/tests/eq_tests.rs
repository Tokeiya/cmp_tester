extern crate cmp_tester;

use cmp_tester::eq_tester as tester;
use mockall::mock;
use std::cmp::Eq;

mock! {
	Sample{}

	impl PartialEq for Sample {
		fn eq(&self, other: &Self) -> bool;
	}

	impl Eq for Sample {}
}

#[test]
pub fn assert_reflexivity_test() {
	let mut mock = MockSample::new();
	mock.expect_eq().times(1).returning(|_| true);

	tester::assert_reflexivity(mock);
}

#[test]
#[should_panic]
fn assert_invalid_reflexivity_test() {
	let mut mock = MockSample::new();
	mock.expect_eq().times(1).returning(|_| false);

	tester::assert_reflexivity(mock);
}

#[test]
fn assert_transitive_test() {
	let mut mock_x = MockSample::new();
	mock_x.expect_eq().times(2).returning(|_| true);

	let mut mock_y = MockSample::new();
	mock_y.expect_eq().times(1).returning(|_| true);

	let mock_z = MockSample::new();

	tester::assert_transitive(mock_x, mock_y, mock_z);
}

#[should_panic]
#[test]
fn assert_transitive_fail_fail_y() {
	let mut mock_x = MockSample::new();
	mock_x.expect_eq().times(1).returning(|_| false);

	let mock_y = MockSample::new();
	let mock_z = MockSample::new();

	tester::assert_transitive(mock_x, mock_y, mock_z);
}
#[test]
#[should_panic]
fn assert_transitive_fail_fail_z() {
	let mut mock_x = MockSample::new();
	let mut cnt = 0usize;

	mock_x.expect_eq().times(2).returning(move |_| {
		cnt += 1;
		if cnt == 1 {
			true
		} else {
			false
		}
	});

	let mock_y = MockSample::new();
	let mock_z = MockSample::new();

	tester::assert_transitive(mock_x, mock_y, mock_z);
}
#[test]
#[should_panic]
fn assert_transitive_fail_fail_yz() {
	let mut mock_x = MockSample::new();
	mock_x.expect_eq().times(2).returning(move |_| true);

	let mut mock_y = MockSample::new();
	mock_y.expect_eq().times(1).returning(|_| false);

	let mock_z = MockSample::new();

	tester::assert_transitive(mock_x, mock_y, mock_z);
}

#[test]
fn assert_symmetric_test() {
	let mut mock_x = MockSample::new();
	mock_x.expect_eq().times(1).returning(|_| true);

	let mut mock_y = MockSample::new();
	mock_y.expect_eq().times(1).returning(|_| true);

	tester::assert_symmetric(mock_x, mock_y);
}

#[test]
fn assert_symmetric_fail() {
	#[should_panic]
	fn f1() {
		let mut mock_x = MockSample::new();
		mock_x.expect_eq().times(1).returning(|_| true);

		let mut mock_y = MockSample::new();
		mock_y.expect_eq().times(1).returning(|_| false);

		tester::assert_symmetric(mock_x, mock_y);
	}

	#[should_panic]
	fn f2() {
		let mut mock_x = MockSample::new();
		mock_x.expect_eq().times(1).returning(|_| false);

		let mut mock_y = MockSample::new();

		tester::assert_symmetric(mock_x, mock_y);
	}

	f1();
	f2();
}

#[test]
fn assert_not_equal_test() {
	let mut mock_x = MockSample::new();
	mock_x.expect_eq().times(1).returning(|_| false);

	let mut mock_y = MockSample::new();

	tester::assert_not_equal(mock_x, mock_y);
}
#[should_panic]
#[test]
fn assert_not_equal_fail_f1() {
	let mut mock_x = MockSample::new();
	mock_x.expect_eq().times(1).returning(|_| true);

	let mut mock_y = MockSample::new();

	tester::assert_not_equal(mock_x, mock_y);
}

#[test]
#[should_panic]
fn assert_not_equal_fail_f2() {
	let mut mock_x = MockSample::new();
	mock_x.expect_eq().times(1).returning(|_| true);

	let mut mock_y = MockSample::new();
	mock_y.expect_eq().times(1).returning(|_| true);

	tester::assert_not_equal(mock_x, mock_y);
}

#[test]
fn assert_eq_test() {
	let mut mock_x = MockSample::new();
	mock_x.expect_eq().returning(|_| true);

	let mut mock_y = MockSample::new();
	mock_y.expect_eq().returning(|_| true);

	let mut mock_z = MockSample::new();
	mock_z.expect_eq().returning(|_| true);

	let mut mock_other = MockSample::new();
	mock_other.expect_eq().returning(|_| false);

	tester::assert_eq(mock_x, mock_y, mock_z, mock_other);
}

#[test]
fn assert_eq_fail_test() {
	#[should_panic]
	fn f1() {
		let mut mock_x = MockSample::new();
		mock_x.expect_eq().returning(|_| true);

		let mut mock_y = MockSample::new();
		mock_y.expect_eq().returning(|_| true);

		let mut mock_z = MockSample::new();
		mock_z.expect_eq().returning(|_| true);

		let mut mock_other = MockSample::new();
		mock_other.expect_eq().returning(|_| true);

		tester::assert_eq(mock_x, mock_y, mock_z, mock_other);
	}

	#[should_panic]
	fn f2() {
		let mut mock_x = MockSample::new();
		mock_x.expect_eq().returning(|_| false);

		let mut mock_y = MockSample::new();
		mock_y.expect_eq().returning(|_| true);

		let mut mock_z = MockSample::new();
		mock_z.expect_eq().returning(|_| true);

		let mut mock_other = MockSample::new();
		mock_other.expect_eq().returning(|_| false);

		tester::assert_eq(mock_x, mock_y, mock_z, mock_other);
	}

	#[should_panic]
	fn f3() {
		let mut mock_x = MockSample::new();
		mock_x.expect_eq().returning(|_| true);

		let mut mock_y = MockSample::new();
		mock_y.expect_eq().returning(|_| false);

		let mut mock_z = MockSample::new();
		mock_z.expect_eq().returning(|_| true);

		let mut mock_other = MockSample::new();
		mock_other.expect_eq().returning(|_| false);

		tester::assert_eq(mock_x, mock_y, mock_z, mock_other);
	}

	#[should_panic]
	fn f4() {
		let mut mock_x = MockSample::new();
		mock_x.expect_eq().returning(|_| true);

		let mut mock_y = MockSample::new();
		mock_y.expect_eq().returning(|_| true);

		let mut mock_z = MockSample::new();
		mock_z.expect_eq().returning(|_| false);

		let mut mock_other = MockSample::new();
		mock_other.expect_eq().returning(|_| false);

		tester::assert_eq(mock_x, mock_y, mock_z, mock_other);
	}

	#[should_panic]
	fn f5() {
		let mut mock_x = MockSample::new();
		mock_x.expect_eq().returning(|_| true);

		let mut mock_y = MockSample::new();
		mock_y.expect_eq().returning(|_| true);

		let mut mock_z = MockSample::new();
		mock_z.expect_eq().returning(|_| true);

		let mut mock_other = MockSample::new();
		mock_other.expect_eq().returning(|_| true);

		tester::assert_eq(mock_x, mock_y, mock_z, mock_other);
	}

	f1();
	f2();
	f3();
	f4();
	f5();
}
