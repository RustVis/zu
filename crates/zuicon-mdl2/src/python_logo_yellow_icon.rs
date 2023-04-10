// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct PythonLogoYellowIcon {}

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

impl Component for PythonLogoYellowIcon {
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
                data-icon={ "PythonLogoYellowIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M2037 1037q0 117-38 229-16 47-34 92t-47 79-67 56-97 21h-732v62h488v187q0 56-24 98t-65 75-91 53-106 35-108 18-96 6q-43 0-95-6t-107-19-105-35-91-53-64-74-24-98v-466q0-50 19-95t53-79 77-54 95-20h488q62 0 117-25t97-68 66-99 25-118V522h183q57 0 97 20t69 55 48 80 31 93q17 66 27 132t11 135zm-740 663q-19 0-36 7t-29 20-20 30-7 36q0 19 7 36t19 30 29 20 37 8q19 0 35-7t29-21 20-30 7-36q0-19-7-35t-19-30-29-20-36-8z" />
            </svg>
        }
    }
}
