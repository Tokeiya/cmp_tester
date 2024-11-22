use super::partial_eq_tester as tester;
use std::cmp::Eq;

pub fn assert_reflexivity<T: Eq>(x: T) {
	tester::assert_reflexivity(x);
}
pub fn assert_transitive<T: Eq>(x: T, y: T, z: T) {
	tester::assert_transitive(x, y, z);
}

pub fn assert_symmetric<T: Eq>(x: T, y: T) {
	tester::assert_symmetric(x, y);
}

pub fn assert_not_equal<T: Eq>(x: T, y: T) {
	tester::assert_not_equal(x, y);
}

pub fn assert_eq<T: Eq>(x: T, y: T, z: T, ne: T) {
	tester::assert_eq(x, y, z, ne);
}
