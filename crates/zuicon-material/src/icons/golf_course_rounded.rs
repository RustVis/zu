// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(GolfCourseRounded)]
pub fn golf_course_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("GolfCourseRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M11 18.03V8.98l4.22-2.15c.73-.37.73-1.43-.01-1.79l-4.76-2.33C9.78 2.38 9 2.86 9 3.6V19c0 .55-.45 1-1 1s-1-.45-1-1v-.73c-1.79.35-3 .99-3 1.73 0 1.1 2.69 2 6 2s6-.9 6-2c0-.99-2.16-1.81-5-1.97z"/>
        </SvgIcon>
    }
}
