// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct RedEyeIcon {}

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

impl Component for RedEyeIcon {
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
                data-icon={ "RedEyeIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1024 512q150 0 292 39t272 107 246 163 214 203q-98 110-213 203t-246 162-273 108-292 39q-150 0-292-39t-272-107-247-162T0 1024q97-109 213-203t246-162 273-108 292-39zm384 512q0-79-30-149t-83-122-122-82-149-31q-79 0-149 30t-122 83-82 122-31 149q0 79 30 149t83 122 122 82 149 31q79 0 149-30t122-83 82-122 31-149zm-1231 0q94 91 200 166t227 127q-45-64-68-139t-24-154q0-78 23-153t69-140q-121 51-227 126t-200 167zm1267 293q121-51 227-126t200-167q-94-91-200-166t-227-127q45 64 68 139t24 154q0 78-23 153t-69 140zm-420-165q-27 0-50-10t-40-27-28-41-10-50q0-27 10-50t27-40 41-28 50-10q27 0 50 10t40 27 28 41 10 50q0 27-10 50t-27 40-41 28-50 10z" />
            </svg>
        }
    }
}
