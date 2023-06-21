// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::prelude::*;
use yew_router::prelude::Routable;

use crate::views::data_display::{
    AvatarPage, BadgePage, ChipPage, DividerPage, IconsPage, ListPage, MaterialIconsPage,
    TablePage, TooltipPage, TypographyPage,
};
use crate::views::feedback::{
    AlertPage, BackdropPage, DialogPage, ProgressPage, SkeletonPage, SnackbarPage,
};
use crate::views::home_page::HomePage;
use crate::views::inputs::{
    AutocompletePage, ButtonGroupPage, ButtonPage, CheckboxPage, FloatingActionButtonPage,
    RadioGroupPage, SelectPage, SliderPage, SwitchPage, TextFieldPage, ToggleButtonPage,
    TransferListPage,
};
use crate::views::layout::{BoxPage, ContainerPage, StackPage};
use crate::views::navigation::{BottomNavigationPage, BreadcrumbsPage};
use crate::views::surfaces::{AccordionPage, AppBarPage, CardPage, PaperPage};

use crate::views::inputs::RatingPage;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Routable)]
pub enum Route {
    #[at("/")]
    Home,

    // inputs
    #[at("/autocomplete")]
    Autocomplete,
    #[at("/button")]
    Button,
    #[at("/button-group")]
    ButtonGroup,
    #[at("/checkbox")]
    Checkbox,
    #[at("/floating-action-button")]
    FloatingActionButton,
    #[at("/radio-group")]
    RadioGroup,
    #[at("/rating")]
    Rating,
    #[at("/select")]
    Select,
    #[at("/slider")]
    Slider,
    #[at("/switch")]
    Switch,
    #[at("/text-field")]
    TextField,
    #[at("/toggle-button")]
    ToggleButton,
    #[at("/transfer-list")]
    TransferList,

    // data display
    #[at("/avatar")]
    Avatar,
    #[at("/badge")]
    Badge,
    #[at("/chip")]
    Chip,
    #[at("/divider")]
    Divider,
    #[at("/icons")]
    Icons,
    #[at("/list")]
    List,
    #[at("/material-icons")]
    MaterialIcons,
    #[at("/table")]
    Table,
    #[at("/tooltip")]
    Tooltip,
    #[at("/typography")]
    Typography,

    // feedback
    #[at("/alert")]
    Alert,
    #[at("/backdrop")]
    Backdrop,
    #[at("/dialog")]
    Dialog,
    #[at("/progress")]
    Progress,
    #[at("/skeleton")]
    Skeleton,
    #[at("/snackbar")]
    Snackbar,

    // surfaces
    #[at("/accordion")]
    Accordion,
    #[at("/app-bar")]
    AppBar,
    #[at("/card")]
    Card,
    #[at("/paper")]
    Paper,

    // navigation
    #[at("/bottom-navigation")]
    BottomNavigation,
    #[at("/breadcrumbs")]
    Breadcrumbs,

    // layout
    #[at("/box")]
    Box,
    #[at("/container")]
    Container,
    #[at("/stack")]
    Stack,
    // utils
}

#[must_use]
#[allow(clippy::cognitive_complexity)]
#[allow(clippy::let_unit_value)]
pub fn switch_route(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<HomePage />},
        // inputs
        Route::Autocomplete => html! {<AutocompletePage />},
        Route::Button => html! {<ButtonPage />},
        Route::ButtonGroup => html! {<ButtonGroupPage />},
        Route::Checkbox => html! {<CheckboxPage />},
        Route::FloatingActionButton => html! {<FloatingActionButtonPage />},
        Route::RadioGroup => html! {<RadioGroupPage />},
        Route::Rating => html! {<RatingPage />},
        Route::Select => html! {<SelectPage />},
        Route::Slider => html! {<SliderPage />},
        Route::Switch => html! {<SwitchPage />},
        Route::TextField => html! {<TextFieldPage />},
        Route::ToggleButton => html! {<ToggleButtonPage />},
        Route::TransferList => html! {<TransferListPage />},

        // data display
        Route::Avatar => html! {<AvatarPage />},
        Route::Badge => html! {<BadgePage />},
        Route::Chip => html! {<ChipPage />},
        Route::Divider => html! {<DividerPage />},
        Route::Icons => html! {<IconsPage />},
        Route::List => html! {<ListPage />},
        Route::MaterialIcons => html! { <MaterialIconsPage />},
        Route::Table => html! {<TablePage />},
        Route::Tooltip => html! {<TooltipPage />},
        Route::Typography => html! {<TypographyPage />},

        // feedback
        Route::Alert => html! {<AlertPage />},
        Route::Backdrop => html! {<BackdropPage />},
        Route::Dialog => html! {<DialogPage />},
        Route::Progress => html! {<ProgressPage />},
        Route::Skeleton => html! {<SkeletonPage />},
        Route::Snackbar => html! {<SnackbarPage />},

        // surfaces
        Route::Accordion => html! {<AccordionPage />},
        Route::AppBar => html! {<AppBarPage />},
        Route::Card => html! {<CardPage />},
        Route::Paper => html! {<PaperPage />},

        // navigation
        Route::BottomNavigation => html! {<BottomNavigationPage />},
        Route::Breadcrumbs => html! {<BreadcrumbsPage />},

        // layout
        Route::Box => html! {<BoxPage />},
        Route::Container => html! {<ContainerPage />},
        Route::Stack => html! {<StackPage />},
        // utils
    }
}
