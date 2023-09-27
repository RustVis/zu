// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Headset {}

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

impl Component for Headset {
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
                data-icon={ "Headset" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1024 0q106 0 204 27t183 78 156 120 120 155 77 184 28 204v448q0 40-15 75t-41 61-61 41-75 15h-192V768h192q17 0 33 3t31 9q0-135-49-253t-134-207-203-140-254-52q-88 0-170 23t-153 64-129 100-100 130-65 153-23 170v12q15-5 31-8t33-4h192v640H512v128q0 53 20 99t55 82 81 55 100 20q0-27 10-50t27-40 41-28 50-10h256q27 0 50 10t40 27 28 41 10 50v128q0 27-10 50t-27 40-41 28-50 10H896q-27 0-50-10t-40-27-28-41-10-50q-80 0-149-30t-122-82-83-122-30-150v-140q-29-10-52-28t-40-41-26-52-10-59V768q0-106 27-204t78-183 120-156 155-120 184-77 204-28zM896 1920h256v-128H896v128zM448 896q-26 0-45 19t-19 45v256q0 26 19 45t45 19h64V896h-64zm1216 64q0-26-19-45t-45-19h-64v384h64q26 0 45-19t19-45V960z" />
            </svg>
        }
    }
}
