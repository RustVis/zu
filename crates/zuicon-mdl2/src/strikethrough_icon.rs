// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct StrikethroughIcon {}

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

impl Component for StrikethroughIcon {
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
                data-icon={ "StrikethroughIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M2048 1152v128h-75q-10 100-45 189t-95 158-144 108-192 40q-224 0-336-197h-4v172h-133v-470H896v470H763v-189h-4q-52 101-141 157t-204 57q-70 0-131-19t-106-58-71-96-26-131q0-51 12-100t43-91H0v-128h332q2 0 16-3t33-7 36-8 23-5l323-47q0-61-11-116t-38-98-73-68-116-25q-99 0-185 36T179 911V766q36-25 80-45t91-33 94-21 92-7q96 0 164 28t112 80 64 125 20 164v95h128V173h133v716h4q60-111 158-170t225-59q109 0 189 39t133 107 80 157 31 189h71zM763 1280H314q-57 29-77 76t-20 107q0 49 17 86t48 61 70 37 86 13q54 0 100-15t85-44 67-68 48-89q16-42 20-80t5-84zm394-128h683q-3-73-23-141t-59-120-99-84-142-32q-82 0-147 29t-112 81-72 120-29 147zm678 128h-678v27q0 72 24 136t69 113 105 76 137 28q82 0 142-32t103-85 66-121 32-142z" />
            </svg>
        }
    }
}
