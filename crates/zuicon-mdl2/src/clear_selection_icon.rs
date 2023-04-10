// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct ClearSelectionIcon {}

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

impl Component for ClearSelectionIcon {
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
                data-icon={ "ClearSelectionIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1713 896q69 0 130 26t106 72 72 107 27 131q0 66-25 127t-73 110l-449 448-90-90 448-449q29-29 45-67t16-79q0-43-16-80t-45-66-66-45-81-17q-41 0-79 16t-67 45l-195 195h165v128h-384v-384h128v165q47-47 93-99t97-95 111-71 132-28zm79-128h-128V640h128v128zm0-256h-128V384h128v128zm0-256h-128V128h128v128zm-256 0h-128V128h128v128zm-256 0h-128V128h128v128zM896 128h128v128H896V128zm-256 0h128v128H640V128zm-256 0h128v128H384V128zm-256 0h128v128H128V128zm0 256h128v128H128V384zm0 256h128v128H128V640zm0 256h128v128H128V896zm0 256h128v128H128v-128zm0 256h128v128H128v-128zm0 256h128v128H128v-128zm256 0h128v128H384v-128zm256 0h128v128H640v-128zm256 0h128v128H896v-128zm256 0h128v128h-128v-128z" />
            </svg>
        }
    }
}
