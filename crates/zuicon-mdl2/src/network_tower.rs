// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct NetworkTower {}

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

impl Component for NetworkTower {
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
                data-icon={ "NetworkTower" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1188 794q45-43 69-99t25-119q0-63-24-119t-70-99l89-92q64 62 98 141t35 169q0 89-34 168t-99 142l-89-92zm267-712q50 48 89 105t67 120 41 131 14 138q0 70-14 138t-41 131-66 120-90 105l-89-92q83-80 127-183t45-219q0-115-44-218t-128-184l89-92zM643 886q-64-62-98-141t-35-169q0-89 34-168t99-142l89 92q-45 43-69 99t-25 119q0 63 24 119t70 99l-89 92zm-178 184q-50-48-89-105t-67-120-41-131-14-138q0-70 14-138t41-131 66-120 90-105l89 92q-83 80-127 183t-45 219q0 115 44 218t128 184l-89 92zm687-494q0 45-19 84t-55 67l397 1193h-134l-86-256H665l-86 256H445L842 727q-35-27-54-66t-20-85q0-40 15-75t41-61 61-41 75-15q40 0 75 15t61 41 41 61 15 75zm-192-64q-26 0-45 19t-19 45q0 26 19 45t45 19q26 0 45-19t19-45q0-26-19-45t-45-19zm0 266l-125 374h250L960 778zm253 758l-86-256H793l-86 256h506z" />
            </svg>
        }
    }
}
