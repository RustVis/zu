// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct CiOutlined {}

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

impl Component for CiOutlined {
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
                class={ props.class.unwrap_or("") }
                width={ props.width.unwrap_or("1em") }
                height={ props.height.unwrap_or("1em") }
                focusable={ "false" }
                data-icon={ "ci" }
                viewBox={ "0 0 1024 1024" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M512 64C264.6 64 64 264.6 64 512s200.6 448 448 448 448-200.6 448-448S759.4 64 512 64zm0 820c-205.4 0-372-166.6-372-372s166.6-372 372-372 372 166.6 372 372-166.6 372-372 372zm218-572.1h-50.4c-4.4 0-8 3.6-8 8v384.2c0 4.4 3.6 8 8 8H730c4.4 0 8-3.6 8-8V319.9c0-4.4-3.6-8-8-8zm-281.4 49.6c49.5 0 83.1 31.5 87 77.6.4 4.2 3.8 7.4 8 7.4h52.6c2.4 0 4.4-2 4.4-4.4 0-81.2-64-138.1-152.3-138.1C345.4 304 286 373.5 286 488.4v49c0 114 59.4 182.6 162.3 182.6 88 0 152.3-55.1 152.3-132.5 0-2.4-2-4.4-4.4-4.4h-52.7c-4.2 0-7.6 3.2-8 7.3-4.2 43-37.7 72.4-87 72.4-61.1 0-95.6-44.9-95.6-125.2v-49.3c.1-81.4 34.6-126.8 95.7-126.8z"/>
            </svg>
        }
    }
}
