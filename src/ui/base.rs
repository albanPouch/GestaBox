use gpui::{
    App, Application, Bounds, Context, Entity, IntoElement, MouseButton, Window, WindowBounds,
    WindowOptions, div, prelude::*, px, rgb, size,
};

use crate::db::{client, client::Client, employee, employee::Employee, mission, mission::Mission};

use super::text_input::TextInput;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ActiveView {
    Home,
    ClientList,
    MissionList,
    EmployeeList,
}

struct ClientForm {
    selected_id: Option<i32>,
    nom: Entity<TextInput>,
    prenom: Entity<TextInput>,
    raison_social: Entity<TextInput>,
    telephone: Entity<TextInput>,
    message: String,
}

struct MissionForm {
    selected_id: Option<i32>,
    temps_theorique: Entity<TextInput>,
    description: Entity<TextInput>,
    date_debut: Entity<TextInput>,
    date_fin: Entity<TextInput>,
    ville_mission: Entity<TextInput>,
    departement_mission: Entity<TextInput>,
    id_status: i32,
    id_intervenant: i32,
    message: String,
}

struct EmployeeForm {
    selected_id: Option<i32>,
    id_category: Entity<TextInput>,
    message: String,
}

struct GestaBoxApp {
    active_view: ActiveView,
    clients: Vec<Client>,
    missions: Vec<Mission>,
    employees: Vec<Employee>,
    client_form: ClientForm,
    mission_form: MissionForm,
    employee_form: EmployeeForm,
}

