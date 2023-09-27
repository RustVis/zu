// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct NonprofitLogo32 {}

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

impl Component for NonprofitLogo32 {
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
                data-icon={ "NonprofitLogo32" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M925 1280q0 34-24 58l-193 193q-24 24-59 24t-59-23-24-59q0-35 24-59l193-193q24-24 59-24t59 24 24 59zm-205-104q0 35-24 59l-143 142q-24 24-58 24-35 0-59-23t-24-59q0-35 24-59l142-143q11-11 27-17t32-7q35 0 59 24t24 59zm345 273q0 33-25 58l-178 178q-24 24-59 24t-59-23-24-59q0-35 24-59l179-178q25-25 58-25 34 0 59 24t25 60zm-108 415q-35 0-59-24t-24-59q0-35 24-59l135-134q25-25 59-25 35 0 59 25t24 60q0 16-7 31t-18 26l-134 135q-24 24-59 24zm963-1153q0 88-23 154t-62 123-90 108-108 110l-590-593-297 295q-30 28-70 28-41 0-69-28t-29-70q0-41 29-70l428-426q74-73 168-111t198-39q107 0 201 41t163 112 110 165 41 201zm-272 604q0 35-24 59t-59 24q-34 0-58-24l-307-308q-7-7-19-7-11 0-19 7t-8 19q0 10 8 18l336 336q25 25 25 59 0 35-24 59t-60 24q-34 0-58-24l-336-336q-7-7-19-7-11 0-18 7t-8 19q0 10 8 18l336 336q24 24 24 59t-24 59-59 24q-26 0-42-11t-33-30q10-21 10-49 0-27-10-50t-28-41-42-27-51-10q20-31 20-69 0-27-10-50t-27-41-41-28-49-10h-9q-4 0-10 2 3-11 5-21t2-22q0-26-10-49t-27-41-41-28-50-10q-21 0-40 7t-37 19q0-27-10-50t-27-41-40-28-51-10q-25 0-49 9t-42 28l-127 128q-60-61-113-113t-92-110-63-126-23-160q0-106 41-199t112-162 165-110 199-41q97 0 186 34t161 100L579 736q-20 20-31 46t-12 56q0 30 11 56t31 45 46 31 56 12q29 0 55-11t47-31l265-264 577 580q24 24 24 59zm-583 539l111-111q8 18 8 36 0 35-24 59t-59 24q-18 0-36-8z" />
            </svg>
        }
    }
}
