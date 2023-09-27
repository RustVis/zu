// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct ZoomIn {}

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

impl Component for ZoomIn {
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
                data-icon={ "ZoomIn" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M2029 1939q19 19 19 45t-19 45-45 19q-26 0-45-19l-785-784q-96 79-210 121t-240 42q-97 0-187-25t-168-71-142-110-111-143-71-168T0 704q0-97 25-187t71-168 110-142T349 96t168-71T704 0q97 0 187 25t168 71 142 110 111 143 71 168 25 187q0 125-42 239t-121 211l784 785zM704 1280q119 0 224-45t183-124 123-183 46-224q0-119-45-224t-124-183-183-123-224-46q-119 0-224 45T297 297 174 480t-46 224q0 119 45 224t124 183 183 123 224 46zm64-640h256v128H768v256H640V768H384V640h256V384h128v256z" />
            </svg>
        }
    }
}
