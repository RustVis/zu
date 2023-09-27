// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct TouchPointer {}

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

impl Component for TouchPointer {
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
                data-icon={ "TouchPointer" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1856 768q40 0 75 15t61 41 41 61 15 75v384q0 119-45 224t-124 183-183 123-224 46q-141 0-266-55t-228-157l-319-318-272 136-138-277L0 1499V165l896 896V320q0-40 15-75t41-61 61-41 75-15q40 0 75 15t61 41 41 61 15 75v331q27-10 59-10 49 0 95 23t72 65q44-25 94-25 51 0 93 24t69 65q44-25 94-25zM424 960h190L128 475v714l156-156 161 321 141-71-162-323zm1496 0q0-26-19-45t-45-19q-26 0-45 19t-19 45v128q0 26-19 45t-45 19q-26 0-45-19t-19-45V896q0-26-19-45t-45-19q-26 0-45 19t-19 45v128q0 26-19 45t-45 19q-26 0-45-19t-19-45V832q0-26-19-45t-45-19q-26 0-45 19t-19 45v128q0 26-19 45t-45 19q-26 0-45-19t-19-45V320q0-26-19-45t-45-19q-26 0-45 19t-19 45v787q0 33-18 60t-49 41q-19 8-42 8-44 0-78-32l-75-75q-21-21-50-21-30 0-51 21-11 11-16 24t-5 27q0 30 21 51l408 408q81 85 184 129t219 44q93 0 174-35t142-96 96-142 36-175V960z" />
            </svg>
        }
    }
}
