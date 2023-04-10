// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct CloudWeatherIcon {}

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

impl Component for CloudWeatherIcon {
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
                data-icon={ "CloudWeatherIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1661 896q80 0 150 30t123 81 83 122 31 151q0 80-30 149t-82 122-123 83-149 30H512q-106 0-199-40t-163-109-110-163-40-200q0-106 40-199t109-163 163-110 200-40q46 0 93 9 40-62 92-111t115-84 132-52 144-18q111 0 209 39t175 107 125 163 64 203zm3 640q53 0 99-20t82-55 55-81 20-100q0-53-20-99t-55-82-81-55-100-20h-128v-64q0-93-35-174t-96-143-142-96-175-35q-70 0-135 21t-119 59-97 91-67 120q-75-35-158-35-80 0-149 30t-122 82-83 123-30 149q0 80 30 149t82 122 122 83 150 30h1152z" />
            </svg>
        }
    }
}
