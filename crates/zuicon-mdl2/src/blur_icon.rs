// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct BlurIcon {}

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

impl Component for BlurIcon {
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
                data-icon={ "BlurIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M384 128H256V0h128v128zm256 0H512V0h128v128zm256 0H768V0h128v128zm256 256h-128V256h128v128zm256 256h-128V512h128v128zm256 256h-128V768h128v128zm-512-768h-128V0h128v128zm384 256h-128V256h128v128zm0 128V384h128v128h-128zm256 256h128v128h-128V768zM640 384H512V256h128v128zm256 0H768V256h128v128zm256 256h-128V512h128v128zm256 256h-128V768h128v128zM896 640H768V512h128v128zm256 256h-128V768h128v128zm256 256h-128v-128h128v128zM256 384V256h128v128H256zm128 128V384h128v128H384zm128 128V512h128v128H512zm128 128V640h128v128H640zm128 128V768h128v128H768zm128 128V896h128v128H896zm128 128v-128h128v128h-128zm128 128v-128h128v128h-128zm128 128v-128h128v128h-128zm128 128v-128h128v128h-128zm128 128v-128h128v128h-128zm128 0h128v128h128v128H0V0h128v128h128v128H128v1536h1536v-128zm128-128h128v128h-128v-128zm0-256h128v128h-128v-128zm0-256h128v128h-128v-128zm-128 384h-128v-128h128v128zm-128-384h128v128h-128v-128zm0-896h-128V0h128v128zm256 256h128v128h-128V384zM1920 0v128h-128V0h128z" />
            </svg>
        }
    }
}
