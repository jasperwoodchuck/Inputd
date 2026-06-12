use std::{
	env,
	fs,
	io,
	path::PathBuf,
};

pub fn config_path() -> Result<PathBuf, io::Error> {
	let dot_config_dir = env::var("XDG_CONFIG_HOME")
		.map(PathBuf::from)
		.or_else(|_| env::var("HOME").map(|h| PathBuf::from(h).join(".config")))
		.map_err(|err| {
			io::Error::new(
				io::ErrorKind::NotFound,
				format!("config dir not found: {err}"),
			)
		})?;

	Ok(dot_config_dir.join("inputd/config.binds"))
}

pub fn load_config() -> Result<String, io::Error> {
	fs::read_to_string(config_path()?)
}
