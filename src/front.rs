use yew::{
	Component,
	Context,
	html,
	Html,
	Properties
};

#[derive(PartialEq, Properties)]
pub struct Props;

pub struct MyApp;

impl Component for MyApp
{
	type Message = ();
	type Properties = Props;

	fn create(_ctx: &Context<Self>) -> Self 
	{
		MyApp
	}

	fn view(&self, _ctx: &Context<Self>) -> Html
	{
		html! {
			<p>{"Hello World!"}</p>
		}
	}
}