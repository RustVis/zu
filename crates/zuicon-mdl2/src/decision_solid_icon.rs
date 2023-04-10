// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct DecisionSolidIcon {}

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

impl Component for DecisionSolidIcon {
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
                data-icon={ "DecisionSolidIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M490 1343q-25 0-48-10t-42-28L37 942Q0 905 0 852q0-27 10-50t27-40 41-27 50-10q26 0 49 9t42 28l362 362q18 18 27 41t10 50q0 26-10 49t-27 41-41 28-50 10zm724-724q-25 0-48-10t-42-28L762 219q-38-38-38-91 0-27 10-50t27-40 41-28 50-10q53 0 90 37l363 363q18 18 27 41t10 50q0 26-10 49t-27 41-41 28-50 10zm-520 391L332 648l316-316 362 362-316 316zm1244 838q19 19 19 45t-19 45-45 19q-26 0-45-19L897 988l91-91 950 951zM0 1920h1024v128H0v-128zm128-256h768v128H128v-128z" />
            </svg>
        }
    }
}
