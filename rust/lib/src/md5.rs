use crossbeam_channel::{ bounded, Sender };
use std::io::Write as _;
use std::process::{ ChildStdin, Command, Stdio };
use std::thread::spawn;

pub struct Md5 {
	sender: Sender<Option<(ChildStdin, String)>>
}

impl Md5 {
	pub fn new() -> Self {
		let (sender, receiver) = bounded(0);

		let md5 = Md5 { sender };

		spawn(move || {
			while let Some((mut stdin, input)) = receiver.recv().unwrap() {
				let input_bytes = input.into_bytes();
				stdin.write_all(&input_bytes).unwrap();
			}
		});

		md5
	}

	pub fn hash(&self, input: String) -> String {
		let mut cmd = Command::new("md5")
			.stdin(Stdio::piped())
			.stdout(Stdio::piped())
			.spawn()
			.unwrap();

		let stdin = cmd.stdin.take().unwrap();
		self.sender.send(Some((stdin, input))).unwrap();

		let stdout = cmd.wait_with_output().unwrap().stdout;
		let output = std::str::from_utf8(&stdout).unwrap();
		output.trim().into()
	}
}

impl Drop for Md5 {
	fn drop(&mut self) {
		self.sender.send(None).unwrap();
	}
}