impl GestaBoxApp {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            active_view: ActiveView::Home,
            clients: Vec::new(),
            missions: Vec::new(),
            employees: Vec::new(),
            client_form: ClientForm {
                selected_id: None,
                nom: cx.new(|cx| TextInput::new(cx, "Nom")),
                prenom: cx.new(|cx| TextInput::new(cx, "Prenom")),
                raison_social: cx.new(|cx| TextInput::new(cx, "Raison sociale")),
                telephone: cx.new(|cx| TextInput::new(cx, "Telephone")),
                message: String::new(),
            },
            mission_form: MissionForm {
                selected_id: None,
                temps_theorique: cx.new(|cx| TextInput::new(cx, "Temps theorique")),
                description: cx.new(|cx| TextInput::new(cx, "Description")),
                date_debut: cx.new(|cx| TextInput::new(cx, "Date debut YYYY-MM-DD")),
                date_fin: cx.new(|cx| TextInput::new(cx, "Date fin YYYY-MM-DD")),
                ville_mission: cx.new(|cx| TextInput::new(cx, "Ville")),
                departement_mission: cx.new(|cx| TextInput::new(cx, "Departement")),
                id_status: 1,
                id_intervenant: 1,
                message: String::new(),
            },
            employee_form: EmployeeForm {
                selected_id: None,
                id_category: cx.new(|cx| TextInput::new(cx, "Id categorie")),
                message: String::new(),
            },
        }
    }

    fn switch_view(&mut self, view: ActiveView, cx: &mut Context<Self>) {
        self.active_view = view;
        self.reload_current_view(cx);
        cx.notify();
    }

    fn reload_current_view(&mut self, cx: &mut Context<Self>) {
        match self.active_view {
            ActiveView::ClientList => self.reload_clients(cx),
            ActiveView::MissionList => self.reload_missions(cx),
            ActiveView::EmployeeList => self.reload_employees(cx),
            ActiveView::Home => {}
        }
    }

    fn reload_clients(&mut self, _: &mut Context<Self>) {
        self.clients = if let Ok(conn) = crate::db::get_connection() {
            client::get_all(&conn).unwrap_or_default()
        } else {
            Vec::new()
        };
    }

    fn reload_missions(&mut self, _: &mut Context<Self>) {
        self.missions = if let Ok(conn) = crate::db::get_connection() {
            mission::get_all(&conn).unwrap_or_default()
        } else {
            Vec::new()
        };
    }

    fn reload_employees(&mut self, _: &mut Context<Self>) {
        self.employees = if let Ok(conn) = crate::db::get_connection() {
            employee::get_all(&conn).unwrap_or_default()
        } else {
            Vec::new()
        };
    }

    fn set_input(input: &Entity<TextInput>, cx: &mut Context<Self>, value: impl Into<String>) {
        let value = value.into();
        input.update(cx, |input, _cx| input.set_value(value));
    }

    fn read_input(input: &Entity<TextInput>, cx: &mut Context<Self>) -> String {
        input.read(cx).value()
    }

    fn next_client_id(&self) -> i32 {
        self.clients.iter().map(|client| client.id_client).max().unwrap_or(0) + 1
    }

    fn next_mission_id(&self) -> i32 {
        self.missions.iter().map(|mission| mission.id_mission).max().unwrap_or(0) + 1
    }

    fn next_employee_id(&self) -> i32 {
        self.employees
            .iter()
            .map(|employee| employee.id_employee)
            .max()
            .unwrap_or(0)
            + 1
    }

    fn clear_client_form(&mut self, cx: &mut Context<Self>) {
        self.client_form.selected_id = None;
        self.client_form.message.clear();
        Self::set_input(&self.client_form.nom, cx, "");
        Self::set_input(&self.client_form.prenom, cx, "");
        Self::set_input(&self.client_form.raison_social, cx, "");
        Self::set_input(&self.client_form.telephone, cx, "");
    }

    fn clear_mission_form(&mut self, cx: &mut Context<Self>) {
        self.mission_form.selected_id = None;
        self.mission_form.message.clear();
        Self::set_input(&self.mission_form.temps_theorique, cx, "");
        Self::set_input(&self.mission_form.description, cx, "");
        Self::set_input(&self.mission_form.date_debut, cx, "");
        Self::set_input(&self.mission_form.date_fin, cx, "");
        Self::set_input(&self.mission_form.ville_mission, cx, "");
        Self::set_input(&self.mission_form.departement_mission, cx, "");
        self.mission_form.id_status = 1;
        self.mission_form.id_intervenant = 1;
    }

    fn clear_employee_form(&mut self, cx: &mut Context<Self>) {
        self.employee_form.selected_id = None;
        self.employee_form.message.clear();
        Self::set_input(&self.employee_form.id_category, cx, "");
    }

    fn select_client(&mut self, id: i32, cx: &mut Context<Self>) {
        if let Some(client) = self.clients.iter().find(|client| client.id_client == id) {
            self.client_form.selected_id = Some(id);
            self.client_form.message.clear();
            Self::set_input(&self.client_form.nom, cx, client.nom_client.clone());
            Self::set_input(&self.client_form.prenom, cx, client.prenom_client.clone());
            Self::set_input(
                &self.client_form.raison_social,
                cx,
                client.raison_social.clone().unwrap_or_default(),
            );
            Self::set_input(&self.client_form.telephone, cx, client.telephone_client.clone());
        }
    }

    fn select_mission(&mut self, id: i32, cx: &mut Context<Self>) {
        if let Some(mission) = self.missions.iter().find(|mission| mission.id_mission == id) {
            self.mission_form.selected_id = Some(id);
            self.mission_form.message.clear();
            Self::set_input(&self.mission_form.temps_theorique, cx, mission.temps_theorique.clone());
            Self::set_input(&self.mission_form.description, cx, mission.description.clone());
            Self::set_input(&self.mission_form.date_debut, cx, mission.date_debut.clone());
            Self::set_input(&self.mission_form.date_fin, cx, mission.date_fin.clone());
            Self::set_input(&self.mission_form.ville_mission, cx, mission.ville_mission.clone());
            Self::set_input(
                &self.mission_form.departement_mission,
                cx,
                mission.departement_mission.clone(),
            );
            self.mission_form.id_status = mission.id_status;
            self.mission_form.id_intervenant = mission.id_intervenant;
        }
    }

    fn today_iso() -> String {
        "2026-04-05".to_string()
    }

    fn select_employee(&mut self, id: i32, cx: &mut Context<Self>) {
        if let Some(employee) = self.employees.iter().find(|employee| employee.id_employee == id) {
            self.employee_form.selected_id = Some(id);
            self.employee_form.message.clear();
            Self::set_input(&self.employee_form.id_category, cx, employee.id_category.to_string());
        }
    }

    fn save_client(&mut self, _: &gpui::MouseDownEvent, _: &mut Window, cx: &mut Context<Self>) {
        let nom = Self::read_input(&self.client_form.nom, cx);
        let prenom = Self::read_input(&self.client_form.prenom, cx);
        let raison_social_raw = Self::read_input(&self.client_form.raison_social, cx);
        let telephone = Self::read_input(&self.client_form.telephone, cx);

        if nom.trim().is_empty() || prenom.trim().is_empty() || telephone.trim().is_empty() {
            self.client_form.message = "Nom, prenom et telephone sont obligatoires.".into();
            cx.notify();
            return;
        }

        let raison_social = if raison_social_raw.trim().is_empty() {
            None
        } else {
            Some(raison_social_raw.trim())
        };

        let result = if let Ok(conn) = crate::db::get_connection() {
            if let Some(id) = self.client_form.selected_id {
                client::update(&conn, id, nom.trim(), prenom.trim(), raison_social, telephone.trim())
            } else {
                client::insert(
                    &conn,
                    self.next_client_id(),
                    nom.trim(),
                    prenom.trim(),
                    raison_social,
                    telephone.trim(),
                )
            }
        } else {
            self.client_form.message = "Connexion DB impossible.".into();
            cx.notify();
            return;
        };

        match result {
            Ok(_) => {
                self.reload_clients(cx);
                self.client_form.message = "Client enregistre.".into();
                self.clear_client_form(cx);
            }
            Err(err) => self.client_form.message = format!("Erreur client: {err}"),
        }
        cx.notify();
    }

    fn delete_client(&mut self, _: &gpui::MouseDownEvent, _: &mut Window, cx: &mut Context<Self>) {
        let Some(id) = self.client_form.selected_id else {
            self.client_form.message = "Selectionne un client a supprimer.".into();
            cx.notify();
            return;
        };

        let result = crate::db::get_connection().and_then(|conn| client::delete(&conn, id));
        match result {
            Ok(_) => {
                self.reload_clients(cx);
                self.client_form.message = "Client supprime.".into();
                self.clear_client_form(cx);
            }
            Err(err) => self.client_form.message = format!("Erreur suppression client: {err}"),
        }
        cx.notify();
    }

    fn save_employee(
        &mut self,
        _: &gpui::MouseDownEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let id_category_raw = Self::read_input(&self.employee_form.id_category, cx);
        let Ok(id_category) = id_category_raw.trim().parse::<i32>() else {
            self.employee_form.message = "Id categorie invalide.".into();
            cx.notify();
            return;
        };

        let result = if let Ok(conn) = crate::db::get_connection() {
            if let Some(id) = self.employee_form.selected_id {
                employee::update(&conn, id, id_category)
            } else {
                employee::insert(&conn, self.next_employee_id(), id_category)
            }
        } else {
            self.employee_form.message = "Connexion DB impossible.".into();
            cx.notify();
            return;
        };

        match result {
            Ok(_) => {
                self.reload_employees(cx);
                self.employee_form.message = "Employe enregistre.".into();
                self.clear_employee_form(cx);
            }
            Err(err) => self.employee_form.message = format!("Erreur employe: {err}"),
        }
        cx.notify();
    }

    fn delete_employee(
        &mut self,
        _: &gpui::MouseDownEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(id) = self.employee_form.selected_id else {
            self.employee_form.message = "Selectionne un employe a supprimer.".into();
            cx.notify();
            return;
        };

        let result = crate::db::get_connection().and_then(|conn| employee::delete(&conn, id));
        match result {
            Ok(_) => {
                self.reload_employees(cx);
                self.employee_form.message = "Employe supprime.".into();
                self.clear_employee_form(cx);
            }
            Err(err) => self.employee_form.message = format!("Erreur suppression employe: {err}"),
        }
        cx.notify();
    }

    fn save_mission(
        &mut self,
        _: &gpui::MouseDownEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let temps_theorique = Self::read_input(&self.mission_form.temps_theorique, cx);
        let description = Self::read_input(&self.mission_form.description, cx);
        let date_debut = Self::read_input(&self.mission_form.date_debut, cx);
        let date_fin = Self::read_input(&self.mission_form.date_fin, cx);
        let ville_mission = Self::read_input(&self.mission_form.ville_mission, cx);
        let departement_mission = Self::read_input(&self.mission_form.departement_mission, cx);
        let date_creation = self
            .mission_form
            .selected_id
            .and_then(|id| {
                self.missions
                    .iter()
                    .find(|mission| mission.id_mission == id)
                    .map(|mission| mission.date_creation.clone())
            })
            .unwrap_or_else(Self::today_iso);
        let derniere_modif = Self::today_iso();

        if temps_theorique.trim().is_empty()
            || description.trim().is_empty()
            || date_debut.trim().is_empty()
            || date_fin.trim().is_empty()
            || ville_mission.trim().is_empty()
            || departement_mission.trim().is_empty()
        {
            self.mission_form.message = "Tous les champs mission sont obligatoires.".into();
            cx.notify();
            return;
        }

        let result = if let Ok(conn) = crate::db::get_connection() {
            if let Some(id) = self.mission_form.selected_id {
                mission::update(
                    &conn,
                    id,
                    temps_theorique.trim(),
                    description.trim(),
                    date_creation.trim(),
                    date_debut.trim(),
                    date_fin.trim(),
                    derniere_modif.trim(),
                    ville_mission.trim(),
                    departement_mission.trim(),
                    self.mission_form.id_status,
                    self.mission_form.id_intervenant,
                )
            } else {
                mission::insert(
                    &conn,
                    self.next_mission_id(),
                    temps_theorique.trim(),
                    description.trim(),
                    date_creation.trim(),
                    date_debut.trim(),
                    date_fin.trim(),
                    derniere_modif.trim(),
                    ville_mission.trim(),
                    departement_mission.trim(),
                    self.mission_form.id_status,
                    self.mission_form.id_intervenant,
                )
            }
        } else {
            self.mission_form.message = "Connexion DB impossible.".into();
            cx.notify();
            return;
        };

        match result {
            Ok(_) => {
                self.reload_missions(cx);
                self.mission_form.message = "Mission enregistree.".into();
                self.clear_mission_form(cx);
            }
            Err(err) => self.mission_form.message = format!("Erreur mission: {err}"),
        }
        cx.notify();
    }

    fn delete_mission(
        &mut self,
        _: &gpui::MouseDownEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(id) = self.mission_form.selected_id else {
            self.mission_form.message = "Selectionne une mission a supprimer.".into();
            cx.notify();
            return;
        };

        let result = crate::db::get_connection().and_then(|conn| mission::delete(&conn, id));
        match result {
            Ok(_) => {
                self.reload_missions(cx);
                self.mission_form.message = "Mission supprimee.".into();
                self.clear_mission_form(cx);
            }
            Err(err) => self.mission_form.message = format!("Erreur suppression mission: {err}"),
        }
        cx.notify();
    }

    fn assign_best_mission_intervenant(
        &mut self,
        _: &gpui::MouseDownEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(id_mission) = self.mission_form.selected_id else {
            self.mission_form.message =
                "Selectionne d'abord une mission a affecter.".into();
            cx.notify();
            return;
        };

        let result =
            crate::db::get_connection().and_then(|conn| mission::assign_best_intervenant(&conn, id_mission));

        match result {
            Ok(Some(recommendation)) => {
                self.reload_missions(cx);
                self.select_mission(id_mission, cx);
                self.mission_form.message = format!(
                    "Affectee a {} {} via {} (niveau {}, preference {}).",
                    recommendation.prenom_intervenant,
                    recommendation.nom_intervenant,
                    recommendation.libelle_competence,
                    recommendation.niveau,
                    recommendation.preference
                );
            }
            Ok(None) => {
                self.mission_form.message =
                    "Aucun intervenant disponible avec la competence demandee.".into();
            }
            Err(err) => {
                self.mission_form.message = format!("Erreur affectation mission: {err}");
            }
        }
        cx.notify();
    }

    fn render_home(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_wrap()
            .gap_6()
            .p_8()
            .justify_center()
            .child(self.render_dashboard_card("Clients", "CRUD clients", 0x4CAF50, ActiveView::ClientList, cx))
            .child(self.render_dashboard_card("Missions", "CRUD missions", 0x2196F3, ActiveView::MissionList, cx))
            .child(self.render_dashboard_card("Employes", "CRUD employes", 0xFFC107, ActiveView::EmployeeList, cx))
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
                this.switch_view(target_view, cx);
            }))
            .child(div().h_2().w_full().rounded_t_lg().bg(rgb(color_hex)))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .p_4()
                    .gap_2()
                    .child(div().text_lg().font_weight(gpui::FontWeight::BOLD).child(title.to_string()))
                    .child(div().text_sm().text_color(rgb(0xAAAAAA)).child(subtitle.to_string())),
            )
    }

    fn render_button(
        &self,
        label: &str,
        color: u32,
        cx: &mut Context<Self>,
        on_click: impl Fn(&mut Self, &gpui::MouseDownEvent, &mut Window, &mut Context<Self>) + 'static,
    ) -> impl IntoElement {
        div()
            .px_3()
            .py_2()
            .bg(rgb(color))
            .rounded_md()
            .cursor_pointer()
            .child(label.to_string())
            .on_mouse_down(MouseButton::Left, cx.listener(on_click))
    }

    fn render_back_button(&self, cx: &mut Context<Self>) -> impl IntoElement {
        self.render_button("Retour", 0x505862, cx, |this, _, _, cx| {
            this.switch_view(ActiveView::Home, cx);
        })
    }

    fn render_client_list(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .size_full()
            .gap_6()
            .p_6()
            .child(
                super::client_render::render_list(
                    &self.clients,
                    self.client_form.selected_id,
                    cx,
                    |this, cx| this.switch_view(ActiveView::Home, cx),
                    |this, id, cx| this.select_client(id, cx),
                ),
            )
            .child(
                div()
                    .w_96()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(div().text_xl().font_weight(gpui::FontWeight::BOLD).child(
                        if self.client_form.selected_id.is_some() {
                            "Modifier le client"
                        } else {
                            "Nouveau client"
                        },
                    ))
                    .child(self.client_form.nom.clone())
                    .child(self.client_form.prenom.clone())
                    .child(self.client_form.raison_social.clone())
                    .child(self.client_form.telephone.clone())
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .child(self.render_button("Enregistrer", 0x2E8B57, cx, Self::save_client))
                            .child(self.render_button("Nouveau", 0x3B82F6, cx, |this, _, _, cx| this.clear_client_form(cx)))
                            .child(self.render_button("Supprimer", 0xB91C1C, cx, Self::delete_client)),
                    )
                    .child(div().text_sm().text_color(rgb(0xFCA5A5)).child(self.client_form.message.clone())),
            )
    }

    fn render_mission_list(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .size_full()
            .gap_6()
            .p_6()
            .child(
                super::mission_render::render_list(
                    &self.missions,
                    self.mission_form.selected_id,
                    cx,
                    |this, cx| this.switch_view(ActiveView::Home, cx),
                    |this, id, cx| this.select_mission(id, cx),
                ),
            )
            .child(
                div()
                    .w_96()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(div().text_xl().font_weight(gpui::FontWeight::BOLD).child(
                        if self.mission_form.selected_id.is_some() {
                            "Modifier la mission"
                        } else {
                            "Nouvelle mission"
                        },
                    ))
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0xA3A3A3))
                            .child("Les dates techniques et les IDs sont geres automatiquement."),
                    )
                    .child(self.mission_form.temps_theorique.clone())
                    .child(self.mission_form.description.clone())
                    .child(self.mission_form.date_debut.clone())
                    .child(self.mission_form.date_fin.clone())
                    .child(self.mission_form.ville_mission.clone())
                    .child(self.mission_form.departement_mission.clone())
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .child(self.render_button("Auto-affecter", 0x8B5CF6, cx, Self::assign_best_mission_intervenant))
                            .child(self.render_button("Enregistrer", 0x2E8B57, cx, Self::save_mission))
                            .child(self.render_button("Nouveau", 0x3B82F6, cx, |this, _, _, cx| this.clear_mission_form(cx)))
                            .child(self.render_button("Supprimer", 0xB91C1C, cx, Self::delete_mission)),
                    )
                    .child(div().text_sm().text_color(rgb(0xFCA5A5)).child(self.mission_form.message.clone())),
            )
    }

    fn render_employee_list(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .size_full()
            .gap_6()
            .p_6()
            .child(
                super::employee_render::render_list(
                    &self.employees,
                    self.employee_form.selected_id,
                    cx,
                    |this, cx| this.switch_view(ActiveView::Home, cx),
                    |this, id, cx| this.select_employee(id, cx),
                ),
            )
            .child(
                div()
                    .w_96()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(div().text_xl().font_weight(gpui::FontWeight::BOLD).child(
                        if self.employee_form.selected_id.is_some() {
                            "Modifier l'employe"
                        } else {
                            "Nouvel employe"
                        },
                    ))
                    .child(self.employee_form.id_category.clone())
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .child(self.render_button("Enregistrer", 0x2E8B57, cx, Self::save_employee))
                            .child(self.render_button("Nouveau", 0x3B82F6, cx, |this, _, _, cx| this.clear_employee_form(cx)))
                            .child(self.render_button("Supprimer", 0xB91C1C, cx, Self::delete_employee)),
                    )
                    .child(div().text_sm().text_color(rgb(0xFCA5A5)).child(self.employee_form.message.clone())),
            )
    }
}

