// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct BorderDotIcon {}

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

impl Component for BorderDotIcon {
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
                data-icon={ "BorderDotIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M0 0h128v128H0V0zm384 0h128v128H384V0zm384 0h128v128H768V0zm384 0h128v128h-128V0zm384 0h128v128h-128V0zm512 0v128h-128V0h128zM0 1920h128v128H0v-128zm384 0h128v128H384v-128zm384 0h128v128H768v-128zm384 0h128v128h-128v-128zm384 0h128v128h-128v-128zM0 1536h128v128H0v-128zm0-384h128v128H0v-128zm0-384h128v128H0V768zm0-384h128v128H0V384zm1920 1152h128v128h-128v-128zm0-384h128v128h-128v-128zm0-384h128v128h-128V768zm0-384h128v128h-128V384zm0 1536h128v128h-128v-128z" />
            </svg>
        }
    }
}
