// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct CollegeFootballIcon {}

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

impl Component for CollegeFootballIcon {
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
                data-icon={ "CollegeFootballIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1536 0q106 0 199 40t163 109 110 163 40 200q0 212-55 408t-155 367-240 311-311 240-367 155-408 55q-106 0-199-40t-163-109-110-163-40-200q0-212 55-408t155-367 240-311 311-240 367-155 408-55zm384 512q0-79-30-149t-82-122-123-83-149-30q-68 0-135 6t-133 20l626 626q13-66 19-133t7-135zM128 1536q0 80 30 149t82 122 122 83 150 30q68 0 135-6t133-20l-626-626q-13 66-19 133t-7 135zm797 322q167-51 314-140t268-210 210-268 141-315l-735-735q-167 51-314 140T541 540 331 808t-141 315l735 735zm310-1135l90 90-512 512-90-90 512-512z" />
            </svg>
        }
    }
}
