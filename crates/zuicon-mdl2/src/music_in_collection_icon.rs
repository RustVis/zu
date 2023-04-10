// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct MusicInCollectionIcon {}

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

impl Component for MusicInCollectionIcon {
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
                data-icon={ "MusicInCollectionIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1792 1408q0 62-29 109t-76 80-104 50-111 17q-54 0-111-17t-103-49-76-80-30-110q0-61 29-109t76-80 104-50 111-17q51 0 100 12t92 39V226L768 450v1214q0 62-29 109t-76 80-104 50-111 17q-54 0-111-17t-103-49-76-80-30-110q0-61 29-109t76-80 104-50 111-17q51 0 100 12t92 39V350L1792 62v1346zM448 1792q27 0 60-8t62-23 50-40 20-57q0-33-20-57t-49-39-63-24-60-8q-27 0-60 8t-62 23-50 40-20 57q0 33 20 57t49 39 63 24 60 8zm1024-256q27 0 60-8t62-23 50-40 20-57q0-33-20-57t-49-39-63-24-60-8q-27 0-60 8t-62 23-50 40-20 57q0 33 20 57t49 39 63 24 60 8z" />
            </svg>
        }
    }
}
