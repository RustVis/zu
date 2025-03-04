// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_router::BrowserRouter;
use zu::theme_provider::ThemeProvider;

use crate::components::main_content::MainContent;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <ThemeProvider>
            <BrowserRouter>
                <MainContent />
            </BrowserRouter>
        </ThemeProvider>
    }
}
