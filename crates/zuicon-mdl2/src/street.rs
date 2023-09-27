// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Street {}

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

impl Component for Street {
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
                data-icon={ "Street" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M861 1021l-90-90 637-638 637 638-90 90-35-34v549h-384v-384h-256v384H896V987l-35 34zm163 387h128v-384h512v384h128V859l-384-384-384 384v549zm1024 256v128H0v-128h2048zM256 1146q-56-11-102-40t-81-72-54-93T0 832q0-73 14-132t37-114 53-111 64-122q10-20 26-38t37-31 43-20 46-8q48 0 81 22t57 57 43 74 36 74l15 30q21 42 37 77t28 70 17 77 6 95q0 57-19 108t-53 94-82 71-102 41v390H256v-390zm64-762q-9 0-23 18t-32 46-35 62-33 65-27 56-16 35q-15 38-20 82t-6 84q0 40 15 75t41 61 61 41 75 15q40 0 75-15t61-41 41-61 15-75q0-40-5-84t-21-82q-4-10-15-35t-27-56-34-65-35-62-31-46-24-18z" />
            </svg>
        }
    }
}
