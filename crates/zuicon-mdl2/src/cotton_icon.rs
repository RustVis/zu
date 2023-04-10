// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct CottonIcon {}

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

impl Component for CottonIcon {
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
                data-icon={ "CottonIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1504 393q91 18 167 64t131 112 87 150 31 177q0 101-37 191t-103 160-153 112-189 48q-71 106-178 173t-236 81v387H896v-387q-128-14-235-81t-179-173q-101-5-189-47t-153-112-102-160T0 896q0-93 31-176t86-150 132-113 167-64q29-88 83-160t125-124 157-80T960 0q93 0 178 28t157 81 126 124 83 160zm-96 887q79 0 149-30t122-82 83-122 30-150q0-79-30-149t-82-122-123-83-149-30h-3q-1 0-3 1-12-82-51-152t-98-123-134-81-159-29q-84 0-159 29t-134 81-98 122-51 153h-3q-1 0-3-1-80 1-150 31t-122 81-82 122-30 150q0 80 30 149t82 122 122 83 150 30h45q24 51 59 93t79 75 94 54 107 29v-129q-56-12-103-41t-81-70-53-94-19-109h128q0 30 9 58t26 53 40 42 53 28V896h128v373q29-10 52-28t41-42 26-52 9-59h128q0 57-19 109t-53 93-81 71-103 41v129q55-8 106-29t95-53 79-75 59-94h45z" />
            </svg>
        }
    }
}
