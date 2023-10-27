#[macro_export]
macro_rules! get_yd {
	() => {{
		let file = camino::Utf8PathBuf::from(file!());
		let mut filename = file.file_name().unwrap().chars();
		assert!(filename.next() == Some('y'));
		let year = {
			let year_digits = [
				filename.next().unwrap(),
				filename.next().unwrap(),
				filename.next().unwrap(),
				filename.next().unwrap()
			];
			assert!(year_digits.iter().copied().all(char::is_numeric));
			String::from_iter(year_digits)
		};
		assert!(filename.next() == Some('-'));
		assert!(filename.next() == Some('d'));
		let day = {
			let day_digits = [
				filename.next().unwrap(),
				filename.next().unwrap()
			];
			assert!(day_digits.iter().copied().all(char::is_numeric));
			String::from_iter(day_digits)
		};
		assert!(filename.next() == Some('.'));
		assert!(filename.next() == Some('r'));
		assert!(filename.next() == Some('s'));
		assert!(filename.next().is_none());

		(year, day)
	}}
}
#[macro_export]
macro_rules! get_input {
	() => {{
		let (year, day) = aoc::get_yd!();

		let path = format!("./input/y{year}-d{day}.txt");
		match std::fs::read_to_string(&path) {
			Ok(s) => { s }
			Err(_) => {
				println!("failed to get input, check that {path} exists");
				std::process::exit(1);
			}
		}
	}}
}

pub type Map<K, V> = hashbrown::HashMap<K, V, ahash::RandomState>;
pub type Set<T> = hashbrown::HashSet<T, ahash::RandomState>;

pub fn new_map<K, V>() -> Map<K, V> {
	Map::<K, V>::with_hasher(ahash::RandomState::new())
}
pub fn new_set<T>() -> Set<T> {
	Set::<T>::with_hasher(ahash::RandomState::new())
}
