// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Mouse3Fill {}

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

impl Component for Mouse3Fill {
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
                class={ props.class.unwrap_or("") }
                width={ props.width.unwrap_or("1em") }
                height={ props.height.unwrap_or("1em") }
                focusable={ "false" }
                data-icon={ "mouse3-fill" }
                viewBox={ "0 0 1024 1024" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M8.5.069A15.328 15.328 0 0 0 7 0c-.593 0-1.104.157-1.527.463-.418.302-.717.726-.93 1.208-.386.873-.522 2.01-.54 3.206l4.497 1V.069zM3.71 5.836 3.381 6A2.5 2.5 0 0 0 2 8.236v2.576C2 13.659 4.22 16 7 16h2c2.78 0 5-2.342 5-5.188V8.123l-9-2v.003l.008.353c.007.3.023.715.053 1.175.063.937.186 2.005.413 2.688a.5.5 0 1 1-.948.316c-.273-.817-.4-2-.462-2.937A30.16 30.16 0 0 1 4 6.003c0-.034.003-.067.01-.1l-.3-.067zM14 7.1V5.187c0-1.13-.272-2.044-.748-2.772-.474-.726-1.13-1.235-1.849-1.59A7.495 7.495 0 0 0 9.5.212v5.887l4.5 1z"/>
            </svg>
        }
    }
}
