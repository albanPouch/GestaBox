BEGIN;

INSERT OR IGNORE INTO Client (id_client, nom_client, prenom_client, raison_social, telephone_client) VALUES
    (1, 'Dupont', 'Jean', 'Dupont SA', '0102030405'),
    (2, 'Durand', 'Marie', NULL, '0607080910'),
    (3, 'Bernard', 'John', NULL, '0607080911');

INSERT OR IGNORE INTO Status (id_status, libelle_status) VALUES
    (1, 'En cours'),
    (2, 'Termine'),
    (3, 'Annule');

INSERT OR IGNORE INTO CategorieEmploye (id_categorie, libelle_categorie, taux_journalier) VALUES
    (1, 'Junior', 150.0),
    (2, 'Senior', 400.0),
    (3, 'Expert', 350.0);

INSERT OR IGNORE INTO SousTraitant (id_sous_traitant, nom_entreprise, taux_journalier) VALUES
    (1, 'SousTreat SARL', 200.0);

INSERT OR IGNORE INTO Occupation (id_occupation, libelle_occupation) VALUES
    (1, 'Reunion'),
    (2, 'Developpement'),
    (3, 'Architecture');

INSERT OR IGNORE INTO Planning (id_planning, annee) VALUES
    (1, '2023'),
    (2, '2024');

INSERT OR IGNORE INTO Competence (id_competence, libelle_competence) VALUES
    (1, 'Rust'),
    (2, 'SQL'),
    (3, 'Management');

INSERT OR IGNORE INTO type_mission (id_type_mission, libelle_type_mission) VALUES
    (1, 'Audit'),
    (2, 'Developpement');

INSERT OR IGNORE INTO Employee (id_employee, id_category) VALUES
    (1, 1),
    (2, 2),
    (3, 3);

INSERT OR IGNORE INTO Intervenant (
    id_intervant,
    nom_intervenant,
    prenom_intervenant,
    horaires,
    ville,
    code_postal,
    telephone_intervant,
    id_planning,
    id_employee,
    id_sous_traitant
) VALUES
    (1, 'Martin', 'Paul', '9h-17h', 'Paris', '75001', '0600000001', 1, 1, NULL),
    (2, 'Smith', 'John', '10h-18h', 'Lyon', '69001', '0600000002', 2, NULL, 1);

INSERT OR IGNORE INTO Mission (
    id_mission,
    temps_theorique,
    description,
    date_creation,
    date_debut,
    date_fin,
    derniere_modif,
    ville_mission,
    departement_mission,
    id_status,
    id_intervenant
) VALUES
    (1, '5j', 'Mission Site Web', '2023-01-01', '2023-02-01', '2023-03-01', '2023-01-02', 'Paris', '75', 1, 1),
    (2, '2j', 'Audit Securite', '2023-04-01', '2023-05-01', '2023-06-01', '2023-04-02', 'Lyon', '69', 2, 2);

INSERT OR IGNORE INTO Activite (
    id_activite,
    libelle_activite,
    temps_tache_theorique,
    prioritaire,
    faisable_en_teletravail,
    id_status,
    id_mission
) VALUES
    (1, 'Analyse Besoins', '1j', 1, 1, 2, 1),
    (2, 'Dev Backend', '3j', 1, 1, 1, 1),
    (3, 'Audit Technique', '2j', 1, 0, 1, 2);

INSERT OR IGNORE INTO Typer (id_type_mission, id_mission, importance) VALUES
    (2, 1, 5),
    (1, 2, 3);

INSERT OR IGNORE INTO Associer (id_client, id_mission, date_demande) VALUES
    (1, 1, '2023-01-01');

INSERT OR IGNORE INTO Posseder (id_intervenant, id_competence, niveau, preference) VALUES
    (1, 1, 4, 5),
    (1, 2, 5, 4),
    (2, 1, 2, 3),
    (2, 2, 3, 5);

UPDATE Posseder SET niveau = 4, preference = 5 WHERE id_intervenant = 1 AND id_competence = 1;
UPDATE Posseder SET niveau = 5, preference = 4 WHERE id_intervenant = 1 AND id_competence = 2;
UPDATE Posseder SET niveau = 2, preference = 3 WHERE id_intervenant = 2 AND id_competence = 1;
UPDATE Posseder SET niveau = 3, preference = 5 WHERE id_intervenant = 2 AND id_competence = 2;

INSERT OR IGNORE INTO Intervenir (id_intervenant, id_activite, date_intervention, heure_intervention) VALUES
    (1, 1, '2023-02-10', '10:00');

INSERT OR IGNORE INTO Demander (id_activite, id_competence) VALUES
    (2, 1),
    (3, 2);

INSERT OR IGNORE INTO Occuper (id_occupation, id_planning, date_debut, date_fin) VALUES
    (1, 1, '2023-01-10 09:00', '2023-01-10 11:00');

COMMIT;
