// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(WbIridescentRounded)]
pub fn wb_iridescent_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("WbIridescentRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M6 15h12c.55 0 1-.45 1-1v-3.95c0-.55-.45-1-1-1H6c-.55 0-1 .45-1 1V14c0 .55.45 1 1 1zm5-13v1.05c0 .55.45.95 1 .95s1-.4 1-.95V2c0-.55-.45-1-1-1s-1 .45-1 1zm7.34 2.3l-.38.38c-.39.39-.39 1.02 0 1.41.39.39 1.02.39 1.41 0l.38-.38c.39-.39.39-1.02 0-1.41-.39-.39-1.02-.39-1.41 0zM13 22v-.96c0-.55-.45-1-1-1s-1 .45-1 1V22c0 .55.45 1 1 1s1-.45 1-1zm6.74-3.61l-.39-.39c-.39-.39-1.02-.39-1.41 0-.39.39-.39 1.02 0 1.41l.38.39c.39.39 1.02.39 1.41 0l.01-.01c.39-.38.39-1.02 0-1.4zM4.25 5.71l.39.39c.39.39 1.02.39 1.41 0 .39-.39.39-1.02 0-1.41l-.39-.39c-.39-.39-1.02-.39-1.41 0-.38.39-.38 1.03 0 1.41zm1.42 14.08l.38-.38c.39-.39.39-1.02 0-1.41-.39-.39-1.02-.39-1.41 0l-.38.38c-.39.39-.39 1.02 0 1.41.38.39 1.02.39 1.41 0z"/>
        </SvgIcon>
    }
}
