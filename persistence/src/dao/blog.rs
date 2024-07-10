use crate::dto::blog::{BlogDto, CreateBlogDto, UpdateBlogDto};
use crate::error::DaoError;
use chrono::Utc;
use migration::sea_orm::ActiveValue::Set;
use migration::sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    TransactionTrait, TryIntoModel,
};
use migration::JoinType;
use ppaass_blog_domain::entity::{
    BlogActiveModel, BlogAdditionalInfo, BlogColumn, BlogEntity, BlogRelation, UserColumn,
    UserEntity,
};
use sea_orm::{PaginatorTrait, QuerySelect, RelationTrait};
use uuid::Uuid;
pub async fn get_blog(database: &DatabaseConnection, token: &str) -> Result<BlogDto, DaoError> {
    let (blog_from_db, owner_from_db) = BlogEntity::find()
        .filter(BlogColumn::Token.eq(token))
        .find_also_related(UserEntity)
        .one(database)
        .await?
        .ok_or(DaoError::BlogNotFoundByToken(token.to_owned()))?;
    let owner_from_db =
        owner_from_db.ok_or(DaoError::BlogOwnerNotFoundByBlogToken(token.to_owned()))?;
    Ok(BlogDto {
        token: blog_from_db.token,
        title: blog_from_db.title,
        summary: blog_from_db.summary,
        labels: blog_from_db.additional_info.labels,
        create_date: blog_from_db.create_date,
        update_date: blog_from_db.update_date,
        owner_username: owner_from_db.username,
    })
}

pub async fn create_blog(
    database: &DatabaseConnection,
    CreateBlogDto {
        title,
        summary,
        labels,
        username,
    }: CreateBlogDto,
) -> Result<BlogDto, DaoError> {
    let owner_from_db = UserEntity::find()
        .filter(UserColumn::Username.eq(&username))
        .one(database)
        .await?
        .ok_or(DaoError::UserNotFoundByUsername(username))?;
    let blog_to_db = BlogActiveModel {
        title: Set(title),
        summary: Set(summary),
        additional_info: Set(BlogAdditionalInfo { labels }),
        token: Set(Uuid::new_v4().to_string()),
        create_date: Set(Utc::now()),
        update_date: Set(Utc::now()),
        user_id: Set(owner_from_db.id),
        ..Default::default()
    };
    let blog_from_db = database
        .transaction(|txn| Box::pin(async { blog_to_db.save(txn).await }))
        .await?;
    let blog_from_db = blog_from_db.try_into_model()?;
    Ok(BlogDto {
        token: blog_from_db.token,
        title: blog_from_db.title,
        summary: blog_from_db.summary,
        labels: blog_from_db.additional_info.labels,
        create_date: blog_from_db.create_date,
        update_date: blog_from_db.update_date,
        owner_username: owner_from_db.username,
    })
}

pub async fn update_blog(
    database: &DatabaseConnection,
    UpdateBlogDto {
        token,
        title,
        summary,
        labels,
    }: UpdateBlogDto,
) -> Result<BlogDto, DaoError> {
    let (blog_from_db, owner_from_db) = BlogEntity::find()
        .filter(BlogColumn::Token.eq(&token))
        .find_also_related(UserEntity)
        .one(database)
        .await?
        .ok_or(DaoError::BlogNotFoundByToken(token.to_owned()))?;
    let owner_from_db =
        owner_from_db.ok_or(DaoError::BlogOwnerNotFoundByBlogToken(token.to_owned()))?;
    let mut blog_from_db = blog_from_db.into_active_model();
    if let Some(title) = title {
        blog_from_db.title = Set(title);
    }
    if let Some(summary) = summary {
        blog_from_db.summary = Set(summary);
    }
    if let Some(labels) = labels {
        blog_from_db.additional_info = Set(BlogAdditionalInfo { labels });
    }
    blog_from_db.update_date = Set(Utc::now());
    let blog_from_db = database
        .transaction(|txn| Box::pin(async { blog_from_db.save(txn).await }))
        .await?;
    let blog_from_db = blog_from_db.try_into_model()?;
    Ok(BlogDto {
        token: blog_from_db.token,
        title: blog_from_db.title,
        summary: blog_from_db.summary,
        labels: blog_from_db.additional_info.labels,
        create_date: blog_from_db.create_date,
        update_date: blog_from_db.update_date,
        owner_username: owner_from_db.username,
    })
}

pub async fn find_all_blogs_by_username(
    database: &DatabaseConnection,
    username: String,
    page_index: u64,
    page_size: u64,
) -> Result<Vec<BlogDto>, DaoError> {
    let blogs_page = BlogEntity::find()
        .join(JoinType::InnerJoin, BlogRelation::User.def())
        .filter(UserColumn::Username.eq(&username))
        .paginate(database, page_size);
    let blogs = blogs_page.fetch_page(page_index).await?;
    Ok(blogs
        .into_iter()
        .map(move |blog| BlogDto {
            token: blog.token,
            title: blog.title,
            summary: blog.summary,
            labels: blog.additional_info.labels,
            create_date: blog.create_date,
            update_date: blog.update_date,
            owner_username: username.clone(),
        })
        .collect())
}
