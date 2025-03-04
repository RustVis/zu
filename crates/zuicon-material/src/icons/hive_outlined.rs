// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(HiveOutlined)]
pub fn hive_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("HiveOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21.5,9l-2.25-4h-3.31l-1.69-3h-4.5L8.06,5H4.75L2.5,9l1.69,3L2.5,15l2.25,4h3.31l1.69,3h4.5l1.69-3h3.31l2.25-4l-1.69-3 L21.5,9z M19.21,9l-1.12,2h-2.14l-1.12-2l1.12-2h2.14L19.21,9z M10.94,14l-1.12-2l1.12-2h2.12l1.12,2l-1.12,2H10.94z M13.08,4 l1.12,1.98L13.06,8h-2.12L9.8,5.98L10.92,4H13.08z M5.92,7h2.14l1.12,2l-1.12,2H5.92L4.79,9L5.92,7z M4.79,15l1.12-2h2.14l1.12,2 l-1.12,2H5.92L4.79,15z M10.92,20L9.8,18.02L10.94,16h2.12l1.13,2.02L13.08,20H10.92z M18.08,17h-2.14l-1.12-2l1.12-2h2.14l1.12,2 L18.08,17z"/>
        </SvgIcon>
    }
}
