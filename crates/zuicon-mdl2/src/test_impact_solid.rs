// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct TestImpactSolid {}

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

impl Component for TestImpactSolid {
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
                data-icon={ "TestImpactSolid" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M798 1850q0 39 16 75t43 63 63 44 75 16H354q-40 0-75-15t-61-41-42-61-15-75q0-27 7-51t21-48l569-990q10-18 10-35V128H640V0h768v128h-128v385h-11q-33 0-60 6t-57 27V128H896v604q0 52-28 100l-330 576h486l-199 348q-13 23-20 45t-7 49zm1061-91q14 23 21 46t7 51q0 40-15 75t-41 61-62 41-74 15H995q49 0 94-25t71-68l290-501q6 34 25 62t46 48 59 32 66 11q58 0 108-32l105 184zm-6-779q28 0 47 21t20 49q0 14-8 28l-212 370q-18 31-54 31-29 0-51-19t-22-49q0-18 7-30t18-26l-179-103-370 639q-8 14-23 21t-32 8q-14 0-26-6t-21-16-15-23-6-26q0-14 8-27l366-638-175-102q-8 12-14 22t-12 17-17 12-25 5q-29 0-52-18t-23-49q0-7 1-16t6-15l217-370q8-14 23-22t32-8q28 0 50 21t22 49q0 17-8 28t-17 25l473 272q15-22 28-38t44-17z" />
            </svg>
        }
    }
}
