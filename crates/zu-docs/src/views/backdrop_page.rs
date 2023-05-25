// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{function_component, html, Html};
use zu::backdrop::Backdrop;
use zu::button::Button;
use zu::circular_progress::CircularProgress;
use zu::styles::color::Color;

#[function_component(BackdropPage)]
pub fn backdrop_page() -> Html {
    // TODO(Shaohua): Handle events and add callback.
    let open = true;

    html! {
        <div class="container">
        <h1>{"Backdrop"}</h1>
        <p>{"The Backdrop signals a state change within the application and \
        can be used for creating loaders, dialogs, and more. "}</p>

        <h2>{"Example"}</h2>
        <p>{"he demo below shows a basic Backdrop with a Circular Progress component \
         in the foreground to indicate a loading state. "}</p>

        <div class="demo-box">
            <Button>{"Show backdrop"}</Button>
            <Backdrop
                style="color: #fff; z-index: calc(var(--zu-zIndex-drawer) + 1);"
                open={open}>
                <CircularProgress color={Color::Inherit} />
            </Backdrop>
        </div>
        </div>
    }
}
