// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct OfflineStorageIcon {}

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

impl Component for OfflineStorageIcon {
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
                data-icon={ "OfflineStorageIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M960 128q70 0 158 8t176 28 168 53 132 85q31 30 50 65t20 81v1152q0 45-19 80t-51 66q-52 51-131 84t-168 54-177 28-158 8q-70 0-158-8t-176-28-168-53-132-85q-31-30-50-65t-20-81V448q0-45 19-80t51-66q52-51 131-84t168-54 177-28 158-8zm0 128q-51 0-106 3t-111 12-110 22-102 33q-15 6-40 18t-49 29-41 35-17 40q0 8 3 15t8 14q21 32 60 56t89 42 106 30 112 20 107 11 91 4q40 0 91-3t107-11 112-20 105-31 89-42 61-56q5-7 8-14t3-15q0-20-17-39t-41-36-49-28-40-19q-48-19-102-32t-109-22-111-12-107-4zm0 1536q51 0 106-3t112-12 110-22 101-33q15-6 40-18t49-29 41-35 17-40v-194q-57 38-129 63t-149 40-154 21-144 6q-67 0-144-6t-154-21-149-40-129-63v194q0 21 17 40t41 35 49 28 40 19q47 20 101 33t110 21 111 12 107 4zm0-384q51 0 106-3t112-12 110-22 101-33q15-6 40-18t49-29 41-35 17-40v-194q-57 38-129 63t-149 40-154 21-144 6q-67 0-144-6t-154-21-149-40-129-63v194q0 21 17 40t41 35 49 28 40 19q47 20 101 33t110 21 111 12 107 4zm0-384q52 0 107-3t110-12 110-22 102-33q15-6 40-18t49-29 41-35 17-40V638q-57 37-128 62t-149 40-155 21-144 7q-67 0-144-6t-154-22-149-40-129-62v194q0 20 17 39t41 36 49 28 40 19q48 20 102 33t109 21 111 12 107 4z" />
            </svg>
        }
    }
}
