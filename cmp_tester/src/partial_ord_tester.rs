use std::cmp::PartialOrd;

pub fn assert_greater<T, U>(pivot: T, greater: U)
where
	T: PartialOrd<U>,
{
	todo!()
}

pub fn assert_less<T, U>(pivot: T, less: U)
where
	T: PartialOrd<U>,
{
	todo!()
}

pub fn assert_greater_or_eq<T, U>(pivot: T, greater: U, equivalent: U)
where
	T: PartialOrd<U>,
{
	todo!()
}

pub fn assert_less_or_eq<T, U>(pivot: T, less: U, equivalent: U)
where
	T: PartialOrd<U>,
{
	todo!()
}

pub fn assert_ord<T, U>(pivot: T, greater: U, less: U, equivalent: U)
where
	T: PartialOrd<U>,
{
	todo!()
}

pub fn test_greater<T, U>(pivot: T, greater: U) -> bool
where
	T: PartialOrd<U>,
{
	todo!()
}

pub fn test_less<T, U>(pivot: T, less: U) -> bool
where
	T: PartialOrd<U>,
{
	todo!()
}

pub fn test_greater_or_eq<T, U>(pivot: T, greater: U, equivalent: U) -> bool
where
	T: PartialOrd<U>,
{
	todo!()
}

pub fn test_less_or_eq<T, U>(pivot: T, less: U, equivalent: U) -> bool
where
	T: PartialOrd<U>,
{
	todo!()
}

pub fn test_ord<T, U>(pivot: T, greater: U, less: U, equivalent: U) -> bool
where
	T: PartialOrd<U>,
{
	todo!()
}
