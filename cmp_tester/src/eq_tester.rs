use std::cmp::Eq;

pub fn assert_reflexivity<T: Eq>(x: T) {
	assert!(x.eq(&x));
}

pub fn assert_transitive<T: Eq>(x: T, y: T, z: T) {
	assert!(x.eq(&y));
	assert!(x.eq(&z));
	assert!(y.eq(&z));
}

pub fn assert_symmetric<T: Eq>(x: T, y: T) {
	assert!(x.eq(&y));
	assert!(y.eq(&x));
}

pub fn assert_not_equal<T: Eq>(x: T, y: T) {
	assert!(!x.eq(&y));
	assert!(x.ne(&y));

	assert!(y.ne(&x));
	assert!(!y.eq(&x));
}

pub fn assert_eq<T: Eq>(x: T, y: T, z: T, other: T) {
	assert_reflexivity(&x);
	assert_transitive(&x, &y, &z);
	assert_symmetric(&x, &y);

	assert_not_equal(&x, &other);
}
