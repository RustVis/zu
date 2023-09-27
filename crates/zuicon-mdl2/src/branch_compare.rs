// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct BranchCompare {}

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

impl Component for BranchCompare {
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
                data-icon={ "BranchCompare" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1664 1414q56 12 103 41t81 70 53 94 19 109q0 66-25 124t-69 102-102 69-124 25q-66 0-124-25t-102-68-69-102-25-125q0-57 19-109t53-93 81-71 103-41V576q0-40-15-75t-41-61-61-41-75-15h-166l163 163-90 90-318-317L1251 3l90 90-163 163h166q66 0 124 25t101 69 69 102 26 124v838zm-64 506q40 0 75-15t61-41 41-61 15-75q0-40-15-75t-41-61-61-41-75-15q-40 0-75 15t-61 41-41 61-15 75q0 40 15 75t41 61 61 41 75 15zm-803-509l318 317-318 317-90-90 163-163H704q-66 0-124-25t-102-68-69-102-25-125V634q-56-12-103-41t-81-70-53-94-19-109q0-66 25-124t68-101 102-69T448 0q66 0 124 25t101 69 69 102 26 124q0 57-19 109t-53 93-81 71-103 41v838q0 40 15 75t41 61 61 41 75 15h166l-163-163 90-90zM256 320q0 40 15 75t41 61 61 41 75 15q40 0 75-15t61-41 41-61 15-75q0-40-15-75t-41-61-61-41-75-15q-40 0-75 15t-61 41-41 61-15 75z" />
            </svg>
        }
    }
}
