// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct SaveToMobile {}

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

impl Component for SaveToMobile {
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
                data-icon={ "SaveToMobile" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1792 1664v128h-256v-128h256zM384 128v640h768v128H256V128H128v1381l154 155h102v-512h768v128H512v384h128v-256h128v256h384v128H229L0 1562V128q0-27 10-50t27-40 41-28 50-10h1536q27 0 50 10t40 27 28 41 10 50v512h-128V128h-128v512h-128V128H384zm1664 768v1024q0 27-10 50t-27 40-41 28-50 10h-512q-27 0-50-10t-40-27-28-41-10-50V896q0-27 10-50t27-40 41-28 50-10h512q27 0 50 10t40 27 28 41 10 50zm-128 1024V896h-512v1024h512z" />
            </svg>
        }
    }
}
