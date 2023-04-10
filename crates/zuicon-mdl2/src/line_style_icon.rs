// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct LineStyleIcon {}

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

impl Component for LineStyleIcon {
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
                data-icon={ "LineStyleIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1920 384v128H0V384h1920zM128 896H0V768h128v128zm256 0H256V768h128v128zm128-128h128v128H512V768zm384 128H768V768h128v128zm256 0h-128V768h128v128zm128-128h128v128h-128V768zm384 128h-128V768h128v128zm128-128h128v128h-128V768zM256 1280H0v-128h256v128zm128-128h256v128H384v-128zm384 0h256v128H768v-128zm384 0h256v128h-256v-128zm384 0h256v128h-256v-128zM0 1536h384v128H0v-128zm512 0h128v128H512v-128zm256 0h384v128H768v-128zm768 0h384v128h-384v-128zm-256 0h128v128h-128v-128z" />
            </svg>
        }
    }
}
