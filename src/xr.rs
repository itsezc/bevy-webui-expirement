use bevy::prelude::*;

pub fn run_game() -> App
{
	let app = App::new();

	#[cfg(target_arch = "wasm32")]
	app.app_plugin();

	return app;
}