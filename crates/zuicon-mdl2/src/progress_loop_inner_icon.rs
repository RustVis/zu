// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct ProgressLoopInnerIcon {}

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

impl Component for ProgressLoopInnerIcon {
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
                data-icon={ "ProgressLoopInnerIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M448 1024q0 60 12 118t36 112l-111 64q-32-69-48-143t-17-151q0-91 22-176t64-160 99-138 129-111 153-78 173-38v129q-109 12-202 61T595 640 487 815t-39 209zm640-701q90 8 172 38t154 77 129 111 99 139 63 160 23 176q0 77-16 151t-49 144l-111-64q23-55 35-113t13-118q0-110-39-208t-108-176-162-126-203-62V323zm-64 1277q69 0 134-16t125-46 111-74 93-99l111 65q-50 70-113 125t-137 94-156 58-168 21q-86 0-168-20t-156-59-138-94-113-126l112-64q41 55 92 98t111 74 125 47 135 16z" />
            </svg>
        }
    }
}
