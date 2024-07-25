use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, FromQueryResult, IntoActiveModel,
    PaginatorTrait, QueryFilter, QuerySelect, RelationTrait, TransactionTrait, TryIntoModel,
};
use sea_orm::ActiveValue::Set;
use sea_orm::JoinType;
use sea_orm::sea_query::{Alias, Expr, Query};
use uuid::Uuid;
use ppaass_blog_domain::entity::{
    BlogColumn, BlogEntity, LabelColumn, LabelEntity, PostActiveModel, PostColumn, PostEntity,
    PostLabelColumn, PostLabelEntity, PostRelation,
};
use crate::dao::label::save_all_label;
use crate::dao::PageDto;
use crate::dao::post_label::{find_labels_by_post, save_post_label};
use crate::dto::post::{CreatePostDto, PostDto, UpdatePostDto};
use crate::error::DaoError;
pub async fn create_post<C: ConnectionTrait + TransactionTrait>(
    database: &C,
    CreatePostDto {
        title,
        summary,
        content,
        labels,
        blog_token,
    }: CreatePostDto,
) -> Result<PostDto, DaoError> {
    let labels_clone = labels.clone();
    let blog_token_clone = blog_token.clone();
    let post_from_db = database
        .transaction(|txn| {
            Box::pin(async move {
                let blog_from_db = BlogEntity::find()
                    .filter(BlogColumn::Token.eq(&blog_token))
                    .one(txn)
                    .await?
                    .ok_or(DaoError::BlogNotFoundByToken(blog_token.clone()))?;
                let post_to_db = PostActiveModel {
                    token: Set(Uuid::new_v4().to_string()),
                    title: Set(title),
                    summary: Set(summary),
                    content: Set(content),
                    blog_id: Set(blog_from_db.id),
                    create_date: Set(Utc::now()),
                    update_date: Set(Utc::now()),
                    ..Default::default()
                };
                let post_from_db = post_to_db.save(txn).await?;
                let post_from_db = post_from_db.try_into_model()?;
                let label_ids = save_all_label(txn, labels).await?;
                for label_id in label_ids {
                    save_post_label(txn, post_from_db.id, label_id).await?;
                }
                Ok(post_from_db)
            })
        })
        .await?;

    Ok(PostDto {
        token: post_from_db.token,
        title: post_from_db.title,
        content: post_from_db.content,
        summary: post_from_db.summary,
        blog_token: blog_token_clone,
        create_date: post_from_db.create_date,
        update_date: post_from_db.update_date,
    })
}

pub async fn update_post<C: ConnectionTrait + TransactionTrait>(
    database: &C,
    UpdatePostDto {
        title,
        content,
        labels,
        token,
        blog_token,
    }: UpdatePostDto,
) -> Result<PostDto, DaoError> {
    let (post_from_db, blog_token) = database
        .transaction(|txn| {
            Box::pin(async move {
                let blog_from_db = BlogEntity::find()
                    .filter(BlogColumn::Token.eq(&blog_token))
                    .one(txn)
                    .await?
                    .ok_or(DaoError::BlogNotFoundByToken(blog_token))?;
                let post_from_db = PostEntity::find()
                    .filter(
                        PostColumn::Token
                            .eq(&token)
                            .and(PostColumn::BlogId.eq(blog_from_db.id)),
                    )
                    .one(txn)
                    .await?
                    .ok_or(DaoError::PostNotFoundByToken(token))?;
                let mut post_from_db = post_from_db.into_active_model();
                if let Some(title) = title {
                    post_from_db.title = Set(title);
                }
                if let Some(content) = content {
                    post_from_db.content = Set(content);
                }

                post_from_db.update_date = Set(Utc::now());
                let post_from_db = post_from_db.save(txn).await?;
                let post_from_db = post_from_db.try_into_model()?;

                if let Some(labels) = labels {
                    let label_ids = save_all_label(txn, labels).await?;
                    for label_id in label_ids {
                        save_post_label(txn, post_from_db.id, label_id).await?;
                    }
                }
                Ok((post_from_db, blog_from_db.token))
            })
        })
        .await?;
    let labels = find_labels_by_post(database, post_from_db.id).await?;
    Ok(PostDto {
        token: post_from_db.token,
        title: post_from_db.title,
        content: post_from_db.content,
        summary: post_from_db.summary,
        blog_token,
        create_date: post_from_db.create_date,
        update_date: post_from_db.update_date,
    })
}

pub async fn find_all_posts_by_blog_token<C: ConnectionTrait>(
    database: &C,
    blog_token: String,
    page_index: u64,
    page_size: u64,
) -> Result<PageDto<PostDto>, DaoError> {
    let post_page = PostEntity::find()
        .join(JoinType::InnerJoin, PostRelation::Blog.def())
        .filter(BlogColumn::Token.eq(&blog_token))
        .paginate(database, page_size);
    let page_number = post_page.num_pages().await?;
    let posts = post_page.fetch_page(page_index).await?;
    let mut post_dto_list = Vec::new();
    for post in posts {
        let labels = find_labels_by_post(database, post.id).await?;
        let post_dto = PostDto {
            token: post.token,
            title: post.title,
            content: post.content,
            blog_token: blog_token.clone(),
            create_date: post.create_date,
            update_date: post.update_date,
            summary: post.summary,
        };
        post_dto_list.push(post_dto);
    }

    Ok(PageDto {
        items: post_dto_list,
        page_size,
        page_number,
        page_index,
    })
}

pub async fn find_all_posts_by_labels<C: ConnectionTrait>(
    database: &C,
    labels: Vec<String>,
    page_index: u64,
    page_size: u64,
) -> Result<PageDto<PostDto>, DaoError> {
    let final_posts_by_labels_query_statement = Query::select()
        .column((PostEntity, PostColumn::Token))
        .column((PostEntity, PostColumn::Title))
        .column((PostEntity, PostColumn::Summary))
        .column((PostEntity, PostColumn::Content))
        .column((PostEntity, PostColumn::CreateDate))
        .column((PostEntity, PostColumn::UpdateDate))
        .expr_as(
            Expr::col((BlogEntity, BlogColumn::Token)),
            Alias::new("blog_token".to_string()),
        )
        .from(PostEntity)
        .from(LabelEntity)
        .from(PostLabelEntity)
        .from(BlogEntity)
        .and_where(
            Expr::col((PostEntity, PostColumn::Id))
                .equals((PostLabelEntity, PostLabelColumn::PostId)),
        )
        .and_where(
            Expr::col((LabelEntity, LabelColumn::Id))
                .equals((PostLabelEntity, PostLabelColumn::LabelId)),
        )
        .and_where(Expr::col((PostEntity, PostColumn::BlogId)).equals((BlogEntity, BlogColumn::Id)))
        .and_where(Expr::col((LabelEntity, LabelColumn::Text)).is_in(labels))
        .to_owned();

    let database_backend = database.get_database_backend();
    let result_paginate =
        PostDto::find_by_statement(database_backend.build(&final_posts_by_labels_query_statement))
            .paginate(database, page_size);
    let page_number = result_paginate.num_pages().await?;
    let result_page = result_paginate.fetch_page(page_index).await?;
    Ok(PageDto {
        items: result_page,
        page_size,
        page_number,
        page_index,
    })
}
