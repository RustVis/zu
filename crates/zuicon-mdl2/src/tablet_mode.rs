// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct TabletMode {}

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

impl Component for TabletMode {
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
                data-icon={ "TabletMode" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1920 1035q29 10 52 28t41 42 26 52 9 59v256q0 119-45 224t-124 183-183 123-224 46q-110 0-208-39t-176-108-127-162-62-203H128q-27 0-50-10t-40-27-28-41-10-50V256q0-27 10-50t27-40 41-28 50-10h1664q27 0 50 10t40 27 28 41 10 50v779zM896 1280q0-53 20-99t55-82 81-55 100-20V704q0-40 15-75t41-61 61-41 75-15q40 0 74 15t61 41 41 61 15 75v160q0 79 1 160h256V256H128v1152h768v-128zm1024-64q0-26-19-45t-45-19h-448V704q0-26-19-45t-45-19q-26 0-45 19t-19 45v768q0 26-19 45t-45 19q-26 0-45-19t-19-45v-320q-27 0-50 10t-40 27-28 41-10 50v192q0 93 35 174t96 142 142 96 175 36q93 0 174-35t142-96 96-142 36-175v-256z" />
            </svg>
        }
    }
}
