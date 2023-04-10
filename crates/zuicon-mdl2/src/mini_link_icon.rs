// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct MiniLinkIcon {}

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

impl Component for MiniLinkIcon {
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
                data-icon={ "MiniLinkIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1728 1408q66 0 124 25t101 69 69 102 26 124q0 66-25 124t-69 102-102 69-124 25h-256q-57 0-109-19t-93-53-71-81-41-103H890q-12 56-41 103t-70 81-94 53-109 19H320q-66 0-124-25t-102-68-69-102-25-125q0-66 25-124t68-101 102-69 125-26h256q57 0 109 19t93 53 71 81 41 103h268q12-56 41-103t70-81 94-53 109-19h256zM576 1920q30 0 58-9t53-26 42-40 28-53H576q-26 0-45-19t-19-45q0-26 19-45t45-19h181q-10-29-28-52t-42-41-52-26-59-9H320q-40 0-75 15t-61 41-41 61-15 75q0 40 15 75t41 61 61 41 75 15h256zm1152 0q40 0 75-15t61-41 41-61 15-75q0-40-15-75t-41-61-61-41-75-15h-256q-30 0-58 9t-53 26-42 40-28 53h181q26 0 45 19t19 45q0 26-19 45t-45 19h-181q10 29 28 52t42 41 52 26 59 9h256z" />
            </svg>
        }
    }
}
