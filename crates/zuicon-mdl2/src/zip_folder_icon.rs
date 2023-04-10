// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct ZipFolderIcon {}

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

impl Component for ZipFolderIcon {
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
                data-icon={ "ZipFolderIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1792 0q27 0 50 10t40 27 28 41 10 50v480q0 45-9 77t-24 58-31 46-31 40-23 44-10 55v992q0 27-10 50t-27 40-41 28-50 10H256V0h1536zM640 128v384h256V128H640zm1024 800q0-31-9-54t-24-44-31-41-31-45-23-58-10-78V128h-512v512H768v128H640V640H512V128H384v1792h384v-128h128v128h768V928zm128-800h-128v480q0 24 4 42t13 33 20 29 27 32q15-17 26-31t20-30 13-33 5-42V128zM640 896h128v128H640V896zm0 256h128v128H640v-128zm0 256h128v128H640v-128zm128 256v128H640v-128h128zm0-768V768h128v128H768zm0 256v-128h128v128H768zm0 256v-128h128v128H768zm0 256v-128h128v128H768z" />
            </svg>
        }
    }
}
