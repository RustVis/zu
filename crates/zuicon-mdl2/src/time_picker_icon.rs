// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct TimePickerIcon {}

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

impl Component for TimePickerIcon {
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
                data-icon={ "TimePickerIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M192 768q40 0 75 15t61 41 41 61 15 75v128q0 40-15 75t-41 61-61 41-75 15q-40 0-75-15t-61-41-41-61-15-75V960q0-40 15-75t41-61 61-41 75-15zm64 336V944q0-26-19-45t-45-19q-26 0-45 19t-19 45v160q0 26 19 45t45 19q26 0 45-19t19-45zm448-336q40 0 75 15t61 41 41 61 15 75v128q0 40-15 75t-41 61-61 41-75 15q-40 0-75-15t-61-41-41-61-15-75V960q0-40 15-75t41-61 61-41 75-15zm64 336V944q0-26-19-45t-45-19q-26 0-45 19t-19 45v160q0 26 19 45t45 19q26 0 45-19t19-45zm576-336q40 0 75 15t61 41 41 61 15 75v128q0 40-15 75t-41 61-61 41-75 15q-40 0-75-15t-61-41-41-61-15-75V960q0-40 15-75t41-61 61-41 75-15zm64 336V944q0-26-19-45t-45-19q-26 0-45 19t-19 45v160q0 26 19 45t45 19q26 0 45-19t19-45zm448-336q40 0 75 15t61 41 41 61 15 75v128q0 40-15 75t-41 61-61 41-75 15q-40 0-75-15t-61-41-41-61-15-75V960q0-40 15-75t41-61 61-41 75-15zm64 336V944q0-26-19-45t-45-19q-26 0-45 19t-19 45v160q0 26 19 45t45 19q26 0 45-19t19-45zm-896-80q-26 0-45-19t-19-45q0-26 19-45t45-19q26 0 45 19t19 45q0 26-19 45t-45 19zm0 256q-26 0-45-19t-19-45q0-26 19-45t45-19q26 0 45 19t19 45q0 26-19 45t-45 19zm403-659l-403-402-403 402-90-90 493-494 493 494-90 90zm-403 1390l-493-494 90-90 403 402 403-402 90 90-493 494z" />
            </svg>
        }
    }
}
