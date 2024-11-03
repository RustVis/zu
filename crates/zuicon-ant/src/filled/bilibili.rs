// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Bilibili {}

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

impl Component for Bilibili {
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
                data-icon={ "bilibili" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <g><path d="M198.134 484.45c-7.999-4.463-16.498-8.43-24.997-11.9a273.55 273.55 0 0 0-26.996-7.438c-2.5-.992-2.5.991-2.5 1.487 0 7.934.5 18.843 1.5 27.768 1 7.438 2 15.372 4 22.81 0 .496 0 .991.5 1.487.999.992 1.999 1.488 2.999.496 7.999-4.463 15.998-8.43 22.997-13.388 7.499-5.454 15.498-11.9 21.997-18.347 1.5-1.487 0-2.479.5-2.975m323.96-11.9a273.55 273.55 0 0 0-26.997-7.438c-2.5-.992-2.5.991-2.5 1.487 0 7.934.5 18.843 1.5 27.768 1 7.438 2 15.372 4 22.81 0 .496 0 .991.5 1.487 1 .992 2 1.488 3 .496 7.999-4.463 15.998-8.43 22.997-13.388 7.499-5.454 15.498-11.9 21.997-18.347 2-1.487.5-2.479.5-2.975-7.5-4.463-16.498-8.43-24.997-11.9" transform="translate(112 112)"/><path d="M629.496 0H171C76.499 0 0 76.5 0 171.5v458C.5 723.5 77 800 170.999 800h457.997c94.5 0 171.002-76.5 171.002-170.5v-458C800.495 76.5 723.996 0 629.496 0m95 343.5h15.5v48h-15.5zm-95.5-1.5 2 43-13.5 1.5-5-44.5zm-23.5 0 4 45.5-14.5 1.5-6.5-47.5h17zm-230.498 1.5h15v48h-15zm-96-1.5 2 43-13.5 1.5-5-44.5zm-23.5 0 4 45.5-14.5 2-6-47.5zm-3.5 149C231.498 556.5 104 550.5 92.5 548.5 83.5 387 69.5 352 58 273l54.5-22.5c1 71.5 9 185 9 185s108.5-15.5 132 47c.5 3 0 6-1.5 8.5m20.5 35.5-23.5-124h35.5l13 123zm44.5-8-27-235 33.5-1.5 21 236h-27.5zm34-175h17.5v48h-13.5zm41 190h-26.5l-9.5-126h36zm209.998-43C581.496 556 453.997 550 442.497 548c-9-161-23-196-34.5-275l54.5-22.5c1 71.5 9 185 9 185s108.5-15.5 132 46.5c.5 3 0 6-1.5 8.5m19.5 36-23-124h35.5l13 123zm45.5-8-27.5-235 33.5-1.5 21 236h-27zm33.5-175h17.5v48h-13zm41 190h-26.5l-9.5-126h36z" transform="translate(112 112)"/></g>
            </svg>
        }
    }
}
