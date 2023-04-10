// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct ProgressRingDotsIcon {}

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

impl Component for ProgressRingDotsIcon {
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
                data-icon={ "ProgressRingDotsIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1024 96q33 0 62 12t51 35 34 51 13 62q0 33-12 62t-35 51-51 34-62 13q-33 0-62-12t-51-35-34-51-13-62q0-33 12-62t35-51 51-34 62-13zM337 481q0-30 11-56t30-45 46-31 57-12q30 0 56 11t45 31 31 46 12 56q0 30-11 56t-31 46-46 31-56 11q-30 0-56-11t-46-30-31-46-11-57zm-81 415q27 0 50 10t40 27 28 41 10 50q0 27-10 50t-27 40-41 28-50 10q-27 0-50-10t-40-27-28-41-10-50q0-27 10-50t27-40 41-28 50-10zm113 671q0-23 9-43t24-36 35-24 44-9q23 0 43 9t36 24 24 35 9 44q0 23-9 43t-24 36-35 24-44 9q-23 0-43-9t-36-24-24-35-9-44zm655 129q40 0 68 28t28 68q0 40-28 68t-68 28q-40 0-68-28t-28-68q0-40 28-68t68-28zm463-129q0-34 23-57t57-23q34 0 57 23t23 57q0 34-23 57t-57 23q-34 0-57-23t-23-57zm305-607q26 0 45 19t19 45q0 26-19 45t-45 19q-26 0-45-19t-19-45q0-26 19-45t45-19zm-225-655q36 0 68 14t56 38 38 56 14 68q0 36-14 68t-38 56-56 38-68 14q-36 0-68-14t-56-38-38-56-14-68q0-36 14-68t38-56 56-38 68-14z" />
            </svg>
        }
    }
}
