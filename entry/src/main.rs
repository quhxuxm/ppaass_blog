use std::ops::Deref;
use std::sync::Arc;
use anyhow::Result;
use chrono::Utc;
use migration::{Migrator, MigratorTrait};
use migration::sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectOptions, Database, DatabaseConnection, EntityTrait,
    ModelTrait, QueryFilter,
};
use migration::sea_orm::ActiveValue::Set;
use ppaass_blog_domain::entity::blog::ActiveModel as BlogActiveModel;
use ppaass_blog_domain::entity::blog::Entity as BlogEntity;
use ppaass_blog_domain::entity::post::ActiveModel as PostActiveModel;
use ppaass_blog_domain::entity::post::Entity as PostEntity;
use ppaass_blog_domain::entity::user::{
    ActiveModel as UserActiveModel, Column as UserEntityColumn,
};
use ppaass_blog_domain::entity::user::Entity as UserEntity;
async fn init_database() -> Result<Arc<DatabaseConnection>> {
    let database_connect_options = ConnectOptions::new("mysql://root:123456@localhost/ppaass-db")
        .set_schema_search_path("ppaass-db") // Override the default schema
        .to_owned();
    let database = Database::connect(database_connect_options).await?;
    // Migrator::down(&database, None).await?;
    Migrator::up(&database, None).await?;
    Ok(Arc::new(database))
}

async fn init_user(database: &DatabaseConnection, index: usize) -> Result<()> {
    let user = UserActiveModel {
        user_name: Set(format!("username{index}")),
        password: Set(format!("password-username{index}")),
        display_name: Set(format!("Display name of user {index}")),
        register_date: Set(Utc::now()),
        ..Default::default()
    };
    let inserted_user = user.insert(database).await?;
    for j in 0..100 {
        let blog = BlogActiveModel {
            title: Set(format!("Title of blog {j} for user {index}")),
            summary: Set(format!("Summary of blog {j} for user {index}")),
            create_date: Set(Utc::now()),
            update_date: Set(Utc::now()),
            user_id: Set(inserted_user.id),
            ..Default::default()
        };
        let inserted_blog = blog.insert(database).await?;
        for p in 0..100 {
            let post = PostActiveModel {
                title: Set(format!("Title of post {p} in blog {j} of user {index}")),
                content: Set(Some(format!(
                    "Content of post {p} in blog {j} of user {index}"
                ))),
                create_date: Set(Utc::now()),
                update_date: Set(Utc::now()),
                blog_id: Set(inserted_blog.id),
                ..Default::default()
            };
            post.insert(database).await?;
        }
    }
    Ok(())
}

async fn init_seed_data(database: Arc<DatabaseConnection>) -> Result<()> {
    let mut tasks = Vec::new();
    for i in 0..100 {
        let database = database.clone();
        let join_handle = tokio::spawn(async move { init_user(&database, i).await });
        tasks.push(join_handle);
    }
    futures::future::join_all(tasks).await;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let database = init_database().await?;
    // init_seed_data(database).await?;
    let target_users = UserEntity::find()
        .filter(UserEntityColumn::DisplayName.contains("user 1"))
        .all(database.deref())
        .await?;
    for user in target_users.iter() {
        println!("###### Found user: {}", user.display_name);
        let blogs_and_posts = user
            .find_related(BlogEntity)
            .find_with_related(PostEntity)
            .all(database.deref())
            .await?;
        for (blog, posts) in blogs_and_posts.iter() {
            println!("@@@@ Found blog: {}", blog.title);
            for post in posts.iter() {
                println!(">>>> Found post: {}", post.title);
            }
        }
    }
    Ok(())
}
