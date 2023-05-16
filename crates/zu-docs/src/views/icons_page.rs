// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{function_component, html, Html};
use zu::svg_icon::{Color, FontSize, SvgIcon};

use super::abc::Abc;

#[function_component(IconsPage)]
pub fn icons_page() -> Html {
    let path = html! {
        <path d="M10 20v-6h4v6h5v-8h3L12 3 2 12h3v8z" />
    };

    html! {
        <div class="container">
        <h1>{"Icons"}</h1>

        <h2>{"SvgIcon"}</h2>
        <p>{"If you need a custom SVG icon you can use the SvgIcon wrapper.\
            This component extends the native <svg> element:"}</p>
        <div class="demo-box">
            <SvgIcon color={Color::Primary} font_size={FontSize::Small}>
                {path.clone()}
            </SvgIcon>
            <SvgIcon color={Color::Secondary}>
                {path.clone()}
            </SvgIcon>
            <SvgIcon color={Color::Action} font_size={FontSize::Large}>
                {path.clone()}
            </SvgIcon>
            <SvgIcon color={Color::Disabled} style="font-size: 50px">
                {path.clone()}
            </SvgIcon>
        </div>

        <Abc color={Color::Error} font_size={FontSize::Large} />

        </div>
    }
}
