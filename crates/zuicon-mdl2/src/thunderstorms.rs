// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Thunderstorms {}

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

impl Component for Thunderstorms {
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
                data-icon={ "Thunderstorms" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1761 1036q63 17 115 52t91 85 60 109 21 126q0 80-30 149t-82 122-123 83-149 30h-384l128-128h256q53 0 99-20t82-55 55-81 20-100q0-55-20-101t-57-81-83-54-102-20q-12-82-51-152t-98-122-134-81-159-29q-77 0-146 25t-127 69-98 106-61 135q-44-38-97-58t-111-21q-66 0-124 25t-102 68-69 102-25 125q0 66 25 124t68 102 102 69 125 25h256l-64 128H576q-93 0-174-35t-142-96-96-142-36-175q0-75 24-144-71-54-111-134T0 896q0-79 30-149t82-122 122-83 150-30h21q10 0 22 2 27-60 69-107t95-81 114-52 127-18q84 0 159 29t135 81 99 123 51 154q84 9 160 41t139 84 112 120 74 148zM723 921q34-57 79-104t100-83 116-59 128-31q-11-56-40-104t-70-82-94-54-110-20q-62 0-117 22t-98 62-70 93-32 116q-30-17-63-27t-68-10q-53 0-99 20t-82 55-55 81-20 100q0 54 21 103t63 85q63-89 159-138t205-50q74 0 147 25zm109 1127l256-512H864l192-384h224l-128 256h320l-640 640z" />
            </svg>
        }
    }
}