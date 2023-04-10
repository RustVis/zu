// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct ProFootballIcon {}

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

impl Component for ProFootballIcon {
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
                data-icon={ "ProFootballIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1536 0q106 0 199 40t163 109 110 163 40 200q0 212-55 408t-155 367-240 311-311 240-367 155-408 55q-106 0-199-40t-163-109-110-163-40-200q0-212 55-408t155-367 240-311 311-240 367-155 408-55zM512 1920q194 0 373-50t336-142 286-221 220-285 142-336 51-374q0-79-30-149t-82-122-123-83-149-30q-194 0-373 50T827 320 541 541 321 826t-142 336-51 374q0 80 30 149t82 122 122 83 150 30zm749-1325l192 192-90 90-51-50-101 101 50 51-90 90-51-51-102 102 51 51-90 90-51-50-101 101 50 51-90 90-192-192 90-90 51 50 101-101-50-51 90-90 51 51 102-102-51-51 90-90 51 50 101-101-50-51 90-90z" />
            </svg>
        }
    }
}
