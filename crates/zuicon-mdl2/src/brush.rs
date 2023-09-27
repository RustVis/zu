// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Brush {}

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

impl Component for Brush {
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
                data-icon={ "Brush" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1803 128q51 0 95 19t78 53 52 77 20 96q0 49-18 93t-54 80q-303 303-605 606t-606 606q-6 61-33 114t-69 92-98 61-117 23H192q-40 0-75-15t-61-41-41-61-15-75v-64h64q26 0 45-19t19-45q0-61 22-116t62-98 92-70 114-33q303-303 606-605t606-606q35-35 79-53t94-19zM576 1435q55 24 98 67t67 98l116-116q-61-104-165-165l-116 116zm-128 485q40 0 75-15t61-41 41-61 15-75q0-40-15-75t-41-61-61-41-75-15q-40 0-75 15t-61 41-41 61-15 75q0 55-29 102t-80 71q18 19 45 19h256zM1886 456q34-34 34-83 0-24-9-45t-25-38-37-25-46-9q-49 0-83 34l-935 936q99 66 165 165l936-935z" />
            </svg>
        }
    }
}
