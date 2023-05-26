// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{function_component, html, Html};
use zu::bottom_navigation::BottomNavigation;
use zu::bottom_navigation_action::BottomNavigationAction;
use zuicon_material::Favorite as FavoriteIcon;
use zuicon_material::Folder as FolderIcon;
use zuicon_material::LocationOn as LocationOnIcon;
use zuicon_material::Restore as RestoreIcon;

use crate::components::demo_box::DemoBox;

#[function_component(BottomNavigationPage)]
pub fn bottom_navigation_page() -> Html {
    // TODO(Shaohua): Handles onChange()

    html! {
        <div class="container">
        <h1>{"Bottom Navigation"}</h1>
        <p>{"Bottom navigation bars display three to five destinations at the bottom of a screen."}</p>

        <h2>{"Bottom navigation"}</h2>
        <p>{"When there are only three actions, display both icons and text labels at all times."}</p>
        <DemoBox>
            <BottomNavigation show_labels={true}>
                <BottomNavigationAction label={html!{"Recent"}} icon={html!{<RestoreIcon />}} />
                <BottomNavigationAction label={html!{"Favorites"}} icon={html!{<FavoriteIcon />}} />
                <BottomNavigationAction label={html!{"Nearby"}} icon={html!{<LocationOnIcon />}} />
            </BottomNavigation>
        </DemoBox>

        <h2>{"Bottom navigation with no label"}</h2>
        <p>{"If there are four or five actions, display inactive views as icons only."}</p>
        <DemoBox>
            <BottomNavigation style="width: 500px;">
                <BottomNavigationAction
                    label={html!{"Recents"}}
                    icon={html!{<RestoreIcon />}}
                    />
                <BottomNavigationAction
                    label={html!{"Favorites"}}
                    icon={html!{<FavoriteIcon />}}
                    />
                <BottomNavigationAction
                    label={html!{"Nearby"}}
                    icon={html!{<LocationOnIcon />}}
                    />
                <BottomNavigationAction
                    label={html!{"Folder"}}
                    icon={html!{<FolderIcon />}}
                    />
            </BottomNavigation>
        </DemoBox>
        </div>
    }
}
