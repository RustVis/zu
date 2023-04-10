// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct FileTemplateIcon {}

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

impl Component for FileTemplateIcon {
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
                data-icon={ "FileTemplateIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1243 0l549 549v219h-128V640h-512V128H256v1792h768v128H128V0h1115zm37 512h293l-293-293v293zm640 1408h128v128h-128v-128zm-256 0h128v128h-128v-128zm-256 0h128v128h-128v-128zm256-1024h128v128h-128V896zm-256 0h128v128h-128V896zm-256 1024h128v128h-128v-128zm768-256h128v128h-128v-128zm-768 0h128v128h-128v-128zm768-256h128v128h-128v-128zm-768 0h128v128h-128v-128zm768-256h128v128h-128v-128zm-768 0h128v128h-128v-128zm896-256v128h-128V896h128zm-896 0h128v128h-128V896z" />
            </svg>
        }
    }
}
