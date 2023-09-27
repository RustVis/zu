// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Giftbox {}

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

impl Component for Giftbox {
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
                data-icon={ "Giftbox" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1792 768v1280H128V768h292q-35-59-35-128 0-53 20-99t54-82 81-55 100-20q97 0 181 45t139 127q54-81 138-126t182-46q53 0 99 20t82 55 55 81 20 100q0 34-9 66t-27 62h292zM896 896H256v1024h640V896zm0-128q0-53-20-99t-55-82-81-55-100-20q-27 0-50 10t-40 27-28 41-10 50q0 27 10 50t27 40 41 28 50 10h256zm384-256q-53 0-99 20t-82 55-55 81-20 100h256q27 0 50-10t40-27 28-41 10-50q0-27-10-50t-27-40-41-28-50-10zm384 384h-640v1024h640V896z" />
            </svg>
        }
    }
}
