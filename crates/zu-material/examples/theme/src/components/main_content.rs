// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::prelude::*;
use zu_material::skeleton::Skeleton;
use zu_material::theme_provider::ThemeProvider;

#[function_component(MainContent)]
pub fn main_content() -> Html {
    html! {
        <ThemeProvider>
            <Skeleton />
        </ThemeProvider>
    }
}
