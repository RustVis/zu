// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct RepeatOneIcon {}

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

impl Component for RepeatOneIcon {
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
                data-icon={ "RepeatOneIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M128 1024q0 69 15 136t44 130 70 118 95 100l-91 91q-62-55-110-120t-82-139-51-153-18-163q0-106 27-204t78-183 120-156 155-120 184-77 204-28h646L1251 93l90-90 317 317-317 317-90-90 163-163H768q-88 0-170 23t-153 64-129 100-100 130-65 153-23 170zm1659-574q62 54 110 119t82 139 51 154 18 162q0 106-27 204t-78 183-120 156-155 120-184 77-204 28H634l163 163-90 90-317-317 317-317 90 90-163 163h646q88 0 170-23t153-64 129-100 100-130 65-153 23-170q0-70-15-137t-43-130-70-117-95-101l90-89zm-635 958h-128V820q-29 21-61 34t-67 23V746q56-19 106-45t99-61h51v768z" />
            </svg>
        }
    }
}
