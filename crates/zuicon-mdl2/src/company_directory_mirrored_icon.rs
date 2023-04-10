// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct CompanyDirectoryMirroredIcon {}

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

impl Component for CompanyDirectoryMirroredIcon {
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
                data-icon={ "CompanyDirectoryMirroredIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M128 0v2048h1664v-254h128v-128h-128v-257h128v-128h-128V769h128V641h-128V385h128V257h-128V0H128zm128 128h1408v1792H256V128zm128 384h640V384H384v128zm0 256h640V640H384v128zm960 892q39 0 73-14t60-40 40-60 15-74q0-39-14-73t-40-59-60-41-74-15q-39 0-73 15t-59 40-41 60-15 73q0 39 15 73t40 60 60 40 73 15zm0-256q29 0 48 19t20 49q0 29-19 48t-49 20q-29 0-48-19t-20-49q0-29 19-48t49-20zm0-640q39 0 73-14t60-40 40-60 15-74q0-39-14-73t-40-59-60-41-74-15q-39 0-73 15t-59 40-41 60-15 73q0 39 15 73t40 60 60 40 73 15zm0-256q29 0 48 19t20 49q0 29-19 48t-49 20q-29 0-48-19t-20-49q0-29 19-48t49-20zm-960 900h640v-128H384v128zm0 256h640v-128H384v128z" />
            </svg>
        }
    }
}
