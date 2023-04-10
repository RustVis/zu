// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct EDiscoveryIcon {}

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

impl Component for EDiscoveryIcon {
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
                data-icon={ "eDiscoveryIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M220 1792l-42 128h1486v128H0q2-6 12-37t25-73 30-91 29-88 23-68 9-27V896H0V768l896-448V0h383v256h-255v64l896 448v128h-128v128h-128V896h-128v128h-128V896h-128v128h-128V896h-128v896H220zm420-128V896H512v768h128zm256 0V896H768v768h128zm64-1233L286 768h1348L960 431zM256 896v768h128V896H256zm1792 1088q0 26-19 45t-45 19q-26 0-45-19l-291-290q-39 26-84 39t-92 14q-66 0-124-25t-102-68-69-102-25-125q0-66 25-124t68-101 102-69 125-26q66 0 124 25t101 69 69 102 26 124q0 47-13 92t-40 84l290 291q19 19 19 45zm-768-512q0 40 15 75t41 61 61 41 75 15q40 0 75-15t61-41 41-61 15-75q0-40-15-75t-41-61-61-41-75-15q-40 0-75 15t-61 41-41 61-15 75z" />
            </svg>
        }
    }
}
