// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct PreviewIcon {}

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

impl Component for PreviewIcon {
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
                data-icon={ "PreviewIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1115 0l549 549v91h-640V0h91zm37 512h293l-293-293v293zm384 384V768h128v128h-128zm0 256v-128h128v128h-128zm0 256v-128h128v128h-128zm0 256v-128h128v128h-128zm0 256v-128h128v128h-128zm-256 0v-128h128v128h-128zm-256 0v-128h128v128h-128zm-256 0v-128h128v128H768zm-256 0v-128h128v128H512zm-256 0v-128h128v128H256zm0-1024V768h128v128H256zm0-256V512h128v128H256zm0-256V256h128v128H256zm0-256V0h128v128H256zm256 0V0h128v128H512zm256 0V0h128v128H768zM256 1152v-128h128v128H256zm0 256v-128h128v128H256zm0 256v-128h128v128H256z" />
            </svg>
        }
    }
}
