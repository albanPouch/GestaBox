mod db;
mod ui;

fn main() {
    db::init_gestabox_db().expect("TODO: JE PANIQUE ENCORE PLUS");
    ui::base::run();
}
