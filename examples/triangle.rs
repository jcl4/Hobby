use hobby::{Hobby, HobbySettings};


fn main() {
	let settings = HobbySettings::default();
	let hobby = Hobby::new(settings);

	hobby.run();
}