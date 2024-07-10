use crate::dto::post::{CreatePostDto, PostDto, UpdatePostDto};
use crate::error::DaoError;
use chrono::Utc;
use migration::JoinType;
use ppaass_blog_domain::entity::{
    BlogColumn, BlogEntity, PostActiveModel, PostAdditionalInfo, PostColumn, PostEntity,
    PostRelation,
};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel,
    PaginatorTrait, QueryFilter, QuerySelect, RelationTrait, TransactionTrait, TryIntoModel,
};
use uuid::Uuid;
pub async fn create_post(
    database: &DatabaseConnection,
    CreatePostDto {
        title,
        content,
        labels,
        blog_token,
    }: CreatePostDto,
) -> Result<PostDto, DaoError> {
    let blog_from_db = BlogEntity::find()
        .filter(BlogColumn::Token.eq(&blog_token))
        .one(database)
        .await?
        .ok_or(DaoError::BlogNotFoundByToken(blog_token.clone()))?;
    let post_to_db = PostActiveModel {
        token: Set(Uuid::new_v4().to_string()),
        title: Set(title),
        content: Set(content),
        blog_id: Set(blog_from_db.id),
        create_date: Set(Utc::now()),
        update_date: Set(Utc::now()),
        additional_info: Set(PostAdditionalInfo { labels }),
        ..Default::default()
    };
    let post_from_db = database
        .transaction(|txn| Box::pin(async { post_to_db.save(txn).await }))
        .await?;
    let post_from_db = post_from_db.try_into_model()?;
    Ok(PostDto {
        token: post_from_db.token,
        title: post_from_db.title,
        content: post_from_db.content,
        labels: post_from_db.additional_info.labels,
        blog_token,
        create_date: post_from_db.create_date,
        update_date: post_from_db.update_date,
    })
}

pub async fn update_post(
    database: &DatabaseConnection,
    UpdatePostDto {
        title,
        content,
        labels,
        token,
        blog_token,
    }: UpdatePostDto,
) -> Result<PostDto, DaoError> {
    let blog_from_db = BlogEntity::find()
        .filter(BlogColumn::Token.eq(&blog_token))
        .one(database)
        .await?
        .ok_or(DaoError::BlogNotFoundByToken(blog_token))?;
    let post_from_db = PostEntity::find()
        .filter(
            PostColumn::Token
                .eq(&token)
                .and(PostColumn::BlogId.eq(blog_from_db.id)),
        )
        .one(database)
        .await?
        .ok_or(DaoError::PostNotFoundByToken(token))?;
    let mut post_from_db = post_from_db.into_active_model();
    if let Some(title) = title {
        post_from_db.title = Set(title);
    }
    if let Some(content) = content {
        post_from_db.content = Set(content);
    }
    if let Some(labels) = labels {
        post_from_db.additional_info = Set(PostAdditionalInfo { labels });
    }
    post_from_db.update_date = Set(Utc::now());
    let post_from_db = database
        .transaction(|txn| Box::pin(async { post_from_db.save(txn).await }))
        .await?;
    let post_from_db = post_from_db.try_into_model()?;
    Ok(PostDto {
        token: post_from_db.token,
        title: post_from_db.title,
        content: post_from_db.content,
        labels: post_from_db.additional_info.labels,
        blog_token: blog_from_db.token,
        create_date: post_from_db.create_date,
        update_date: post_from_db.update_date,
    })
}

pub async fn find_all_posts_by_blog_token(
    database: &DatabaseConnection,
    blog_token: String,
    page_index: u64,
    page_size: u64,
) -> Result<Vec<PostDto>, DaoError> {
    let post_page = PostEntity::find()
        .join(JoinType::InnerJoin, PostRelation::Blog.def())
        .filter(BlogColumn::Token.eq(&blog_token))
        .paginate(database, page_size);
    let post_page = post_page.fetch_page(page_index).await?;
    Ok(post_page
        .into_iter()
        .map(|post| PostDto {
            token: post.token,
            title: post.title,
            content: post.content,
            labels: post.additional_info.labels,
            blog_token: blog_token.clone(),
            create_date: post.create_date,
            update_date: post.update_date,
        })
        .collect())
}
