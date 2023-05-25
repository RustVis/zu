// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DriveFileMoveSharp)]
pub fn drive_file_move_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DriveFileMoveSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22,6H12l-2-2H2v16h20V6z M12,17v-3H8v-2h4V9l4,4L12,17z"/>
        </SvgIcon>
    }
}
