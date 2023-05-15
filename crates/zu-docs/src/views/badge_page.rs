// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{function_component, html, Html};
use zu::badge::{Badge, Color, Content};

#[function_component(BadgePage)]
pub fn badge_page() -> Html {
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

        </div>
    }
}
