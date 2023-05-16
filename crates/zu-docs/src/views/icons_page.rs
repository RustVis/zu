// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{function_component, html, Html};
use zu::svg_icon::{Color, FontSize, SvgIcon};
use zuicon_material::Home;

use super::abc::Abc;

#[function_component(IconsPage)]
pub fn icons_page() -> Html {
    html! {
        <div class="container">
        <h1>{"Icons"}</h1>

        <h2>{"SvgIcon"}</h2>
        <p>{"If you need a custom SVG icon you can use the SvgIcon wrapper.\
            This component extends the native <svg> element:"}</p>
        <div class="demo-box">
            <Home />

            <SvgIcon>
                <path d="M10 20v-6h4v6h5v-8h3L12 3 2 12h3v8z" />
            </SvgIcon>
        </div>

        <h3>{"Color"}</h3>
        <div class="demo-box">
            <Home />
            <Home color={Color::Primary} />
            <Home color={Color::Secondary} />
            <Home color={Color::Success} />
            <Home color={Color::Action} />
            <Home color={Color::Disabled} />
            <Home style="color: var(--zu-colors-pink-500);" />
        </div>

        <Abc color={Color::Error} font_size={FontSize::Large} />

        </div>
    }
}
