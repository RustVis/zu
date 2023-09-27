// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct SplitObject {}

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

impl Component for SplitObject {
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
                data-icon={ "SplitObject" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1600 1408q66 0 124 25t101 69 69 102 26 124q0 66-25 124t-69 102-102 69-124 25q-66 0-124-25t-102-68-69-102-25-125q0-74 32-140t91-112l-302-603q-69 23-141 23t-141-23l-302 603q58 46 90 112t33 140q0 66-25 124t-69 102-102 69-124 25q-66 0-124-25t-102-68-69-102-25-125q0-66 25-124t68-101 102-69 125-26q41 0 83 11l302-603q-45-31-81-71t-61-89-38-100-13-108q0-93 35-174t96-143 142-96T960 0q93 0 174 35t143 96 96 142 35 175q0 55-13 107t-38 101-61 88-81 72l302 603q42-11 83-11zM320 1920q40 0 75-15t61-41 41-61 15-75q0-40-15-75t-41-61-61-41-75-15q-40 0-75 15t-61 41-41 61-15 75q0 40 15 75t41 61 61 41 75 15zM640 448q0 66 25 124t68 102 102 69 125 25q66 0 124-25t101-68 69-102 26-125q0-66-25-124t-69-101-102-69-124-26q-66 0-124 25t-102 69-69 102-25 124zm960 1472q40 0 75-15t61-41 41-61 15-75q0-40-15-75t-41-61-61-41-75-15q-40 0-75 15t-61 41-41 61-15 75q0 40 15 75t41 61 61 41 75 15z" />
            </svg>
        }
    }
}
