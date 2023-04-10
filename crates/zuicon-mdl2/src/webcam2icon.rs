// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Webcam2icon {}

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

impl Component for Webcam2icon {
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
                data-icon={ "Webcam2Icon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1920 896q0 148-46 285t-131 250-203 198-260 126v165h384v128H384v-128h384v-165q-143-42-260-126t-202-197-131-251-47-285q0-124 32-238t90-214 140-181 181-140 214-91 239-32q124 0 238 32t214 90 181 140 140 181 91 214 32 239zm-896-768q-106 0-204 27t-183 78-156 120-120 155-77 184-28 204h320q0-93 35-174t96-143 142-96 175-35q93 0 174 35t143 96 96 142 35 175h320q0-106-27-204t-78-183-120-156-155-120-184-77-204-28zm320 768q0-66-25-124t-68-102-102-69-125-25q-66 0-124 25t-102 68-69 102-25 125q0 66 25 124t68 102 102 69 125 25q66 0 124-25t102-68 69-102 25-125zM267 1024q15 92 51 175t90 155 120 128 147 98 167 62 182 22q93 0 181-22t168-62 146-97 121-129 89-154 52-176h-328q-20 71-62 130t-99 101-126 65-142 24q-74 0-142-23t-126-66-99-101-62-130H267zm885 896v-137q-63 9-128 9t-128-9v137h256z" />
            </svg>
        }
    }
}
