// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct AspectRatioIcon {}

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

impl Component for AspectRatioIcon {
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
                data-icon={ "AspectRatioIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M0 256h128v128H0V256zm0 256h128v128H0V512zm0 256h128v128H0V768zm0 256h128v128H0v-128zm0 512h128v128H0v-128zM256 256h128v128H256V256zm0 1280h128v128H256v-128zm1664 0h128v128h-128v-128zM0 1280h128v128H0v-128zm1920 0h128v128h-128v-128zm0-256h128v128h-128v-128zm0-256h128v128h-128V768zm0-256h128v128h-128V512zm128-256v128h-128V256h128zm-384 1280h128v128h-128v-128zm0-1280h128v128h-128V256zm-1152 0h1024v1408H512V256zm128 1280h768V384H640v1152z" />
            </svg>
        }
    }
}
