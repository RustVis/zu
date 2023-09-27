// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Coupon {}

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

impl Component for Coupon {
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
                data-icon={ "Coupon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M624 832q0 36-14 68t-38 56-56 38-68 14q-36 0-68-14t-56-38-38-56-14-68q0-36 14-68t38-56 56-38 68-14q36 0 68 14t56 38 38 56 14 68zm-176 80q33 0 56-23t24-57q0-33-23-56t-57-24q-33 0-56 23t-24 57q0 33 23 56t57 24zm512 128q36 0 68 14t56 38 38 56 14 68q0 36-14 68t-38 56-56 38-68 14q-36 0-68-14t-56-38-38-56-14-68q0-36 14-68t38-56 56-38 68-14zm0 256q33 0 56-23t24-57q0-33-23-56t-57-24q-33 0-56 23t-24 57q0 33 23 56t57 24zM842 640h108l-384 768H458l384-768zm566-256l640 640-640 640H0V384h1408zm-53 1152l512-512-512-512H128v1024h1227zm181-576q26 0 45 19t19 45q0 26-19 45t-45 19q-26 0-45-19t-19-45q0-26 19-45t45-19z" />
            </svg>
        }
    }
}
