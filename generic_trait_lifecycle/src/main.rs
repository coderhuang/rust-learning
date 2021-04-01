use trait_bound_mod::Lol;

fn main() {
	println!("Hello, generic_trait_lifecycle!");
	let mut v = Vec::new();
	v.push(1);
	v.push(30);
	v.push(40);
	v.push(50);
	v.push(60);
	v.push(44);
	v.push(33);
	v.push(66);
	v.push(100);

	let largest = vec_fn::largest_num(&v);
	println!("the larger num in vec v:{}", largest);

	let v = vec![0.3, 0.5, 1.9, 100.1, 200.0];
	let largest = vec_fn::largest_num(&v);
	println!("the larger num in vec v:{}", largest);

	let p = vec_fn::Point::create_instance(0x12345, "12345=>hex".to_string());
	println!("p.x:{},p.y:{}", p.get_x(), p.get_y());
	let p = vec_fn::Point::create_instance(0o12345, "12345".to_string());
	println!("p.x:{},p.y:{}", p.get_x_i32(), p.get_y_str());
	let p = vec_fn::Point::create_instance(0.32, 0b1010_1010);
	// println!("p.x:{},p.y:{}", p.get_x_i32(), p.get_y_str());
	println!("p.x:{},p.y:{}", p.get_x(), p.get_y());

	let x: u32 = 0x123456;
	let y: u8 = 0b1111_0000;
	let pair = trait_bound_mod::Pair::new(x, y);
	pair.print_lol();
}

mod vec_fn {

	pub fn largest_num<T: PartialOrd + Copy>(v: &[T]) -> T {
		let mut largest = v[0];

		for val in v {
			if (*val) > largest {
				largest = *val;
			}
		}
		largest
	}

	pub struct Point<T, U> {
		x: T,
		y: U,
	}

	impl Point<i32, String> {
		pub fn get_x_i32(&self) -> i32 {
			self.x
		}

		pub fn get_y_str(&self) -> String {
			self.y.clone()
		}
	}

	impl<T, E> Point<T, E> {
		pub fn get_x(&self) -> &T {
			&self.x
		}

		pub fn create_instance(x: T, y: E) -> Point<T, E> {
			Point { x: x, y: y }
		}

		pub fn get_y(&self) -> &E {
			&self.y
		}
	}
}

mod trait_mod {

	pub trait Summary {
		fn summarize(&self) -> String {
			"reading".to_string()
		}
	}

	pub struct Tweet {
		username: String,
		content: String,
		reply: bool,
		retweet: bool,
	}

	impl Summary for Tweet {
		fn summarize(&self) -> String {
			format!("{} tweet {}", self.username, self.content)
		}
	}

	pub struct News {
		pub headline: String,
		pub location: String,
		pub author: String,
		pub content: String,
	}

	impl Summary for News {
		fn summarize(&self) -> String {
			format!("{}, by {} ({})", self.headline, self.author, self.location)
		}
	}
}

mod trait_bound_mod {

	use std::fmt::Display;

	pub trait Lol {
		fn print_lol(&self) {
			println!("lol");
		}
	}

	#[derive(Debug)]
	pub struct Pair<T, E> {
		x: T,
		y: E,
	}

	impl<T, E> Pair<T, E> {
		pub fn new(x: T, y: E) -> Pair<T, E> {
			Self { x, y }
		}
	}

	impl<T, E> Lol for Pair<T, E> {}
	
}
