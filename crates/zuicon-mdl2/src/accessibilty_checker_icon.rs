// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct AccessibiltyCheckerIcon {}

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

impl Component for AccessibiltyCheckerIcon {
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
                data-icon={ "AccessibiltyCheckerIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M128 128v1792h384v128H0V0h1115l549 549v91h-640V128H128zm1024 91v293h293l-293-293zm384 1061l-192 256-192-256h128V768h128v512h128zm512 64l-256 192v-128h-192l96-128h96v-128l256 192zm-1401 64q9-81 39-155t82-139l91 91q-69 90-84 203H647zm128 128q15 113 84 203l-91 91q-51-64-81-138t-40-156h128zm377-505q-113 15-203 84l-91-91q64-51 138-81t156-40v128zm421 708q69-90 84-203h128q-9 81-39 155t-82 139l-91-91zm-293 174q113-15 203-84l91 91q-64 51-138 81t-156 40v-128zm-422 7l91-91q90 69 203 84v128q-81-9-155-39t-139-82z" />
            </svg>
        }
    }
}
