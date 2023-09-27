// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct MailReminder {}

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

impl Component for MailReminder {
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
                data-icon={ "MailReminder" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M2048 384v640h-128V648l-896 447-896-447v888h896v128H0V384h2048zM1024 953l881-441H143l881 441zm576 199q66 0 124 25t101 69 69 102 26 124v192h128v128h-256v64q0 40-15 75t-41 61-61 41-75 15q-40 0-75-15t-61-41-41-61-15-75v-64h-256v-128h128v-192q0-66 25-124t68-101 102-69 125-26zm64 640h-128v64q0 26 19 45t45 19q26 0 45-19t19-45v-64zm128-128v-192q0-40-15-75t-41-61-61-41-75-15q-40 0-75 15t-61 41-41 61-15 75v192h384z" />
            </svg>
        }
    }
}
