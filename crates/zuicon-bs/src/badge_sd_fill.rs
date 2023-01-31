// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct BadgeSdFill {}

#[derive(Properties, Debug, Clone, PartialEq, Eq)]
pub struct Props {
    #[prop_or_default]
    pub class: Option<&'static str>,

    #[prop_or_default]
    pub width: Option<&'static str>,

    #[prop_or_default]
    pub height: Option<&'static str>,

    #[prop_or_default]
    pub color: Option<&'static str>,

    #[prop_or_default]
    pub fill: Option<&'static str>,

    #[prop_or_default]
    pub spin: bool,

    #[prop_or_default]
    pub rotate: i16,
}

impl Component for BadgeSdFill {
    type Properties = Props;
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        // TODO(Shaohua): Do not generate style attribute if it is empty.
        let mut style = String::new();
        if props.rotate != 0 {
            style += &format!("transform: rotate({}deg);", props.rotate);
        }
        html! {
            <svg
                class={ props.class.unwrap_or("") }
                width={ props.width.unwrap_or("1em") }
                height={ props.height.unwrap_or("1em") }
                focusable={ "false" }
                data-icon={ "badge-sd-fill" }
                viewBox={ "0 0 1024 1024" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M10.338 5.968h-.844v4.06h.844c1.116 0 1.622-.667 1.622-2.02 0-1.354-.51-2.04-1.622-2.04Z"/>
  <path d="M0 4a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2H2a2 2 0 0 1-2-2V4Zm5.077 7.114c1.521 0 2.378-.764 2.378-1.88 0-1.007-.642-1.473-1.613-1.692l-.932-.216c-.527-.114-.821-.351-.821-.712 0-.466.39-.804 1.046-.804.637 0 1.028.33 1.103.76h1.125c-.058-.923-.849-1.692-2.22-1.692-1.322 0-2.24.717-2.24 1.815 0 .91.588 1.446 1.52 1.657l.927.215c.624.145.923.36.923.778 0 .492-.391.83-1.13.83-.707 0-1.155-.342-1.234-.808H2.762c.052.95.79 1.75 2.315 1.75ZM8.307 11h2.19c1.81 0 2.684-1.107 2.684-3.015 0-1.894-.861-2.984-2.685-2.984H8.308V11Z"/>
            </svg>
        }
    }
}
