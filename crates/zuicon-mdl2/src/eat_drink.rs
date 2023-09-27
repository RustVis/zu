// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct EatDrink {}

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

impl Component for EatDrink {
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
                data-icon={ "EatDrink" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M960 0q26 0 45 19t19 45v704q0 53-20 99t-55 81-82 55-99 21v960q0 26-19 45t-45 19q-26 0-45-19t-19-45v-960q-53 0-99-20t-81-55-55-81-21-100V64q0-26 19-45t45-19q26 0 45 19t19 45v704q0 27 10 50t27 40 41 28 50 10V64q0-26 19-45t45-19q26 0 45 19t19 45v832q27 0 50-10t40-27 28-41 10-50V64q0-26 19-45t45-19zm704 0v1984q0 26-19 45t-45 19q-26 0-45-19t-19-45v-576q-37 0-80 1t-85-1-82-12-70-31-49-57-18-92V448q0-93 35-174t96-142 142-96 175-36h64zm-128 134q-56 11-102 40t-81 72-54 93-19 109v768q0 26 19 45t45 19h192V134z" />
            </svg>
        }
    }
}
