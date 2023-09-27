// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Decimals {}

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

impl Component for Decimals {
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
                data-icon={ "Decimals" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M465 265q90 0 155 32t110 85 72 124 43 147 20 154 5 147q0 71-7 150t-25 157-48 149-79 126-116 86-159 33q-86 0-150-31t-109-82-75-120-46-142-23-150-7-142q0-73 6-155t23-162 47-154 80-130 118-89 165-33zm-16 1285q67 0 115-29t82-78 53-111 30-127 14-126 3-111q0-48-2-110t-12-129-28-131-51-115-80-82-116-31q-71 0-121 31t-84 81-54 116-31 132-14 132-3 116q0 50 3 111t13 125 30 124 53 108 82 75 118 29zm574-116q44 0 73 29t30 74q0 21-8 39t-22 32-33 22-40 8q-44 0-72-28t-29-73q0-44 28-73t73-30zm594-1169q90 0 155 32t110 85 72 124 43 147 20 154 5 147q0 71-7 150t-25 157-48 149-79 126-116 86-159 33q-86 0-150-31t-109-82-75-120-46-142-23-150-7-142q0-73 6-155t23-162 47-154 80-130 118-89 165-33zm-16 1285q67 0 115-29t82-78 53-111 30-127 14-126 3-111q0-48-2-110t-12-129-28-131-51-115-80-82-116-31q-71 0-121 31t-84 81-54 116-31 132-14 132-3 116q0 50 3 111t13 125 30 124 53 108 82 75 118 29z" />
            </svg>
        }
    }
}
