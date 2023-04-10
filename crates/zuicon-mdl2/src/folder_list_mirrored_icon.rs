// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct FolderListMirroredIcon {}

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

impl Component for FolderListMirroredIcon {
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
                data-icon={ "FolderListMirroredIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M608 128q45 0 77 9t58 24 46 31 40 31 44 23 55 10h992q27 0 50 10t40 27 28 41 10 50v896h-128V384H928q-31 0-54 9t-44 24-41 31-45 31-58 23-78 10H128v1152h896v128H0V256q0-27 10-50t27-40 41-28 50-10h480zm0 256q24 0 42-4t33-13 29-20 32-27q-17-15-31-26t-30-20-33-13-42-5H128v128h480zm544 1536h640v128h-640v-128zm768 0h128v128h-128v-128zm0-256h128v128h-128v-128zm-768 0h640v128h-640v-128zm768-256h128v128h-128v-128zm-768 0h640v128h-640v-128z" />
            </svg>
        }
    }
}
