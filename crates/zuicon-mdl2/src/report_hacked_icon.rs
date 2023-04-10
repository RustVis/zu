// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct ReportHackedIcon {}

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

impl Component for ReportHackedIcon {
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
                data-icon={ "ReportHackedIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1024 1536H896v-128h128v128zm0-1152H896v896h128V384zm-64 1661q-119-70-236-149t-224-171-198-196-159-223-105-253T0 768V256q83 0 161-6t152-22 146-45 144-75q85-55 170-81T960 0q51 0 97 6t89 20 86 34 85 48q72 46 144 75t146 45 152 22 161 6v512q0 151-38 285t-105 253-158 223-198 195-224 171-237 150zM128 382v386q0 127 33 244t92 222 138 200 172 178 193 155 204 130q102-59 203-130t194-154 172-178 138-200 91-223 34-244V382q-149-8-285-48t-263-121q-68-44-135-64t-149-21q-81 0-148 20t-136 65q-126 81-262 121t-286 48z" />
            </svg>
        }
    }
}
