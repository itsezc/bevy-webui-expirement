use yew::prelude::*;

mod xr;

#[function_component(App)]
fn app() -> Html
{
	html!
	{
		<h1 class={classes!("text-3xl", "text-blue-500", "bg-black")}>{ "Hello World xD xD" }</h1>
	}
}

fn main()
{
	yew::start_app::<App>();
	xr::run_game();
}