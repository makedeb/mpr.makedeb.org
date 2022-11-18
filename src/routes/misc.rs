use rocket::{get, State};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, QueryOrder, Order, QuerySelect};

use crate::{util, entities::prelude::{PackageBases, PackageBasesColumn}};
use serde_json::json;

#[get("/")]
pub async fn home(db: &State<DatabaseConnection>) -> String {
    let mut ctx = util::get_context();

    // Get the 10 most recently updated PackageBases.
    let updated_pkgbases: Vec<String> = PackageBases::find()
        .filter(PackageBasesColumn::PackagerUid.is_not_null())
        .order_by(PackageBasesColumn::ModifiedTs, Order::Desc)
        .limit(10)
        .all(db.inner())
        .await
        .unwrap()
        .iter()
        .map(|pkg| pkg.name.to_string())
        .collect();
    ctx.insert("updated_pkgbases", json!(updated_pkgbases));
    
    // Get the 10 most popular packages.
    let popular_pkgbases: Vec<String> = PackageBases::find()
        .filter(PackageBasesColumn::PackagerUid.is_not_null())
        .order_by(PackageBasesColumn::Popularity, Order::Desc)
        .limit(10)
        .all(db.inner())
        .await
        .unwrap()
        .iter()
        .map(|pkg| pkg.name.to_string())
        .collect();
    ctx.insert("popular_pkgbases", json!(popular_pkgbases));
    
    let mut pkgstr= String::new();

    pkgstr
}