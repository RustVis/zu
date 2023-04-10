// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct MedicalIcon {}

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

impl Component for MedicalIcon {
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
                data-icon={ "MedicalIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1728 640q66 0 124 25t101 69 69 102 26 124q0 57-19 109t-53 93-81 71-103 41v102q0 89-22 173t-62 160-98 137-129 107-155 70-174 25q-91 0-174-25t-154-70-129-107-98-137-63-159-22-174v-229q-123-11-218-59T133 962 34 781 0 558V0h320q26 0 45 19t19 45q0 26-19 45t-45 19H128v430q0 106 29 192t87 147 140 94 192 33q101 0 184-31t141-89 91-140 32-185V128H832q-26 0-45-19t-19-45q0-26 19-45t45-19h320v558q0 120-34 223t-99 181-160 126-219 59v229q0 107 38 205t107 174 162 120 205 45q111 0 204-45t162-120 107-173 39-206v-102q-56-12-103-41t-81-70-53-94-19-109q0-66 25-124t68-101 102-69 125-26zm0 512q40 0 75-15t61-41 41-61 15-75q0-40-15-75t-41-61-61-41-75-15q-40 0-75 15t-61 41-41 61-15 75q0 40 15 75t41 61 61 41 75 15z" />
            </svg>
        }
    }
}
