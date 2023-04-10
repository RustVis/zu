// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct BreakfastIcon {}

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

impl Component for BreakfastIcon {
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
                data-icon={ "BreakfastIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1408 592q-26 0-45-19t-19-45q0-51 19-98t56-83l79-80q38-38 38-91 0-26 19-45t45-19q26 0 45 19t19 45q0 51-19 98t-56 83l-79 80q-38 38-38 91 0 26-19 45t-45 19zm-384 0q-26 0-45-19t-19-45q0-51 19-98t56-83l79-80q38-38 38-91 0-26 19-45t45-19q26 0 45 19t19 45q0 51-19 98t-56 83l-79 80q-38 38-38 91 0 26-19 45t-45 19zm832 176q40 0 75 15t61 41 41 61 15 75v384q0 40-15 75t-41 61-61 41-75 15h-57q-2 7-3 13t-4 12v39q0 66-25 124t-69 102-102 69-124 25h-384q-78 0-144-35t-110-93H334q-66 0-124-25t-102-68-69-102-25-125v-64h256q0-79 30-149t83-122 122-83 149-30q30 0 58 5t56 14V640h1024v128h64zM654 1152q-53 0-99 20t-82 55-55 81-20 100h370v-228q-26-13-54-20t-60-8zm-320 512h441q-7-29-7-64v-64H153q10 28 28 51t41 41 52 26 60 10zm463 67v1l1 2v-1l-1-2zm867-131V768H896v832q0 40 15 75t41 61 61 41 75 15h384q40 0 75-15t61-41 41-61 15-75zm256-256V960q0-26-19-45t-45-19h-64v512h64q26 0 45-19t19-45z" />
            </svg>
        }
    }
}
