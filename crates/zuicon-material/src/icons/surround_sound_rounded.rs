// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SurroundSoundRounded)]
pub fn surround_sound_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SurroundSoundRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20 4H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zM7.11 16.89c-.43.43-1.14.39-1.51-.09C4.53 15.39 4 13.69 4 12s.53-3.38 1.59-4.8c.37-.48 1.08-.53 1.51-.1.35.35.39.9.1 1.29C6.4 9.46 6 10.73 6 12s.4 2.53 1.2 3.6c.3.39.26.94-.09 1.29zM12 16c-2.21 0-4-1.79-4-4s1.79-4 4-4 4 1.79 4 4-1.79 4-4 4zm4.9.9c-.35-.35-.39-.9-.09-1.29C17.6 14.54 18 13.27 18 12s-.4-2.53-1.2-3.6c-.3-.39-.26-.95.09-1.3.43-.43 1.14-.39 1.51.09 1.07 1.41 1.6 3.1 1.6 4.8 0 1.69-.53 3.38-1.59 4.8-.37.49-1.08.54-1.51.11zM12 10c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2z"/>
        </SvgIcon>
    }
}
