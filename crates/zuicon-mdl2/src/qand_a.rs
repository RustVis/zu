// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct QandA {}

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

impl Component for QandA {
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
                data-icon={ "QandA" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1792 384h128v1280h-256v384l-384-384H768l128-128h437l203 203v-203h256V384zM768 1408l-384 384v-384H0V0h1664v1408H768zm-640-128h384v203l203-203h821V128H128v1152zm640-128v-128h128v128H768zm64-768q-27 0-50 10t-40 27-28 41-10 50H576q0-53 20-99t55-82 81-55 100-20q53 0 99 20t82 55 55 81 20 100q0 46-14 81t-37 65-52 57-59 58q-20 20-25 42t-6 47v16q0 8 1 18H768v-48q0-46 14-81t35-63 47-50 46-45 36-45 14-52q0-27-10-50t-27-40-41-28-50-10z" />
            </svg>
        }
    }
}
