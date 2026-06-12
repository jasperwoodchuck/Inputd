#[macro_export]
macro_rules! unwrap_or {
	($expr:expr,continue) => {
		match $expr {
			Ok(value) => value,
			Err(err) => {
				eprintln!("{err}");
				continue;
			},
		}
	};

	($expr:expr,return) => {
		match $expr {
			Ok(value) => value,
			Err(err) => {
				eprintln!("{err}");
				return;
			},
		}
	};
}
