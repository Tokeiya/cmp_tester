use std::panic::UnwindSafe;

fn main() {
	let mut cnt=0;
	
	foo(||{
		*(&mut cnt)+=1;
		*(&cnt)
	});
	
	println!("cnt:{}",&cnt)
	
}

	#[test]
	fn foo() {}
}
