// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct AustralianRulesIcon {}

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

impl Component for AustralianRulesIcon {
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
                data-icon={ "AustralianRulesIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1536 0q106 0 199 40t163 109 110 163 40 200q0 212-55 408t-155 367-240 311-311 240-367 155-408 55q-106 0-199-40t-163-109-110-163-40-200q0-212 55-408t155-367 240-311 311-240 367-155 408-55zm0 128q-194 0-373 50T827 320 541 541 321 826t-142 336-51 374q0 60 18 117t53 105l447-446-83-83q-19-19-19-45t19-45 45-19q26 0 45 19l83 82 101-101-82-83q-19-19-19-45t19-45 45-19q26 0 45 19l83 83 102-102-83-83q-19-19-19-45t19-45 45-19q26 0 45 19l83 82 101-101-82-83q-19-19-19-45t19-45 45-19q26 0 45 19l83 83 446-447q-48-35-105-53t-117-18zM512 1920q194 0 373-50t336-142 286-221 220-285 142-336 51-374q0-60-18-117t-53-105l-447 446 83 83q19 19 19 45t-19 45-45 19q-14 0-24-5t-21-14l-83-82-101 101 82 83q19 19 19 45t-19 45-45 19q-26 0-45-19l-83-83-102 102 83 83q19 19 19 45t-19 45-45 19q-14 0-24-5t-21-14l-83-82-101 101 82 83q19 19 19 45t-19 45-45 19q-26 0-45-19l-83-83-446 447q48 35 105 53t117 18z" />
            </svg>
        }
    }
}
