// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct AlarmClockIcon {}

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

impl Component for AlarmClockIcon {
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
                data-icon={ "AlarmClockIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1408 1152v128H896V640h128v512h384zm240-1024q83 0 156 31t127 86 85 127 32 156q0 50-12 97t-35 91-56 80-74 64q25 71 37 144t12 148q0 162-56 313t-163 274l200 200-90 90-200-200q-122 106-273 162t-314 57q-162 0-313-56t-274-163l-200 200-90-90 200-200q-106-122-162-273t-57-314q0-75 12-148t37-144q-41-27-74-64t-56-80-35-90-12-98q0-83 31-156t86-127 127-85 156-32q49 0 97 12t91 35 80 56 64 74q71-25 144-37t148-12q75 0 148 12t144 37q27-41 64-74t80-56 90-35 98-12zM400 256q-56 0-105 21t-87 59-58 86-22 106q0 62 26 117t75 94q64-124 161-221t221-161q-38-48-93-74t-118-27zm624 1664q159 0 298-60t244-165 165-244 61-299q0-159-60-298t-165-244-244-165-299-61q-159 0-298 60T482 609 317 853t-61 299q0 159 60 298t165 244 244 165 299 61zm795-1181q48-38 74-93t27-118q0-56-21-105t-59-87-86-58-106-22q-62 0-117 26t-94 75q124 64 221 161t161 221z" />
            </svg>
        }
    }
}
