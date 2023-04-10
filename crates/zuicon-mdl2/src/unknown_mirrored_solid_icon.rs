// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct UnknownMirroredSolidIcon {}

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

impl Component for UnknownMirroredSolidIcon {
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
                data-icon={ "UnknownMirroredSolidIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M960 0Q828 0 705 34t-229 97-194 150-150 194-97 230T0 960q0 132 34 255t97 229 150 194 194 150 230 97 255 35q132 0 255-34t229-97 194-150 150-194 97-229 35-256q0-132-34-255t-97-229-150-194-194-150-229-97T960 0zm-64 1408h128v128H896v-128zm-8-169q0-37-7-70t-36-62q-39-39-77-74t-68-75-49-85-19-105q0-68 26-127t70-104 104-71 128-26q68 0 127 26t104 70 71 104 26 128h-144q0-38-14-71t-40-59-58-39-72-15q-38 0-71 14t-59 40-39 58-15 72q0 41 19 73t47 61 62 60 61 66 48 81 19 107v64H888v-41z" />
            </svg>
        }
    }
}
