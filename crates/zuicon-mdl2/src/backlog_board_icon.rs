// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct BacklogBoardIcon {}

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

impl Component for BacklogBoardIcon {
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
                data-icon={ "BacklogBoardIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M256 384h512v384H256V384zm128 256h256V512H384v128zM256 896h512v384H256V896zm128 256h256v-128H384v128zm512-768h512v384H896V384zm128 256h256V512h-256v128zm512-256h512v384h-512V384zm128 256h256V512h-256v128zM896 896h512v384H896V896zm128 256h256v-128h-256v128zm512-256h512v384h-512V896zm128 256h256v-128h-256v128zM256 1408h512v384H256v-384zm128 256h256v-128H384v128zm1152-256h512v384h-512v-384zm128 256h256v-128h-256v128zm384-1536v128H128v1664H0V128h2048z" />
            </svg>
        }
    }
}
