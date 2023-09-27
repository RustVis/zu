// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct HintText {}

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

impl Component for HintText {
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
                data-icon={ "HintText" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M2048 256v384h-128V384H128v1152h896v128H0V256h2048zm-512 384q106 0 199 40t163 109 110 163 40 200q0 106-38 195t-111 166l-32 34q-12 13-25 29t-23 36-17 38-7 38v168q0 41-15 76t-42 62-62 41-76 16h-128q-41 0-76-15t-62-42-41-61-16-77v-168q-1-25-11-49t-25-46-33-42-35-38q-72-76-110-165t-39-196q0-106 40-199t109-163 163-110 200-40zm125 1155h-250v61q0 25 18 43t43 18h128q25 0 43-18t18-43v-61zm9-134q10-57 32-98t50-76 57-66 54-69 41-85 16-115q0-80-30-150t-82-122-122-82-150-30q-80 0-150 30t-122 82-82 122-30 150q0 76 29 147t83 128q27 27 49 52t40 53 30 58 19 71h268z" />
            </svg>
        }
    }
}
