// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct ServerProcessesIcon {}

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

impl Component for ServerProcessesIcon {
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
                data-icon={ "ServerProcessesIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M2038 1488l-124 51q12 62 0 122l124 51-49 119-124-52q-17 25-38 47t-48 39l52 124-119 49-51-124q-30 6-61 6t-61-6l-51 124-119-49 52-124q-51-35-86-86l-124 52-49-119 124-51q-12-61 0-122l-124-51 49-119 124 52q35-51 86-86l-52-124 119-49 51 124q62-12 122 0l51-124 119 49-52 124q25 17 47 38t39 48l124-52 49 119zm-365 289q37-15 63-42t41-62 14-72-14-74q-14-36-41-63t-63-41q-35-15-73-15-39 0-73 15-36 14-63 41t-41 63q-31 73 0 146 14 36 41 63t63 41q73 31 146 0zm375-1649v896h-128V640H128v1024h896v128H0V128h2048zm-128 384V256H128v256h1792zM896 1408v-140q-31-11-60-34l-121 69-64-110 120-70q-3-16-3-35 0-18 3-35l-120-70 64-110 121 69q29-23 60-34V768h128v140q17 6 31 14t29 20l121-69 64 110-120 70q3 17 3 35 0 19-3 35l120 70-64 110-121-69q-14 11-28 19t-32 15v140H896zm-32-320q0 40 28 68t68 28q40 0 68-28t28-68q0-40-28-68t-68-28q-40 0-68 28t-28 68z" />
            </svg>
        }
    }
}
