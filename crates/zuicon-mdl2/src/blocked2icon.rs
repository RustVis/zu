// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Blocked2icon {}

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

impl Component for Blocked2icon {
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
                data-icon={ "Blocked2Icon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M960 0q132 0 255 34t229 97 194 150 150 194 97 230 35 255q0 132-34 255t-97 229-150 194-194 150-230 97-255 35q-132 0-255-34t-229-97-194-150-150-194-97-229T0 960q0-132 34-255t97-229 150-194 194-150 229-97T960 0zm0 1792q115 0 221-29t199-84 168-130 130-168 84-199 30-222q0-115-29-221t-84-199-130-168-168-130-199-84-222-30q-115 0-221 29t-199 84-168 130-130 168-84 199-30 222q0 115 29 221t84 199 130 168 168 130 199 84 222 30zM384 896h1152v128H384V896z" />
            </svg>
        }
    }
}
