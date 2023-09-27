// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct ClosedCaption {}

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

impl Component for ClosedCaption {
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
                data-icon={ "ClosedCaption" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1920 128v1408h-768v512l-384-512H0V128h1920zM960 1341q30 0 80-1t111-4 124-9 120-16 101-24 64-35q17-17 30-48t22-69 17-82 11-85 6-76 2-60q0-23-2-59t-6-77-11-85-17-83-23-71-29-48q-19-19-62-32t-101-23-122-16-125-9-110-5-80-1q-29 0-79 1t-111 4-125 10-122 15-100 23-63 33q-17 17-30 48t-22 70-17 82-11 85-6 77-2 60q0 24 2 59t6 77 10 85 17 82 23 70 30 48q15 15 43 26t61 20 63 13 52 8q95 12 190 17t191 5z" />
            </svg>
        }
    }
}
