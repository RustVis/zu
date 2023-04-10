// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct SiteScanIcon {}

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

impl Component for SiteScanIcon {
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
                data-icon={ "SiteScanIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M2048 128v1031q-28-28-60-50t-68-40V640H128v1024h1088l-128 128H0V128h2048zm-128 384V256H128v256h1792zm-509 954q0-66 25-123t68-99 100-67 124-24q67 0 125 25t101 68 68 102 25 125q0 65-24 122t-67 101-99 68-123 25q-48 0-94-13t-87-39l-201 201q-23 23-45 46t-46 45q-10 9-19 14t-24 5q-26 0-46-20t-20-46q0-14 5-23t14-20q21-24 44-46t47-45q51-51 100-101t101-101q-25-41-38-86t-14-94zm128 12q0 38 14 71t39 58 58 39 72 14q39 0 74-15t64-43 44-62 17-74q0-36-15-69t-42-60-60-42-70-16q-39 0-74 16t-62 45-43 63-16 75z" />
            </svg>
        }
    }
}
