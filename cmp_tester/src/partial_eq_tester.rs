use std::cmp::PartialEq;

pub fn assert_reflexivity<T: PartialEq>(x: T) {
	assert!(x.eq(&x));
}

pub fn assert_transitive<T, U, V>(x: T, y: U, z: V)
where
	T: PartialEq<U>,
	T: PartialEq<V>,
	U: PartialEq<V>,
	U: PartialEq<T>,
	V: PartialEq<T>,
	V: PartialEq<U>,
{
	assert!(x.eq(&y));
	assert!(x.eq(&z));
	assert!(y.eq(&z));

	assert!(y.eq(&x));
	assert!(y.eq(&z));
	assert!(z.eq(&x));

	assert!(z.eq(&x));
	assert!(z.eq(&y));
	assert!(x.eq(&y));
}

pub fn assert_symmetric<T, U>(x: T, y: U)
where
	T: PartialEq<U>,
	U: PartialEq<T>,
{
	assert!(x.eq(&y));
	assert!(y.eq(&x));
}

pub fn assert_not_equal<T, U>(x: T, y: U)
where
	T: PartialEq<U>,
	U: PartialEq<T>,
{
	assert!(!x.eq(&y));
	assert!(x.ne(&y));

	assert!(y.ne(&x));
	assert!(!y.eq(&x));
}

pub fn assert_eq<T, U, V, N>(x: T, y: T, z: T, ne: T)
where
	T: PartialEq<U>,
	T: PartialEq<V>,
	T: PartialEq<N>,
	T: PartialEq<T>,
	U: PartialEq<V>,
	U: PartialEq<T>,
	U: PartialEq<N>,
	U: PartialEq<U>,
	V: PartialEq<T>,
	V: PartialEq<U>,
	V: PartialEq<N>,
	V: PartialEq<V>,
	N: PartialEq<T>,
	N: PartialEq<U>,
	N: PartialEq<V>,
	N: PartialEq<N>,
{
	assert_reflexivity(&x);
	assert_reflexivity(&y);
	assert_reflexivity(&z);
	assert_reflexivity(&ne);

	assert_symmetric(&x, &y);
	assert_symmetric(&x, &z);
	assert_symmetric(&y, &z);

	assert_transitive(&x, &y, &z);

	assert_not_equal(&x, &ne);
	assert_not_equal(&y, &ne);
	assert_not_equal(&z, &ne);
}
