use std::cmp::PartialOrd;

pub fn assert_greater<T, U>(pivot: T, greater: U)
where
	T: PartialOrd<U>,
	U: PartialOrd<T>,
{
	assert!(pivot < greater);
	assert!(greater > pivot);
}

pub fn assert_less<T, U>(pivot: T, less: U)
where
	T: PartialOrd<U>,
	U: PartialOrd<T>,
{
	assert!(pivot > less);
	assert!(less < pivot);
}

pub fn assert_greater_or_eq<T, U, V>(pivot: T, greater: U, equivalent: V)
where
	T: PartialOrd<U>,
	T: PartialOrd<V>,
	U: PartialOrd<T>,
	V: PartialOrd<T>,
{
	assert!(pivot <= greater);
	assert!(greater >= pivot);
	assert!(pivot >= equivalent);
	assert!(equivalent <= pivot);
}

pub fn assert_less_or_eq<T, U, V>(pivot: T, less: U, equivalent: V)
where
	T: PartialOrd<U>,
	T: PartialOrd<V>,
	U: PartialOrd<T>,
	V: PartialOrd<T>,
{
	assert!(pivot >= less);
	assert!(less <= pivot);
	assert!(pivot <= equivalent);
	assert!(equivalent >= pivot);
}

pub fn assert_ord<T, U, V>(pivot: T, greater: U, less: U, equivalent: V)
where
	T: PartialOrd<U>,
	T: PartialOrd<V>,
	U: PartialOrd<T>,
	V: PartialOrd<T>,
{
	assert_greater(&pivot, &greater);
	assert_less(&pivot, &less);
	assert_greater_or_eq(&pivot, &equivalent, &equivalent);
	assert_less_or_eq(&pivot, &equivalent, &equivalent);
}
