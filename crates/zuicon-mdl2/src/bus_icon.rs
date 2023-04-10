// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct BusIcon {}

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

impl Component for BusIcon {
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
                data-icon={ "BusIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1920 640v256h-128v960q0 34-3 68t-16 62-38 45-71 17q-25 0-53 1t-57-1-55-8-47-21-32-38-12-61H512q0 27-10 50t-27 40-41 28-50 10q-69 0-118-1t-79-17-45-56-14-118V896H0V640h128V320q0-40 15-75t41-61 61-41 75-15h1280q40 0 75 15t61 41 41 61 15 75v320h128zm-256 640H256v256h128v-40q0-22 4-42t18-33 42-13q28 0 41 13t19 32 5 42-1 41h896v-40q0-22 4-42t18-33 42-13q28 0 41 13t19 32 5 42-1 41h128v-256zm-768-128V640H256v512h640zm768-512h-640v512h640V640zM320 256q-26 0-45 19t-19 45v192h1408V320q0-26-19-45t-45-19H320zm1344 1536v-128H256v128h1408z" />
            </svg>
        }
    }
}
