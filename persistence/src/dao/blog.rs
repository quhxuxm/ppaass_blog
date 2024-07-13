use crate::dao::blog_label::{find_labels_by_blog, save_blog_label};
use crate::dao::label::save_all_label;
use crate::dao::PageDto;
use crate::dto::blog::{BlogDto, CreateBlogDto, UpdateBlogDto};
use crate::error::DaoError;
use chrono::Utc;
use ppaass_blog_domain::entity::{
    BlogActiveModel, BlogColumn, BlogEntity, BlogRelation, UserColumn, UserEntity,
};
use sea_orm::ActiveValue::Set;
use sea_orm::JoinType;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, TransactionTrait,
    TryIntoModel,
};
use sea_orm::{ConnectionTrait, PaginatorTrait, QuerySelect, RelationTrait};
use uuid::Uuid;
pub async fn get_blog<C: ConnectionTrait + TransactionTrait>(
    database: &C,
    token: &str,
) -> Result<BlogDto, DaoError> {
    let (blog_from_db, owner) = BlogEntity::find()
        .filter(BlogColumn::Token.eq(token))
        .find_also_related(UserEntity)
        .one(database)
        .await?
        .ok_or(DaoError::BlogNotFoundByToken(token.to_owned()))?;
    let owner = owner.ok_or(DaoError::BlogOwnerNotFoundByBlogToken(token.to_owned()))?;
    let blog_labels = find_labels_by_blog(database, blog_from_db.id).await?;
    Ok(BlogDto {
        token: blog_from_db.token,
        title: blog_from_db.title,
        summary: blog_from_db.summary,
        labels: blog_labels,
        create_date: blog_from_db.create_date,
        update_date: blog_from_db.update_date,
        owner_username: owner.username,
    })
}

pub async fn create_blog<C: ConnectionTrait + TransactionTrait>(
    database: &C,
    CreateBlogDto {
        title,
        summary,
        labels,
        username,
    }: CreateBlogDto,
) -> Result<BlogDto, DaoError> {
    let labels_clone = labels.clone();
    let username_clone = username.clone();
    let blog = database
        .transaction(|txn| {
            Box::pin(async move {
                let owner = UserEntity::find()
                    .filter(UserColumn::Username.eq(&username))
                    .one(txn)
                    .await?
                    .ok_or(DaoError::UserNotFoundByUsername(username.clone()))?;
                let blog = BlogActiveModel {
                    title: Set(title),
                    summary: Set(summary),
                    token: Set(Uuid::new_v4().to_string()),
                    create_date: Set(Utc::now()),
                    update_date: Set(Utc::now()),
                    user_id: Set(owner.id),
                    ..Default::default()
                };
                let blog = blog.save(txn).await?;
                let blog = blog.try_into_model()?;
                let label_ids = save_all_label(txn, labels).await?;
                for label_id in label_ids {
                    save_blog_label(txn, blog.id, label_id).await?;
                }
                Ok(blog)
            })
        })
        .await?;

    Ok(BlogDto {
        token: blog.token,
        title: blog.title,
        summary: blog.summary,
        labels: labels_clone,
        create_date: blog.create_date,
        update_date: blog.update_date,
        owner_username: username_clone,
    })
}

pub async fn update_blog<C: ConnectionTrait + TransactionTrait>(
    database: &C,
    UpdateBlogDto {
        token,
        title,
        summary,
        labels,
    }: UpdateBlogDto,
) -> Result<BlogDto, DaoError> {
    let (blog, owner_username) = database
        .transaction(|txn| {
            Box::pin(async move {
                let (blog, owner) = BlogEntity::find()
                    .filter(BlogColumn::Token.eq(&token))
                    .find_also_related(UserEntity)
                    .one(txn)
                    .await?
                    .ok_or(DaoError::BlogNotFoundByToken(token.to_owned()))?;
                let owner =
                    owner.ok_or(DaoError::BlogOwnerNotFoundByBlogToken(token.to_owned()))?;
                let mut blog = blog.into_active_model();
                if let Some(title) = title {
                    blog.title = Set(title);
                }
                if let Some(summary) = summary {
                    blog.summary = Set(summary);
                }
                blog.update_date = Set(Utc::now());
                let blog = blog.save(txn).await?;
                let blog = blog.try_into_model()?;
                if let Some(labels) = labels {
                    let label_ids = save_all_label(txn, labels).await?;
                    for label_id in label_ids {
                        save_blog_label(txn, blog.id, label_id).await?;
                    }
                }
                Ok((blog, owner.username))
            })
        })
        .await?;
    let labels = find_labels_by_blog(database, blog.id).await?;
    Ok(BlogDto {
        token: blog.token,
        title: blog.title,
        summary: blog.summary,
        labels,
        create_date: blog.create_date,
        update_date: blog.update_date,
        owner_username,
    })
}

pub async fn find_all_blogs_by_username<C: ConnectionTrait + TransactionTrait>(
    database: &C,
    username: String,
    page_index: u64,
    page_size: u64,
) -> Result<PageDto<BlogDto>, DaoError> {
    let blogs_page = BlogEntity::find()
        .join(JoinType::InnerJoin, BlogRelation::User.def())
        .filter(UserColumn::Username.eq(&username))
        .paginate(database, page_size);
    let page_number = blogs_page.num_pages().await?;
    let blogs = blogs_page.fetch_page(page_index).await?;
    let mut blog_dto_list = Vec::new();
    for blog in blogs {
        let labels = find_labels_by_blog(database, blog.id).await?;
        let blog_dto = BlogDto {
            token: blog.token,
            title: blog.title,
            summary: blog.summary,
            labels,
            create_date: blog.create_date,
            update_date: blog.update_date,
            owner_username: username.clone(),
        };
        blog_dto_list.push(blog_dto);
    }
    Ok(PageDto {
        items: blog_dto_list,
        page_index,
        page_size,
        page_number,
    })
}
