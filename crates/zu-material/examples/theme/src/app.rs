// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::prelude::*;
use yew_router::BrowserRouter;

use crate::components::main_content::MainContent;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <MainContent />
        </BrowserRouter>
    }
}
