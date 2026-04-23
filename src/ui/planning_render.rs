use gpui::{div, prelude::*, rgb, IntoElement};
use crate::db::mission::EmploiDuTempsLigne;

pub fn render_liste<V: 'static>(
    lignes: &[EmploiDuTempsLigne],
    filtre: &str,
) -> impl IntoElement {
    let filtre_lower = filtre.to_lowercase();

    let lignes_filtrees: Vec<&EmploiDuTempsLigne> = lignes
        .iter()
        .filter(|l| {
            filtre_lower.is_empty()
                || l.nom.to_lowercase().contains(&filtre_lower)
                || l.prenom.to_lowercase().contains(&filtre_lower)
        })
        .collect();

    div()
        .flex()
        .flex_col()
        .flex_1()
        .p_4()
        .gap_3()
        .children(lignes_filtrees.into_iter().map(|ligne| {
            let badge_color = if ligne.est_prestataire { 0x7C3AED } else { 0x0D7377 };
            let badge_label = if ligne.est_prestataire { "Prestataire" } else { "Employe" };

            div()
                .flex()
                .flex_col()
                .p_3()
                .bg(rgb(0x2D2D2D))
                .rounded_lg()
                .border_1()
                .border_color(rgb(0x404040))
                .gap_2()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_3()
                        .child(
                            div()
                                .text_base()
                                .font_weight(gpui::FontWeight::BOLD)
                                .child(format!("{} {}", ligne.prenom, ligne.nom)),
                        )
                        .child(
                            div()
                                .px_2()
                                .py_1()
                                .bg(rgb(badge_color))
                                .rounded_md()
                                .text_xs()
                                .child(badge_label),
                        ),
                )
                .child(if ligne.missions.is_empty() {
                    div()
                        .text_sm()
                        .text_color(rgb(0x777777))
                        .child("Aucune mission assignee")
                        .into_any_element()
                } else {
                    div()
                        .flex()
                        .flex_col()
                        .gap_1()
                        .children(ligne.missions.iter().map(|m| {
                            let (status_color, status_label) = match m.id_status {
                                1 => (0x2563EB, "En cours"),
                                2 => (0x16A34A, "Termine"),
                                3 => (0xDC2626, "Annule"),
                                _ => (0x6B7280, "Inconnu"),
                            };
                            div()
                                .flex()
                                .items_center()
                                .gap_3()
                                .px_3()
                                .py_2()
                                .bg(rgb(0x1E1E1E))
                                .rounded_md()
                                .child(
                                    div()
                                        .w_2()
                                        .h_2()
                                        .rounded_full()
                                        .bg(rgb(status_color)),
                                )
                                .child(
                                    div()
                                        .flex_1()
                                        .text_sm()
                                        .child(format!(
                                            "#{} — {}",
                                            m.id_mission, m.description
                                        )),
                                )
                                .child(
                                    div()
                                        .text_sm()
                                        .text_color(rgb(0xAAAAAA))
                                        .child(format!(
                                            "{} → {}",
                                            m.date_debut, m.date_fin
                                        )),
                                )
                                .child(
                                    div()
                                        .px_2()
                                        .py_1()
                                        .bg(rgb(status_color))
                                        .rounded_md()
                                        .text_xs()
                                        .child(status_label),
                                )
                        }))
                        .into_any_element()
                })
        }))
}
