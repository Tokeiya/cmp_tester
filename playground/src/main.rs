fn main() {}

#[cfg(test)]
mod tests {

	#[test]
	fn foo() {
		#[should_panic]
		fn f() {
			panic!()
		}

		f();
	}
}
