// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(BrushOutlined)]
pub fn brush_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("BrushOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M7 16c.55 0 1 .45 1 1 0 1.1-.9 2-2 2-.17 0-.33-.02-.5-.05.31-.55.5-1.21.5-1.95 0-.55.45-1 1-1M18.67 3c-.26 0-.51.1-.71.29L9 12.25 11.75 15l8.96-8.96c.39-.39.39-1.02 0-1.41l-1.34-1.34c-.2-.2-.45-.29-.7-.29zM7 14c-1.66 0-3 1.34-3 3 0 1.31-1.16 2-2 2 .92 1.22 2.49 2 4 2 2.21 0 4-1.79 4-4 0-1.66-1.34-3-3-3z"/>
        </SvgIcon>
    }
}
