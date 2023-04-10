// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct NumberedListNumberIcon {}

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

impl Component for NumberedListNumberIcon {
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
                data-icon={ "NumberedListNumberIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M135 349q-14 11-38 21t-42 11v-69q30-10 58-23t55-34h49v385h-82V349zm-39 734h160v69H5v-12q0-17 1-34t9-34q12-28 38-52t52-46 46-44 20-46q0-27-14-38t-41-11q-26 0-49 11t-44 28v-73q50-32 111-32 51 0 86 26t36 81q0 46-25 76t-55 53-55 40-25 38zm160 473q0 31-12 53t-33 35-46 20-54 7q-26 0-49-4t-46-15v-73q20 14 41 21t47 7q26 0 46-11t21-41q0-18-9-28t-24-16-32-6-32-2H54v-64h23q14 0 28-1t27-6 20-14 8-27q0-26-16-36t-40-10q-39 0-74 24v-68q22-11 45-15t48-5q22 0 43 5t39 16 29 30 11 43q0 38-19 60t-56 32v1q37 5 61 27t25 61z" />
            </svg>
        }
    }
}
