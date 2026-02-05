use gpui::{
    div, prelude::*, px, rgb, size, App, Application, Bounds, Context, Window,
    WindowBounds, WindowOptions,
};

struct Home {}

impl Render for Home {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
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
                //  PAS IMPLEMENTE
                    // .child(
                    //     div()
                    //         .text_sm()
                    //         .text_color(rgb(0xAAAAAA))
                    //         .child("Bienvenue, Utilisateur"),
                    // ),
            )
            .child(
                // Main Content Area
                div()
                    .flex()
                    .flex_wrap()
                    .gap_6()
                    .p_8()
                    .justify_center()
                    .child(self.render_dashboard_card("Clients", "Gérer vos clients", 0x4CAF50))
                    .child(self.render_dashboard_card("Missions", "Suivi des missions", 0x2196F3))
                    .child(self.render_dashboard_card("Employés", "Gestion RH", 0xFFC107))
                    .child(self.render_dashboard_card("Paramètres", "Configuration", 0x9E9E9E)),
            )
    }
}

impl Home {
    fn render_dashboard_card(
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
            .cursor_pointer()
            .hover(|s| s.bg(rgb(0x383838)))
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

// Lancement du GUI
pub(crate) fn run() {
    Application::new().run(|cx: &mut App| {
        // Create initial bounds, though maximized will override it effectively for the user
        let bounds = Bounds::centered(None, size(px(1280.0), px(720.0)), cx);
        
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Maximized(bounds)),
                titlebar: None, 
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_| Home {})
            },
        )
        .unwrap();
    });
}