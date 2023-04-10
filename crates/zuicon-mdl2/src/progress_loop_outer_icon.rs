// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct ProgressLoopOuterIcon {}

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

impl Component for ProgressLoopOuterIcon {
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
                data-icon={ "ProgressLoopOuterIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M960 131q-128 9-245 53T498 300 320 470 194 687L73 648q54-138 144-252t205-199T675 63 960 3v128zm894 556q-48-118-126-216t-178-171-217-116-245-53V3q148 9 285 59t252 135 206 198 145 253l-122 39zM128 1024q0 99 21 194t63 183 100 166 135 142l-75 104q-88-73-157-162T98 1460t-73-211-25-225q0-65 8-128t25-127l122 40q-13 53-20 107t-7 108zm1887-255q16 63 24 126t9 129q0 115-25 225t-73 211-117 190-157 163l-75-104q76-64 135-142t100-165 62-183 22-195q0-54-7-108t-20-107l122-40zm-991 1151q127 0 246-34t227-102l75 103q-123 79-262 120t-286 41q-146 0-285-41t-263-120l75-103q107 67 226 101t247 35z" />
            </svg>
        }
    }
}
