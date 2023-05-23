// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{function_component, html, Html};
use zu::badge::{Badge, Content};
use zu::styles::color::Color;

#[function_component(BadgePage)]
pub fn badge_page() -> Html {
    // TODO(Shaohua): Add invisible
    // TODO(Shaohua): Replace span with MailIcon
    html! {
        <div class="container">
        <h1>{"Badge"}</h1>

        <h2>{"Badge Basic"}</h2>
        <p>{"Examples of badges containing text, using primary and secondary colors. The badge is applied to its children."}</p>
        <div class="demo-box">
          <Badge content={Content::Num(4)} color={Color::Primary}>
            <span>{"hello"}</span>
          </Badge>
        </div>

        <h2>{"Color"}</h2>
        <p>{"Use color prop to apply theme palette to component."}</p>
        <div class="demo-box">
            <Badge content={Content::Num(4)} color={Color::Secondary}>
                <span>{"Hello"}</span>
            </Badge>
            <Badge content={Content::Num(4)} color={Color::Success}>
                <span>{"Hello"}</span>
            </Badge>
        </div>

        <h2>{"Maximum value"}</h2>
        <p>{"You can use the max prop to cap the value of the badge content."}</p>
        <div class="demo-box">
            <Badge color={Color::Secondary} content={Content::Num(99)}>
                <span>{"Hello"}</span>
            </Badge>
            <Badge color={Color::Secondary} content={Content::Num(100)}>
                <span>{"Hello"}</span>
            </Badge>
            <Badge color={Color::Secondary} content={Content::Num(1000)} max={999}>
                <span>{"Hello"}</span>
            </Badge>
        </div>

        </div>
    }
}
