// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct CafeIcon {}

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

impl Component for CafeIcon {
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
                data-icon={ "CafeIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1663 512q79 0 149 30t122 82 83 123 30 149q0 80-30 149t-82 122-123 83-149 30h-103q-44 77-105 142t-135 114h279q26 0 45 19t19 45q0 26-19 45t-45 19H191q-26 0-45-19t-19-45q0-26 19-45t45-19h279q-81-54-144-124t-108-152-67-175-24-189V512h1536zM895 1536q88 0 170-23t153-64 129-100 100-130 65-153 23-170V640H255v256q0 88 23 170t64 153 100 129 130 100 153 65 170 23zm768-384q53 0 99-20t82-55 55-81 20-100q0-53-20-99t-55-82-81-55-100-20q0 65 1 130t-1 129-12 127-32 126h44z" />
            </svg>
        }
    }
}
