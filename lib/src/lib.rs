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

		let test_path = format!("./input/y{year}-d{day}.debug.txt");
		// is_err meaning it doesn't exist
		if camino::Utf8PathBuf::from(test_path.clone()).exists() && std::env::var("NO_DEBUG_DATA").is_err() {
			match std::fs::read_to_string(&test_path) {
				Ok(s) => {
					eprintln!("WARNING: {test_path} exists, reading input from there");
					s
				}
				Err(_) => {
					println!("failed to read debug input at {test_path}, it seemed to exist but couldn't read?");
					std::process::exit(1);
				}
			}
		} else {
			let path = format!("./input/y{year}-d{day}.txt");
			match std::fs::read_to_string(&path) {
				Ok(s) => { s }
				Err(_) => {
					println!("failed to get input, neither {path} nor {test_path} seem to exist");
					std::process::exit(1);
				}
			}
		}
	}}
}

#[macro_export]
macro_rules! allow_fun {
	() => {
		std::env::var("NO_FUN").is_err()
	}
}

#[macro_export]
macro_rules! map {
	() => {
		aoc::Map::with_hasher(ahash::RandomState::new())
	};
	($k:ty, $v:ty) => {
		aoc::Map::<$k, $v>::with_hasher(ahash::RandomState::new())
	}
}

#[macro_export]
macro_rules! set {
	() => {
		aoc::Set::with_hasher(ahash::RandomState::new())
	};
	($t:ty) => {
		aoc::Set::<$t>::with_hasher(ahash::RandomState::new())
	}
}

pub type Map<K, V> = hashbrown::HashMap<K, V, ahash::RandomState>;
pub type Set<T> = hashbrown::HashSet<T, ahash::RandomState>;
