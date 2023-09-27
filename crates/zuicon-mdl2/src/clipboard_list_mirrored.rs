// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct ClipboardListMirrored {}

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

impl Component for ClipboardListMirrored {
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
                data-icon={ "ClipboardListMirrored" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M256 256v1792h1536V256h-512q0-53-20-99t-55-82-81-55-100-20q-53 0-99 20t-82 55-55 81-20 100H256zm1152 256H640V384h256V256q0-27 10-50t27-40 41-28 50-10q27 0 50 10t40 27 28 41 10 50v128h256v128zM384 384h128v256h1024V384h128v1536H384V384zm896 512H512v128h768V896zm0 384H512v128h768v-128zm0 384H512v128h768v-128zm256-768h-128v128h128V896zm0 384h-128v128h128v-128zm0 384h-128v128h128v-128z" />
            </svg>
        }
    }
}
