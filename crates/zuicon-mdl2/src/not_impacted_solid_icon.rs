// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct NotImpactedSolidIcon {}

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

impl Component for NotImpactedSolidIcon {
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
                data-icon={ "NotImpactedSolidIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1600 992q-102 0-192 32t-167 88l-189-124-660 1001q-20 29-54 29-18 0-35-11-28-19-28-53 0-19 10-35L946 917 619 702l-20 31q-19 28-47 42t-61 15q-38 0-69-21-28-19-43-47t-15-61q0-37 21-69L738 57q19-28 47-42t60-15q37 0 70 21 28 19 43 47t15 61q0 38-21 69l-21 31 748 493 20-30q19-28 47-43t61-15q37 0 70 21 28 19 42 47t15 61q0 37-21 70l-126 191q-45-14-91-23t-96-9zm317 291q63 64 97 145t34 172q0 91-34 172t-97 144q-64 64-145 98t-172 34q-91 0-172-34t-144-97q-64-64-98-145t-34-172q0-91 34-172t97-144q64-64 145-98t172-34q91 0 172 34t145 97zm-637 317q0 66 25 124t68 102 102 69 125 25q49 0 93-14t83-39l-443-443q-25 38-39 82t-14 94zm587 176q25-38 39-82t14-94q0-66-25-124t-69-101-102-69-124-26q-49 0-93 14t-83 39l443 443z" />
            </svg>
        }
    }
}
