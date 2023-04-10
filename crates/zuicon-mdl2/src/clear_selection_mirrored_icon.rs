// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct ClearSelectionMirroredIcon {}

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

impl Component for ClearSelectionMirroredIcon {
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
                data-icon={ "ClearSelectionMirroredIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M463 896q-69 0-130 26t-106 72-72 107-27 131q0 66 25 127t73 110l449 448 90-90-448-449q-29-29-45-67t-16-79q0-43 16-80t45-66 66-45 81-17q41 0 79 16t67 45l195 195H640v128h384v-384H896v165q-47-47-93-99t-97-95-111-71-132-28zm-79-128h128V640H384v128zm0-256h128V384H384v128zm0-256h128V128H384v128zm256 0h128V128H640v128zm256 0h128V128H896v128zm384-128h-128v128h128V128zm256 0h-128v128h128V128zm256 0h-128v128h128V128zm256 0h-128v128h128V128zm0 256h-128v128h128V384zm0 256h-128v128h128V640zm0 256h-128v128h128V896zm0 256h-128v128h128v-128zm0 256h-128v128h128v-128zm0 256h-128v128h128v-128zm-256 0h-128v128h128v-128zm-256 0h-128v128h128v-128zm-256 0h-128v128h128v-128zm-256 0H896v128h128v-128z" />
            </svg>
        }
    }
}
