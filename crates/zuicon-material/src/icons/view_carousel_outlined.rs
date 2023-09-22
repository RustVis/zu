// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ViewCarouselOutlined)]
pub fn view_carousel_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ViewCarouselOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M2,7h4v10H2V7z M7,19h10V5H7V19z M9,7h6v10H9V7z M18,7h4v10h-4V7z"/>
        </SvgIcon>
    }
}
