use yew::prelude::*;

#[function_component(App)]
fn app() -> Html
{
	html!
	{
		<h1 class={classes!("text-3xl", "text-blue-500")}>{ "Hello World xD xD" }</h1>
	}
}

fn main()
{
	yew::start_app::<App>();
}