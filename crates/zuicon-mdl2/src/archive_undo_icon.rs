// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct ArchiveUndoIcon {}

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

impl Component for ArchiveUndoIcon {
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
                data-icon={ "ArchiveUndoIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1728 1152q67 0 125 25t102 69 68 103 25 126q0 91-34 171t-97 146q-26 26-53 50t-54 48q-45 40-88 79t-88 79l-98-91q46-39 91-78t90-79q28-24 55-48t54-51q47-48 70-104t24-125q0-41-15-76t-42-61-62-40-76-15q-66 0-116 21t-92 58-77 82-69 95h293v128h-512v-512h128v286q46-61 91-113t96-90 115-61 146-22zM0 128h2048v640h-128v256h-128V768H256v1024h1152v128H128V768H0V128zm1920 512V256H128v384h1792zm-896 512H640v-128h384v128z" />
            </svg>
        }
    }
}
