// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct DeveloperTools {}

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

impl Component for DeveloperTools {
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
                data-icon={ "DeveloperTools" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1664 1152h-128v704q0 40-15 75t-41 61-61 41-75 15q-40 0-75-15t-61-41-41-61-15-75v-704h-128v-128h256V475l-128-128V0h384v347l-128 128v549h256v128zm-384-859l64 65 64-65V128h-128v165zm128 859h-128v704q0 26 19 45t45 19q26 0 45-19t19-45v-704zM704 0q66 0 124 25t101 69 69 102 26 124q0 75-34 142t-94 113v1281q0 40-15 75t-41 61-61 41-75 15q-40 0-75-15t-61-41-41-61-15-75V575q-60-45-94-112t-34-143q0-66 25-124t68-101 102-69T704 0zm64 504q28-16 51-34t41-39 26-49 10-62q0-30-9-58t-26-52-41-42-52-28v180l-64 64-64-64V140q-29 10-52 28t-40 42-26 52-10 58q0 35 9 62t27 48 40 40 52 34v1352q0 26 19 45t45 19q26 0 45-19t19-45V504z" />
            </svg>
        }
    }
}
