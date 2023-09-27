// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct CalendarYear {}

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

impl Component for CalendarYear {
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
                data-icon={ "CalendarYear" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1664 512h256v1536H256V512h256V384h128v128h896V384h128v128zm128 128h-128v128h128V640zm-256 0H640v128h896V640zm-1024 0H384v128h128V640zM384 1920h1408V896H384v1024zM256 384V256H128v1408H0V128h256V0h128v128h896V0h128v128h256v128h-256v128h-128V256H384v128H256zm384 1024v-128h128v128H640zm256 0v-128h128v128H896zm256 0v-128h128v128h-128zm256 0v-128h128v128h-128zm-768 256v-128h128v128H640zm256 0v-128h128v128H896zm256 0v-128h128v128h-128zm-256-512v-128h128v128H896zm256 0v-128h128v128h-128zm256 0v-128h128v128h-128z" />
            </svg>
        }
    }
}
