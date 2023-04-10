// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct WheelchairIcon {}

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

impl Component for WheelchairIcon {
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
                data-icon={ "WheelchairIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1728 896q40 0 75 15t61 41 41 60 15 75q0 22-4 39l-128 640q-7 33-24 61t-43 49-56 32-65 12q-39 0-74-15t-61-41-42-62-15-74q0-10 1-19t3-19l82-410h-99q13 61 13 128 0 133-50 249t-137 204-203 137-250 50q-133 0-249-50t-204-137-137-203-50-250q0-115 39-220t108-188 164-141 209-80l246-315-38-25-144 145q-27 27-62 41t-74 15q-40 0-75-15t-61-41-41-61-15-75q0-38 14-73t42-63L696 56q27-27 62-41t74-15q58 0 107 32 87 56 171 114t170 115q0-54 19-101t54-83 82-56 101-21q53 0 99 20t82 55 55 81 20 100q0 53-20 99t-55 82-81 55-100 20h-13q-6 0-14-1 11 42 11 82 0 49-13 89t-34 75-48 68-56 71h359zm-192-768q-27 0-50 10t-40 27-28 41-10 50q0 27 10 50t27 40 41 28 50 10q27 0 50-10t40-27 28-41 10-50q0-27-10-50t-27-40-41-28-50-10zm-456 306L818 770q102 8 196 47t171 106l166-213q20-26 30-56t11-62q0-45-19-83t-55-68q-17-14-37-26t-38-25L942 189l-36-26q-18-13-38-24-10-5-17-8t-19-3q-26 0-45 19L531 403q-19 19-19 45t19 45 45 19q26 0 45-19l219-219 240 160zM768 1920q106 0 199-40t162-110 110-163 41-199q0-106-40-199t-110-162-163-110-199-41q-106 0-199 40t-162 110-110 163-41 199q0 106 40 199t110 162 163 110 199 41zm832-128q23 0 40-14t23-37l128-640q1-4 1-12 0-27-18-46t-46-19h-448q44 59 75 128h295l-113 563q-1 4-1 13 0 26 18 45t46 19zm-832-768q79 0 149 30t122 82 83 123 30 149q0 80-30 149t-82 122-123 83-149 30q-80 0-149-30t-122-82-83-122-30-150q0-79 30-149t82-122 122-83 150-30zm0 640q53 0 99-20t82-55 55-81 20-100q0-53-20-99t-55-82-81-55-100-20q-53 0-99 20t-82 55-55 81-20 100q0 53 20 99t55 82 81 55 100 20z" />
            </svg>
        }
    }
}
