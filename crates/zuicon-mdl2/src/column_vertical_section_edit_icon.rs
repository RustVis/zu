// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct ColumnVerticalSectionEditIcon {}

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

impl Component for ColumnVerticalSectionEditIcon {
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
                data-icon={ "ColumnVerticalSectionEditIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1848 896q42 0 78 15t64 42 42 63 16 78q0 39-15 76t-43 65l-717 719-377 94 94-377 717-718q28-28 65-42t76-15zm72 198q0-31-20-50t-52-20q-14 0-27 4t-23 15l-692 694-34 135 135-34 692-693q21-21 21-51zm-512-838h640v556q-57-39-128-53V384h-384v532l-128 128V256zM256 384V256h256v128H256zm640-128v128H640V256h256zM256 1792v-128h256v128H256zm384 0v-128h200l-32 128H640zM0 896V640h128v256H0zm0-384V256h128v256H0zm0 1152v-256h128v256H0zm0-384v-256h128v256H0zm1152-512v256h-128V768h128zm0-384v256h-128V384h128zm0 768v128l-128 128v-256h128z" />
            </svg>
        }
    }
}