impl Render for GestaBoxApp {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x1E1E1E))
            .text_color(rgb(0xFFFFFF))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .p_4()
                    .bg(rgb(0x2D2D2D))
                    .border_b_1()
                    .border_color(rgb(0x404040))
                    .child(div().text_xl().font_weight(gpui::FontWeight::BOLD).child("GestaBox"))
                    .child(if self.active_view == ActiveView::Home {
                        div().into_any_element()
                    } else {
                        self.render_back_button(cx).into_any_element()
                    }),
            )
            .child(match self.active_view {
                ActiveView::Home => self.render_home(cx).into_any_element(),
                ActiveView::ClientList => self.render_client_list(cx).into_any_element(),
                ActiveView::MissionList => self.render_mission_list(cx).into_any_element(),
                ActiveView::EmployeeList => self.render_employee_list(cx).into_any_element(),
            })
    }
}

pub(crate) fn run() {
    Application::new().run(|cx: &mut App| {
        super::text_input::bind_keys(cx);

        let bounds = Bounds::centered(None, size(px(1280.0), px(720.0)), cx);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Maximized(bounds)),
                titlebar: None,
                ..Default::default()
            },
            |_, cx| cx.new(|cx| GestaBoxApp::new(cx)),
        )
        .unwrap();
    });
}
