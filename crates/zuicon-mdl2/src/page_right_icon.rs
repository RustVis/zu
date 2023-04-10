// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct PageRightIcon {}

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

impl Component for PageRightIcon {
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
                data-icon={ "PageRightIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1029 256q-105 0-202 27t-183 77-154 119-119 155-77 182-28 203q0 105 27 202t77 183 119 154 155 119 182 77 203 28q105 0 202-27t183-77 154-119 119-155 77-182 28-203q0-105-27-202t-77-182-120-155-154-119-182-77-203-28zm0-128q123 0 236 32t213 90 180 139 140 181 90 212 32 237q0 123-32 236t-90 213-139 180-181 140-212 90-237 32q-123 0-236-32t-213-90-180-139-140-181-90-212-32-237q0-123 32-236t90-213 139-180 181-140 212-90 237-32zm-88 1357l-90-90 370-371-370-371 90-90 461 461-461 461z" />
            </svg>
        }
    }
}
