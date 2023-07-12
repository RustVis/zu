// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{function_component, html, AttrValue, Html};
use zu::styles::color::DisabledColor;
use zu::svg_icon::{FontSize, Props, SvgIcon};
use zuicon_material::Home;

use crate::components::demo_box::DemoBox;

#[function_component(Plus)]
pub fn plus(props: &Props) -> Html {
    let new_props = Props {
        icon: From::from("Plus"),
        stroke: AttrValue::from("currentColor"),
        fill: AttrValue::from("none"),
        stroke_width: Some(1.5),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path strokeLinecap="round" strokeLinejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
        </SvgIcon>
    }
}

#[function_component(IconsPage)]
pub fn icons_page() -> Html {
    html! {
        <div class="container">
        <h1>{"Icons"}</h1>

        <h2>{"SvgIcon"}</h2>
        <p>{"If you need a custom SVG icon you can use the SvgIcon wrapper.\
            This component extends the native <svg> element:"}</p>
        <DemoBox>
            <Home />

            <SvgIcon>
                <path d="M10 20v-6h4v6h5v-8h3L12 3 2 12h3v8z" />
            </SvgIcon>
        </DemoBox>

        <h3>{"Color"}</h3>
        <DemoBox>
            <Home />
            <Home color={DisabledColor::Primary} />
            <Home color={DisabledColor::Secondary} />
            <Home color={DisabledColor::Success} />
            <Home color={DisabledColor::Action} />
            <Home color={DisabledColor::Disabled} />
            <Home style="color: var(--zu-colors-pink-500);" />
        </DemoBox>

        <h3>{"Size"}</h3>
        <DemoBox>
            <Home font_size={FontSize::Small} />
            <Home />
            <Home font_size={FontSize::Large} />
            <Home style={"font-size: 40px"} />
        </DemoBox>

        <h3>{"createSvgIcon"}</h3>
        <p>{"The createSvgIcon utility component is used to create the Material Icons. \
        It can be used to wrap an <svg> element or an SVG path \
        which is passed as a child to the SvgIcon component."}</p>
        <DemoBox>
            <Home />
            <Home color={DisabledColor::Primary} />
            <Plus />
            <Plus color={DisabledColor::Secondary} />
        </DemoBox>

        </div>
    }
}
