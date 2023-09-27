// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct MiracastLogoLarge {}

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

impl Component for MiracastLogoLarge {
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
                data-icon={ "MiracastLogoLarge" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M2048 256v1152h-768v-128h640V384H384v256H256V256h1792zM1024 1280h128v256q0 27-10 50t-27 40-41 28-50 10H128q-27 0-50-10t-40-27-28-41-10-50V896q0-27 10-50t27-40 41-28 50-10h512v128H128v640h896v-256zM768 768q80 0 150 30t122 82 82 122 30 150h-128q0-53-20-99t-55-82-81-55-100-20V768zm0 256q27 0 50 10t40 27 28 41 10 50H768v-128zm512 128q0-106-40-199t-110-162-163-110-199-41V512q133 0 249 50t204 137 137 203 50 250h-128z" />
            </svg>
        }
    }
}
