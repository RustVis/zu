// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Fax {}

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

impl Component for Fax {
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
                data-icon={ "Fax" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M320 1024q26 0 45 19t19 45q0 26-19 45t-45 19q-26 0-45-19t-19-45q0-26 19-45t45-19zm1216-640q107 0 205 49 90 45 149 118t94 162 50 185 14 190q0 93-14 190t-49 185-95 161-149 119q-98 49-205 49h-128v256H384v-256H0V896q0-27 10-50t27-40 41-28 50-10h256V0h1024v384h128zM512 768h768V128H512v640zm768 640H512v512h768v-512zm256-128V896H128v768h256v-384h1152zm0 384q74 0 131-26t100-70 72-102 47-122 26-130 8-126q0-60-8-126t-25-130-47-122-72-102-100-70-132-26h-64q-26 0-45 19t-19 45v128q0 28 13 41t32 19 42 5 41-1q27 0 50 10t40 27 28 41 10 50v384q0 27-10 50t-27 40-41 28-50 10h-40q-22 0-42 4t-33 18-13 42v128q0 26 19 45t45 19h64z" />
            </svg>
        }
    }
}
