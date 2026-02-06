use gpui::{
    div, prelude::*, px, rgb, size, App, Application, Bounds, Context, MouseButton, Window,
    WindowBounds, WindowOptions,
};
use crate::db::{client::Client, client, get_connection, mission::{self, Mission}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ActiveView {
    Home,
    ClientList,
    MissionList,
}

struct GestaBoxApp {
    active_view: ActiveView,
    clients: Vec<Client>,
    missions: Vec<Mission>,
}

impl GestaBoxApp {
    fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            active_view: ActiveView::Home,
            clients: Vec::new(),
            missions: Vec::new(),
        }
    }
    
    
    fn switch_view(&mut self, view: ActiveView, cx: &mut Context<Self>) {
        self.active_view = view;
        match self.active_view {
            ActiveView::ClientList => {
                self.clients = super::client_render::load_data();
            }
            ActiveView::MissionList => {
                self.missions = super::mission_render::load_data();
            }
            _ => {}
        }
        cx.notify();
    }

    /////////////////////
    /// MENU HOME
    /////////////////////
    fn render_home(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_wrap()
            .gap_6()
            .p_8()
            .justify_center()
            .child(
                self.render_dashboard_card("Clients", "Gérer vos clients", 0x4CAF50, ActiveView::ClientList, cx)
            )
            .child(
                self.render_dashboard_card("Missions", "Suivi des missions", 0x2196F3, ActiveView::MissionList, cx)
            )
            .child(
                self.render_not_implemented_card("Employés", "Gestion RH", 0xFFC107)
            )
            .child(
                self.render_not_implemented_card("Paramètres", "Configuration", 0x9E9E9E)
            )
    }

    fn render_client_list(&self, cx: &mut Context<Self>) -> impl IntoElement {
        super::client_render::render_list(&self.clients, cx, |this, cx| {
            this.switch_view(ActiveView::Home, cx);
        })
    }

    fn render_mission_list(&self, cx: &mut Context<Self>) -> impl IntoElement {
        super::mission_render::render_list(&self.missions, cx, |this, cx| {
            this.switch_view(ActiveView::Home, cx);
        })
    }

    fn render_dashboard_card(
        &self,
        title: &str,
        subtitle: &str,
        color_hex: u32,
        target_view: ActiveView,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_64()
            .h_40()
            .bg(rgb(0x2D2D2D))
            .rounded_lg()
            .border_1()
            .border_color(rgb(0x404040))
            .shadow_md()
            .cursor_pointer()
            .hover(|s| s.bg(rgb(0x383838)))
            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| {
                this.switch_view(target_view, cx); // The ActiveView enum doesn't implement Copy/Clone by default? We verify this.
            }))
            .child(
                div()
                    .h_2()
                    .w_full()
                    .rounded_t_lg()
                    .bg(rgb(color_hex)),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .p_4()
                    .gap_2()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child(title.to_string()),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0xAAAAAA))
                            .child(subtitle.to_string()),
                    ),
            )
    }

    fn render_not_implemented_card(
        &self,
        title: &str,
        subtitle: &str,
        color_hex: u32,
    ) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_64()
            .h_40()
            .bg(rgb(0x2D2D2D))
            .rounded_lg()
            .border_1()
            .border_color(rgb(0x404040))
            .shadow_md()
            .opacity(0.6) // Visual indication it is disabled/custom
            .child(
                div()
                    .h_2()
                    .w_full()
                    .rounded_t_lg()
                    .bg(rgb(color_hex)),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .p_4()
                    .gap_2()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child(title.to_string()),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0xAAAAAA))
                            .child(subtitle.to_string()),
                    ),
            )
    }
}

impl Render for GestaBoxApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x1E1E1E)) // Dark background
            .text_color(rgb(0xFFFFFF))
            .child(
                // Header
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .p_4()
                    .bg(rgb(0x2D2D2D))
                    .border_b_1()
                    .border_color(rgb(0x404040))
                    .child(
                        div()
                            .text_xl()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("GestaBox"),
                    )
            )
            .child(
                // Content based on view
                match self.active_view {
                    ActiveView::Home => self.render_home(cx).into_any_element(),
                    ActiveView::ClientList => self.render_client_list(cx).into_any_element(),
                    ActiveView::MissionList => self.render_mission_list(cx).into_any_element(),
                }
            )
    }
}

/////////////////////
/// DÉMARRAGE DU GUI
/////////////////////
pub(crate) fn run() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1280.0), px(720.0)), cx);
        
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Maximized(bounds)),
                titlebar: None, 
                ..Default::default()
            },
            |_, cx| {
                cx.new(|cx| GestaBoxApp::new(cx))
            },
        )
        .unwrap();
    });
}