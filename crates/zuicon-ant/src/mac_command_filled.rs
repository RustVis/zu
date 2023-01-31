// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct MacCommandFilled {}

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

impl Component for MacCommandFilled {
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
                class={ props.class.unwrap_or("") }
                width={ props.width.unwrap_or("1em") }
                height={ props.height.unwrap_or("1em") }
                focusable={ "false" }
                data-icon={ "mac-command" }
                viewBox={ "0 0 1024 1024" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <defs><style type="text/css"></style></defs><path d="M624 672c0 26.5 21.5 48 48 48s48-21.5 48-48-21.5-48-48-48h-48v48zM720 352c0-26.5-21.5-48-48-48s-48 21.5-48 48v48h48c26.5 0 48-21.5 48-48z" p-id="7960"></path><path d="M928 64H96c-17.7 0-32 14.3-32 32v832c0 17.7 14.3 32 32 32h832c17.7 0 32-14.3 32-32V96c0-17.7-14.3-32-32-32zM672 560c61.9 0 112 50.1 112 112s-50.1 112-112 112-112-50.1-112-112v-48h-96v48c0 61.9-50.1 112-112 112s-112-50.1-112-112 50.1-112 112-112h48v-96h-48c-61.9 0-112-50.1-112-112s50.1-112 112-112 112 50.1 112 112v48h96v-48c0-61.9 50.1-112 112-112s112 50.1 112 112-50.1 112-112 112h-48v96h48z" p-id="7961"></path><path d="M464 464h96v96h-96zM352 304c-26.5 0-48 21.5-48 48s21.5 48 48 48h48v-48c0-26.5-21.5-48-48-48zM304 672c0 26.5 21.5 48 48 48s48-21.5 48-48v-48h-48c-26.5 0-48 21.5-48 48z" p-id="7962"></path>
            </svg>
        }
    }
}
