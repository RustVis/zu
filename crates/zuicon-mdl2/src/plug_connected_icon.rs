// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct PlugConnectedIcon {}

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

impl Component for PlugConnectedIcon {
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
                xmlns={ "http://www.w3.org/2000/svg" }
                class={ props.class.unwrap_or("") }
                width={ props.width.unwrap_or("16") }
                height={ props.height.unwrap_or("16") }
                focusable={ "false" }
                data-icon={ "PlugConnectedIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M256 1216q0-89 34-171t97-146l227-226 633 633-226 227q-63 63-145 97t-172 34q-73 0-141-22t-127-67l-327 326-90-90 326-327q-44-58-66-126t-23-142zm448 320q64 0 122-24t104-70l136-136-452-452-136 136q-45 45-69 103t-25 123q0 66 25 124t68 102 102 69 125 25zm871-1100q44 58 66 126t23 142q0 89-34 171t-97 146l-227 226-633-633 226-227q63-63 145-97t172-34q73 0 141 22t127 67l327-326 90 90-326 327zm-133 494q45-45 69-103t25-123q0-66-25-124t-69-101-102-69-124-26q-64 0-122 24t-104 70L854 614l452 452 136-136z" />
            </svg>
        }
    }
}
