// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Personalize {}

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

impl Component for Personalize {
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
                data-icon={ "Personalize" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1024 1792v128h256v128H640v-128h256v-128H0V640h1024L895 768H128v896h1664V896l128-128v1024h-896zm-512-384q27 0 50-10t40-27 28-41 10-50q0-49 17-93t48-78 72-56 91-28l711-711q28-28 65-43t76-15q41 0 77 15t64 43 42 63 16 79q0 39-15 76t-43 65l-711 711q-5 48-27 90t-56 72-78 48-93 18H256v-128h256zM1720 384q-29 0-50 21l-445 445q57 44 101 101l445-445q21-21 21-50 0-30-21-51t-51-21zm-678 649q61 40 101 101l92-91q-42-60-102-102l-91 92zm-18 247q0-27-10-50t-27-40-41-28-50-10q-27 0-50 10t-40 27-28 41-10 50q0 68-34 128h162q26 0 49-10t41-27 28-41 10-50z" />
            </svg>
        }
    }
}
