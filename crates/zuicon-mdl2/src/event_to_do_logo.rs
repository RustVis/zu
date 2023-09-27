// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct EventToDoLogo {}

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

impl Component for EventToDoLogo {
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
                data-icon={ "EventToDoLogo" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M128 640v768h256v128H0V128h256V0h128v128h768V0h128v128h256v640h-128V640H128zm128-384H128v256h1280V256h-128v128h-128V256H384v128H256V256zm1792 828q0 20-8 39t-23 34q-208 208-412 414t-413 412q-32 30-75 30t-73-30l-411-410q-15-14-23-33t-8-41q0-43 31-74l177-177q31-31 74-31 42 0 73 31l160 160 576-575q30-30 73-30 20 0 40 8t35 22l176 177q14 14 22 33t9 41zM885 1642l143-143-144-144-143 144 144 143zm1025-559l-144-144q-199 200-395 397t-397 397q36 36 71 72t72 72q200-199 397-396t396-398z" />
            </svg>
        }
    }
}
