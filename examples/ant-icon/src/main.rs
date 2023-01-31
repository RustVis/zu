// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use yew::prelude::*;

use zuicon_ant::{
    home_outlined::HomeOutlined, loading_outlined::LoadingOutlined, setting_filled::SettingFilled,
    smile_outlined::SmileOutlined, sync_outlined::SyncOutlined,
};

struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
          <div className="icons-list" style="font-size: 34px;">
            <HomeOutlined />
            <SettingFilled />
            <SmileOutlined />
            <SyncOutlined spin={true} />
            <SmileOutlined rotate={180} />
            <LoadingOutlined />
          </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
