// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Vbicon {}

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

impl Component for Vbicon {
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
                data-icon={ "VBIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1920 128v1792H128V128h1792zm-119 119H247v1554h1554V247zM639 1664L368 896h140l189 583q5 16 8 32t8 33q2-17 6-33t10-33l194-582h135l-276 768H639zm1025-221q0 56-23 97t-62 69-86 41-99 14h-242V896h240q41 0 83 9t77 30 56 54 22 83q0 67-37 114t-100 70q78 9 124 58t47 129zm-387-226h81q29 0 54-7t45-21 30-37 11-55q0-29-11-48t-29-30-41-16-49-5h-91v219zm107 345q30 0 56-7t47-21 31-38 12-57q0-36-14-59t-36-37-52-19-60-5h-91v243h107z" />
            </svg>
        }
    }
}
