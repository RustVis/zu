// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::prelude::*;
use zu::skeleton::{Animation, Skeleton, Variant};

#[function_component(SkeletonPage)]
pub fn skeleton_page() -> Html {
    html! {
        <div class="container">
            <h1>{"Skeleton"}</h1>
            <p>{"The data for your components might not be immediately available."}</p>
            <div class="demo-box">
                <Skeleton variant={Variant::Rect} width={210} height={118} />
            </div>

            <h2>{"Variants"}</h2>
            <p>{"For variant=Variant::Text, adjust the height via font-size"}</p>
            <div class="demo-box">
                <Skeleton variant={Variant::Text} style={"font-size: '1rem'"} />
            </div>
            <p>{"For other variants, adjust the size with `width` and `height`"}</p>
            <div class="demo-box">
                <Skeleton variant={Variant::Circle} width={40} height={40} />
                <Skeleton variant={Variant::Rect} width={210} height={60} />
                <Skeleton variant={Variant::Rounded} width={210} height={60} />
            </div>

            <h2>{"Animation"}</h2>
            <p>{"By default, the skeleton pulsates, but you can change the animation to a wave or disable it entirely."}</p>
            <div class="demo-box">
                <Skeleton />
                <Skeleton animation={Animation::Wave} />
                <Skeleton animation={Animation::None} />
            </div>
        </div>
    }
}
