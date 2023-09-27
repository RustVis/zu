// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct DriverOff {}

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

impl Component for DriverOff {
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
                data-icon={ "DriverOff" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1152 224q-27 0-50 10t-40 27-28 41-10 50v160H640v384H416q-27 0-50 10t-40 27-28 41-10 50q0 27 10 50t27 40 41 28 50 10h224v384h358l-4 32q-2 16-2 32 0 32 4 63t11 62q1 4 4 13t6 19 5 18 2 10v195q-59-34-93-92t-35-128v-96H512v-384h-96q-53 0-99-20t-82-55-55-81-20-100q0-53 20-99t55-82 81-55 100-20h96V384h384v-32q0-53 20-99t55-82 81-55 100-20q53 0 99 20t82 55 55 81 20 100v32h384v512h-160q-46 0-80 30t-43 75q-34 5-67 13t-65 22v-6q0-3-1-6 0-53 20-99t55-82 81-55 100-20h32V512h-384V352q0-27-10-50t-27-40-41-28-50-10zm896 1376q0 93-35 174t-96 143-142 96-175 35q-93 0-174-35t-143-96-96-142-35-175q0-93 35-174t96-143 142-96 175-35q93 0 174 35t143 96 96 142 35 175zm-768 0q0 66 25 124t68 102 102 69 125 25q47 0 92-13t84-40l-443-443q-26 39-39 84t-14 92zm587 176q26-39 39-84t14-92q0-66-25-124t-69-101-102-69-124-26q-47 0-92 13t-84 40l443 443z" />
            </svg>
        }
    }
}
