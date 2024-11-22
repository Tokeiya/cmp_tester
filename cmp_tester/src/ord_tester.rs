use super::partial_ord_tester as tester;
use std::cmp::Ord;

pub fn assert_greater<T: Ord>(pivot: T, greater: T) {
	tester::assert_greater(pivot, greater);
}

pub fn assert_less<T: Ord>(pivot: T, less: T) {
	tester::assert_less(pivot, less);
}

pub fn assert_greater_or_eq<T: Ord>(pivot: T, greater: T, equivalent: T) {
	tester::assert_greater_or_eq(pivot, greater, equivalent);
}

pub fn assert_less_or_eq<T: Ord>(pivot: T, less: T, equivalent: T) {
	tester::assert_less_or_eq(pivot, less, equivalent);
}

pub fn assert_ord<T: Ord>(pivot: T, greater: T, less: T, equivalent: T) {
	tester::assert_ord(pivot, greater, less, equivalent);
}
