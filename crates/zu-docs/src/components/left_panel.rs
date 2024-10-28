// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_router::prelude::{use_location, Link, Routable};

use crate::router::Route;

#[function_component(LeftPanel)]
pub fn left_panel() -> Html {
    let location = use_location().unwrap();
    let location_path = location.path();

    let link_cls = |r: Route| -> &'static str {
        let route_path = r.to_path();
        if route_path == location_path {
            "nav-link active"
        } else {
            "nav-link"
        }
    };

    html! {
        <div class="col-sm-4 col-lg-2">
            <ul class="nav nav-pills flex-column mb-auto bg-light">
                <li>
                    <div class="section-label">{"Input"}</div>
                    <ul>
                    <li class="nav-item">
                        <Link<Route> to={Route::Autocomplete} classes={link_cls(Route::Autocomplete)}>
                            {"Autocomplete"}
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::Button} classes={link_cls(Route::Button)}>
                        {"Button"}
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::ButtonGroup} classes={link_cls(Route::ButtonGroup)}>
                        {"ButtonGroup"}
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::Checkbox} classes={link_cls(Route::Checkbox)}>
                        {"Checkbox"}
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::FloatingActionButton}
                            classes={link_cls(Route::FloatingActionButton)}>
                        {"Floating Action Button"}
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::RadioGroup}
                            classes={link_cls(Route::RadioGroup)}>
                        {"Radio Group"}
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::Rating}
                            classes={link_cls(Route::Rating)}>
                        {"Rating"}
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::Select}
                            classes={link_cls(Route::Select)}>
                        {"Select"}
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::Slider}
                            classes={link_cls(Route::Slider)}>
                        {"Slider"}
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::Switch}
                            classes={link_cls(Route::Switch)}>
                        {"Switch"}
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::TextField}
                            classes={link_cls(Route::TextField)}>
                        {"Text Field"}
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::ToggleButton}
                            classes={link_cls(Route::ToggleButton)}>
                        {"Toggle Button"}
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::TransferList}
                            classes={link_cls(Route::TransferList)}>
                        {"Transfer List"}
                        </Link<Route>>
                    </li>
                    </ul>
                </li>

                <li>
                    <div class="section-label">{"Data Display"}</div>
                    <ul>
                    <li class="nav-item">
                        <Link<Route> to={Route::Avatar} classes={link_cls(Route::Avatar)}>
                        {"Avatar"}
                        </Link<Route>>
                    </li>

                    <li class="nav-item">
                        <Link<Route> to={Route::Badge} classes={link_cls(Route::Badge)}>
                        {"Badge"}
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::Chip} classes={link_cls(Route::Chip)}>
                        {"Chip"}
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::Divider} classes={link_cls(Route::Divider)}>
                        {"Divider"}
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::Icons} classes={link_cls(Route::Icons)}>
                        {"Icons"}
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::List} classes={link_cls(Route::List)}>
                        {"List"}
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::MaterialIcons} classes={link_cls(Route::MaterialIcons)}>
                        {"Material Icons"}
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::Table} classes={link_cls(Route::Table)}>
                        {"Table"}
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::Tooltip} classes={link_cls(Route::Tooltip)}>
                        {"Tooltip"}
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::Typography} classes={link_cls(Route::Typography)}>
                            {"Typography"}
                        </Link<Route>>
                    </li>
                    </ul>
                </li>

                <li>
                    <div class="section-label">{"Feedback"}</div>
                    <ul>
                    <li class="nav-item">
                        <Link<Route> to={Route::Alert} classes={link_cls(Route::Alert)}>
                        {"Alert"}
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::Backdrop} classes={link_cls(Route::Backdrop)}>
                        {"Backdrop"}
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::Dialog} classes={link_cls(Route::Dialog)}>
                        {"Dialog"}
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::Progress} classes={link_cls(Route::Progress)}>
                            {"Progress"}
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::Skeleton} classes={link_cls(Route::Skeleton)}>
                            {"Skeleton"}
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::Snackbar} classes={link_cls(Route::Snackbar)}>
                        {"Snackbar"}
                        </Link<Route>>
                    </li>
                    </ul>
                </li>

                <li>
                    <div class="section-label">{"Surfaces"}</div>
                    <ul>
                    <li class="nav-item">
                        <Link<Route> to={Route::Accordion} classes={link_cls(Route::Accordion)}>
                        {"Accordion"}
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::AppBar} classes={link_cls(Route::AppBar)}>
                        {"App Bar"}
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::Card} classes={link_cls(Route::Card)}>
                        {"Card"}
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::Paper} classes={link_cls(Route::Paper)}>
                        {"Paper"}
                        </Link<Route>>
                    </li>
                    </ul>
                </li>

                <li>
                    <div class="section-label">{"Navigation"}</div>
                    <ul>
                    <li class="nav-item">
                        <Link<Route> to={Route::BottomNavigation}
                            classes={link_cls(Route::BottomNavigation)}>
                        {"Bottom Navigation"}
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::Breadcrumbs} classes={link_cls(Route::Breadcrumbs)}>
                        {"Breadcrumbs"}
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::Drawer} classes={link_cls(Route::Drawer)}>
                        {"Drawer"}
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::Link} classes={link_cls(Route::Link)}>
                        {"Link"}
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::Menu} classes={link_cls(Route::Menu)}>
                        {"Menu"}
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::Pagination} classes={link_cls(Route::Pagination)}>
                        {"Pagination"}
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::SpeedDial} classes={link_cls(Route::SpeedDial)}>
                        {"SpeedDial"}
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::Stepper} classes={link_cls(Route::Stepper)}>
                        {"Stepper"}
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::Tabs} classes={link_cls(Route::Tabs)}>
                        {"Tabs"}
                        </Link<Route>>
                    </li>
                    </ul>
                </li>

                <li>
                    <div class="section-label">{"Layout"}</div>
                    <ul>
                    <li class="nav-item">
                        <Link<Route> to={Route::Box} classes={link_cls(Route::Box)}>
                        {"Box"}
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::Container} classes={link_cls(Route::Container)}>
                        {"Container"}
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::Grid} classes={link_cls(Route::Grid)}>
                            {"Grid"}
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::ImageList} classes={link_cls(Route::ImageList)}>
                            {"Image List"}
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::Stack} classes={link_cls(Route::Stack)}>
                            {"Stack"}
                        </Link<Route>>
                    </li>
                    </ul>
                </li>

                <li>
                    <div class="section-label">{"Utils"}</div>
                    <ul>
                    <li class="nav-item">
                        <Link<Route> to={Route::CssBaseline} classes={link_cls(Route::CssBaseline)}>
                            {"Css Baseline"}
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::Modal} classes={link_cls(Route::Modal)}>
                            {"Modal"}
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::Popover} classes={link_cls(Route::Popover)}>
                            {"Popover"}
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::Popper} classes={link_cls(Route::Popper)}>
                            {"Popper"}
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::Transitions} classes={link_cls(Route::Transitions)}>
                            {"Transitions"}
                        </Link<Route>>
                    </li>
                    </ul>
                </li>
            </ul>
        </div>
    }
}
