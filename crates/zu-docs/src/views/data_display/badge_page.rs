// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{function_component, html, Html};
use zu::badge::{Badge, Content, Overlap, Variant};
use zu::r#box::r#Box;
use zu::styles::anchor_origin::AnchorOrigin;
use zu::styles::color::Color;
use zu::svg_icon::Color as SvgColor;
use zuicon_material::Mail;

use crate::components::demo_box::DemoBox;

#[function_component(BadgePage)]
pub fn badge_page() -> Html {
    // TODO(Shaohua): Add toggle visibility button
    // TODO(Shaohua): Add anchor-origin check buttons

    let rectangle = html! {
        <Box component="span" style=
            "background-color: var(--zu-palette-primary-main);\
             width: 40px;\
             height: 40px;" />
    };
    let circle = html! {
        <Box component="span" style=
            "background-color: var(--zu-palette-primary-main);\
             width: 40px;\
             height: 40px;\
             border-radius: 50%;" />
    };

    html! {
        <div class="container">
        <h1>{"Badge"}</h1>

        <h2>{"Badge Basic"}</h2>
        <p>{"Examples of badges containing text, using primary and secondary colors. The badge is applied to its children."}</p>
        <DemoBox>
          <Badge content={Content::Num(4)} color={Color::Primary}>
            <Mail color={SvgColor::Action} />
          </Badge>
        </DemoBox>

        <h2>{"Color"}</h2>
        <p>{"Use color prop to apply theme palette to component."}</p>
        <DemoBox>
            <Badge content={Content::Num(4)} color={Color::Secondary}>
                <Mail color={SvgColor::Action} />
            </Badge>
            <Badge content={Content::Num(4)} color={Color::Success}>
                <Mail color={SvgColor::Action} />
            </Badge>
        </DemoBox>

        <h2>{"Badge visibility"}</h2>
        <p>{"The visibility of badges can be controlled using the invisible prop."}</p>
        <DemoBox>
            <Badge content={Content::Num(4)} color={Color::Secondary}>
                <Mail />
            </Badge>
             <Badge color={Color::Secondary} variant={Variant::Dot} invisible={false}>
                <Mail />
            </Badge>
            <Badge color={Color::Secondary} variant={Variant::Dot} invisible={true}>
                <Mail />
            </Badge>
        </DemoBox>

        <h2>{"Maximum value"}</h2>
        <p>{"You can use the max prop to cap the value of the badge content."}</p>
        <DemoBox>
            <Badge color={Color::Secondary} content={Content::Num(99)}>
                <Mail />
            </Badge>
            <Badge color={Color::Secondary} content={Content::Num(100)}>
                <Mail />
            </Badge>
            <Badge color={Color::Secondary} content={Content::Num(1000)} max={999}>
                <Mail />
            </Badge>
        </DemoBox>

        <h2>{"Dot Badge"}</h2>
        <p>{"The dot prop changes a badge into a small dot. This can be used as a notification \
        that something has changed without giving a count."}</p>
        <DemoBox>
            <Badge color={Color::Secondary} variant={Variant::Dot}>
                <Mail />
            </Badge>
        </DemoBox>

        <h2>{"Badge overlap"}</h2>
        <p>{"You can use the overlap prop to place the badge relative to the corner of the wrapped element."}</p>
        <DemoBox>
            <Badge color={Color::Secondary} content={Content::Str(" ")}>
                {rectangle.clone()}
            </Badge>
            <Badge color={Color::Secondary} content={Content::Str(" ")} variant={Variant::Dot}>
                {rectangle}
            </Badge>
            <Badge color={Color::Secondary} overlap={Overlap::Circular}
                content={Content::Str(" ")}>
                {circle.clone()}
            </Badge>
            <Badge color={Color::Secondary} overlap={Overlap::Circular}
                content={Content::Str(" ")} variant={Variant::Dot}>
                {circle}
            </Badge>
        </DemoBox>

        <h2>{"Badge alignment"}</h2>
        <p>{"You can use the anchorOrigin prop to move the badge to any corner of the wrapped element."}</p>
        <DemoBox>
            <Badge color={Color::Secondary} content={Content::Num(42)}
                anchor_origin={AnchorOrigin::top_left()}>
                <Mail />
            </Badge>
            <Badge color={Color::Secondary} content={Content::Num(42)}
                anchor_origin={AnchorOrigin::top_right()}>
                <Mail />
            </Badge>
            <Badge color={Color::Secondary} content={Content::Num(42)}
                anchor_origin={AnchorOrigin::bottom_left()}>
                <Mail />
            </Badge>
            <Badge color={Color::Secondary} content={Content::Num(42)}
                anchor_origin={AnchorOrigin::bottom_right()}>
                <Mail />
            </Badge>
        </DemoBox>

        </div>
    }
}
