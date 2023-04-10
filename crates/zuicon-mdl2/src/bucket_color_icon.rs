// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct BucketColorIcon {}

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

impl Component for BucketColorIcon {
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
                data-icon={ "BucketColorIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1690 960l-858 858-730-730 666-667V192q0-40 15-75t41-61 61-41 75-15q40 0 75 15t61 41 41 61 15 75v640h-128V192q0-26-19-45t-45-19q-26 0-45 19t-19 45v283l-549 549h1098l65-64-211-211 90-90 301 301zm-858 678l485-486H347l485 486zm1147-48q20 35 30 74t10 80q0 61-22 116t-61 97-92 66-116 25q-62 0-116-24t-94-64-63-95-24-116q0-79 40-148l257-450 251 439zm-251 330q36 0 66-14t52-39 34-57 12-67q0-49-24-90l-140-244-146 256q-23 40-23 84 0 35 13 66t37 54 53 37 66 14z" />
            </svg>
        }
    }
}
