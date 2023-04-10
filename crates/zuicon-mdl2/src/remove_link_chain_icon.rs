// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct RemoveLinkChainIcon {}

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

impl Component for RemoveLinkChainIcon {
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
                data-icon={ "RemoveLinkChainIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1264 1536l128 128h-304q-93 0-174-35t-142-96-96-142-36-175q0-67 19-130t55-117 86-96 113-70q30-13 57-20t55-10 55-4 58-1h14v128h-44q-38 0-70 4t-74 22q-45 20-81 49t-61 68-40 83-14 94q0 66 25 124t68 102 102 69 125 25h176zm784-320q0 46-11 96l-119-119q-3-45-19-87t-41-78-61-64-78-45q-40-16-74-19t-72-4h-37V768h64q93 0 174 35t142 96 96 142 36 175zm-641-384q0 68-19 131t-55 116-86 96-113 70q-29 13-56 20t-55 10-55 4-58 1h-14v-128h43q38 0 71-4t74-21q44-19 80-49t62-69 40-84 14-93q0-66-25-124t-69-101-102-69-124-26H448q-66 0-124 25t-102 69-69 102-25 124q0 49 14 94t39 83 62 68 81 50q39 17 72 21t72 4h44v128h-64q-93 0-174-35t-142-96-96-142T0 832q0-93 35-174t96-142 142-96 175-36h512q93 0 174 35t142 96 96 142 35 175z" />
            </svg>
        }
    }
}
