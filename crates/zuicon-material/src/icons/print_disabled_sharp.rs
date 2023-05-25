// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PrintDisabledSharp)]
pub fn print_disabled_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PrintDisabledSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0z" fill="none"/><path d="M9.65 7H18V3.01H6v.35zm1.01 1.01l9 8.99H22v-5.99c0-1.66-1.34-3-3-3h-8.34zM19 10c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zM1.41 1.6L0 3.01l5 5c-1.66 0-3 1.33-3 2.99v6h4v4h12l2.95 2.96 1.41-1.41L1.41 1.6zM8 19.01V15h4l4 4-8 .01z"/>
        </SvgIcon>
    }
}
