// mod xr;
// mod front;

use yew::prelude::*;

#[function_component(App)]
fn app() -> Html 
{
	html! 
	{
		<h1 class={classes!("text-xl", "text-blue-500")}>{ "Hello World xD xD" }</h1>
	}
}

fn main()
{
	yew::start_app::<App>();

	// xr::run_game();
}