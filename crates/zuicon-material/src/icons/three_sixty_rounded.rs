// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ThreeSixtyRounded)]
pub fn three_sixty_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ThreeSixtyRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M12 7C6.48 7 2 9.24 2 12c0 2.24 2.94 4.13 7 4.77v2.02c0 .45.54.67.85.35l2.79-2.79c.2-.2.2-.51 0-.71l-2.79-2.79c-.31-.31-.85-.09-.85.36v1.52c-3.15-.56-5-1.9-5-2.73 0-1.06 3.04-3 8-3s8 1.94 8 3c0 .66-1.2 1.68-3.32 2.34-.41.13-.68.51-.68.94 0 .67.65 1.16 1.28.96C20.11 15.36 22 13.79 22 12c0-2.76-4.48-5-10-5z"/>
        </SvgIcon>
    }
}
