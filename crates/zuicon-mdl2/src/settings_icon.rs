// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct SettingsIcon {}

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

impl Component for SettingsIcon {
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
                data-icon={ "SettingsIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1783 988v18q0 9 1 18v18q0 9-1 18l259 161-159 383-297-68q-24 26-50 50l68 297-383 159-161-259h-18q-9 0-18 1h-18q-9 0-18-1l-161 259-383-159 68-297q-26-24-50-50l-297 68L6 1221l259-161v-18q0-9-1-18v-18q0-9 1-18L6 827l159-383 297 68q24-26 50-50l-68-297L827 6l161 259h18q9 0 18-1h18q9 0 18 1L1221 6l383 159-68 297q26 24 50 50l297-68 159 383-259 161zm-117 130q2-24 4-47t2-48q0-23-2-47t-4-47l236-147-86-208-271 63q-31-38-63-70t-71-64l63-271-208-86-148 236q-23-2-47-4t-47-2q-24 0-47 2t-48 4L782 146l-208 86 63 271q-38 31-70 63t-64 71l-271-63-86 208 236 148q-2 24-4 47t-2 48q0 23 2 47t4 47l-236 147 86 208 271-63q31 38 63 70t71 64l-63 271 208 86 148-236q23 2 47 4t47 2q24 0 47-2t48-4l147 236 208-86-63-271q38-31 70-63t64-71l271 63 86-208-236-148zm-642-470q78 0 146 29t120 81 80 119 30 147q0 78-29 146t-81 120-119 80-147 30q-78 0-146-29t-120-81-80-119-30-147q0-78 29-146t81-120 119-80 147-30zm0 640q55 0 103-20t84-57 56-84 21-103q0-55-20-103t-57-84-84-56-103-21q-55 0-103 20t-84 57-56 84-21 103q0 55 20 103t57 84 84 56 103 21z" />
            </svg>
        }
    }
}
