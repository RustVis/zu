// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use yew::prelude::*;

use zuicon_bs::{app::App, icon_123::Icon123, wifi::Wifi, x::X};

struct MainApp;

impl Component for MainApp {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
          <div className="icons-list" style="font-size: 34px;">
            <App />
            <Icon123 />
            <X spin={true} />
            <Wifi rotate={180} />
          </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<MainApp>::new().render();
}
