// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(RecommendRounded)]
pub fn recommend_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("RecommendRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm6 9.8c.02.17-.02.35-.1.5l-2.1 4.9c-.22.51-.74.83-1.3.8H9c-1.1 0-2-.9-2-2v-5c-.02-.38.13-.74.4-1L12 5l.69.69c.18.19.29.44.3.7v.2L12.41 10H17c.55 0 1 .45 1 1v.8z"/>
        </SvgIcon>
    }
}
