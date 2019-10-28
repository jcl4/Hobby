use hobby::{HobbySettings, Version, WindowSettings, AppInfo};


fn main() {
	let app_name = String::from("Triangle");
	let version = Version::new(1, 0, 0);

	let app_info = AppInfo{
		app_name,
		app_version: version,
	};

	let window_settings = WindowSettings::default();

	let hobby_settings = HobbySettings {
		window_settings,
		app_info,
	};
	
}