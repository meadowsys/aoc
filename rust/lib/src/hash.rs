use std::fmt::Write as _;

pub fn hash_md5(input: &str) -> String {
	use md5::Digest as _;

	let mut hasher = md5::Md5::new();
	hasher.update(input);

	Into::<[u8; 16]>::into(hasher.finalize())
		.into_iter()
		.fold(String::with_capacity(16 * 2), |mut string, e| {
			write!(string, "{e:02x?}").unwrap();
			string
		})
}
