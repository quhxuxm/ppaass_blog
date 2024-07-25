use sea_orm_migration::prelude::*;
use ppaass_blog_persistence::dao::blog::create_blog;
use ppaass_blog_persistence::dao::label::save_label;
use ppaass_blog_persistence::dao::post::create_post;
use ppaass_blog_persistence::dao::user::create_user;
use ppaass_blog_persistence::dto::blog::CreateBlogDto;
use ppaass_blog_persistence::dto::post::CreatePostDto;
use ppaass_blog_persistence::dto::user::CreateUserDto;
#[derive(DeriveMigrationName)]
pub struct Migration;

impl Migration {
    async fn initialize_table<'c>(manager: &SchemaManager<'c>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Label::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Label::Id)
                            .big_unsigned()
                            .primary_key()
                            .not_null()
                            .auto_increment(),
                    )
                    .col(
                        ColumnDef::new(Label::Text)
                            .string()
                            .unique_key()
                            .not_null()
                            .string_len(8),
                    )
                    .col(ColumnDef::new(Label::CreateDate).date_time().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .big_unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(User::Username)
                            .string()
                            .not_null()
                            .unique_key()
                            .string_len(16),
                    )
                    .col(
                        ColumnDef::new(User::DisplayName)
                            .string()
                            .not_null()
                            .unique_key()
                            .string_len(16),
                    )
                    .col(
                        ColumnDef::new(User::Password)
                            .string()
                            .not_null()
                            .string_len(32),
                    )
                    .col(ColumnDef::new(User::RegisterDate).date_time().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(UserLabel::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(UserLabel::UserId).big_unsigned().not_null())
                    .col(ColumnDef::new(UserLabel::LabelId).big_unsigned().not_null())
                    .primary_key(
                        Index::create()
                            .col(UserLabel::UserId)
                            .col(UserLabel::LabelId),
                    )
                    .col(ColumnDef::new(UserLabel::CreateDate).date_time().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Blog::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Blog::Id)
                            .big_unsigned()
                            .primary_key()
                            .not_null()
                            .auto_increment(),
                    )
                    .col(
                        ColumnDef::new(Blog::Token)
                            .string()
                            .unique_key()
                            .not_null()
                            .string_len(256),
                    )
                    .col(
                        ColumnDef::new(Blog::Title)
                            .string()
                            .not_null()
                            .string_len(32),
                    )
                    .col(ColumnDef::new(Blog::Summary).string().string_len(256))
                    .col(ColumnDef::new(Blog::CreateDate).date_time().not_null())
                    .col(ColumnDef::new(Blog::UpdateDate).date_time().not_null())
                    .col(ColumnDef::new(Blog::UserId).big_unsigned().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(BlogLabel::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(BlogLabel::BlogId).big_unsigned().not_null())
                    .col(ColumnDef::new(BlogLabel::LabelId).big_unsigned().not_null())
                    .primary_key(
                        Index::create()
                            .col(BlogLabel::BlogId)
                            .col(BlogLabel::LabelId),
                    )
                    .col(ColumnDef::new(BlogLabel::CreateDate).date_time().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Post::Id)
                            .big_unsigned()
                            .primary_key()
                            .not_null()
                            .auto_increment(),
                    )
                    .col(
                        ColumnDef::new(Post::Title)
                            .string()
                            .not_null()
                            .string_len(32),
                    )
                    .col(
                        ColumnDef::new(Post::Summary)
                            .string()
                            .not_null()
                            .string_len(256),
                    )
                    .col(
                        ColumnDef::new(Post::Token)
                            .string()
                            .not_null()
                            .unique_key()
                            .string_len(256),
                    )
                    .col(
                        ColumnDef::new(Post::Content)
                            .string()
                            .not_null()
                            .string_len(4096),
                    )
                    .col(ColumnDef::new(Post::CreateDate).date_time().not_null())
                    .col(ColumnDef::new(Post::UpdateDate).date_time().not_null())
                    .col(ColumnDef::new(Post::BlogId).big_unsigned().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(PostLabel::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(PostLabel::PostId).big_unsigned().not_null())
                    .col(ColumnDef::new(PostLabel::LabelId).big_unsigned().not_null())
                    .primary_key(
                        Index::create()
                            .col(PostLabel::PostId)
                            .col(PostLabel::LabelId),
                    )
                    .col(ColumnDef::new(PostLabel::CreateDate).date_time().not_null())
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn generate_random_labels(number: u32, total_label_number: u32) -> Vec<String> {
        let mut random_labels = Vec::new();
        for _ in 0..number {
            let random_label = format!("LB{}", rand::random::<u32>() % total_label_number);
            random_labels.push(random_label);
        }
        random_labels
    }
    async fn generate_seed_label<'c>(
        manager: &SchemaManager<'c>,
        seed_label_number: u32,
    ) -> Result<(), DbErr> {
        for i in 0..seed_label_number {
            save_label(manager.get_connection(), format!("LB{i}")).await?;
        }
        Ok(())
    }
    async fn generate_seed_user<'c>(
        manager: &SchemaManager<'c>,
        seed_label_number: u32,
        user_index: u32,
        seed_blog_per_user_number: u32,
        seed_post_per_blog_number: u32,
        label_per_seed_number: u32,
    ) -> Result<(), DbErr> {
        let random_labels =
            Self::generate_random_labels(label_per_seed_number, seed_label_number).await;
        create_user(
            manager.get_connection(),
            CreateUserDto {
                username: format!("quhao{user_index}"),
                password: format!("quhao{user_index}"),
                display_name: format!("Qu Hao {user_index}"),
                labels: random_labels,
            },
        )
        .await?;

        for b in 0..seed_blog_per_user_number {
            let random_labels =
                Self::generate_random_labels(label_per_seed_number, seed_label_number).await;
            let blog = create_blog(
                manager.get_connection(),
                CreateBlogDto {
                    title: format!("quhao{user_index} blog title {b}"),
                    summary: format!("quhao{user_index} blog summary {b}"),
                    labels: random_labels,
                    username: format!("quhao{user_index}"),
                },
            )
            .await?;
            for p in 0..seed_post_per_blog_number {
                let random_labels =
                    Self::generate_random_labels(label_per_seed_number, seed_label_number).await;
                create_post(
                    manager.get_connection(),
                    CreatePostDto {
                        title: format!("quhao{user_index} blog {b} post title {p}"),
                        content: format!("quhao{user_index} blog {b} post content {p}"),
                        summary: format!("quhao{user_index} blog {b} post summary {p}"),
                        labels: random_labels,
                        blog_token: blog.token.clone(),
                    },
                )
                .await?;
            }
        }
        Ok(())
    }

    async fn initialize_seed_data<'c>(
        manager: &SchemaManager<'c>,
        seed_label_number: u32,
        seed_user_number: u32,
        seed_blog_per_user_number: u32,
        seed_post_per_blog_number: u32,
        label_per_seed_number: u32,
    ) -> Result<(), DbErr> {
        Self::generate_seed_label(manager, seed_label_number).await?;

        for user_index in 0..seed_user_number {
            if let Err(e) = Self::generate_seed_user(
                manager,
                seed_label_number,
                user_index,
                seed_blog_per_user_number,
                seed_post_per_blog_number,
                label_per_seed_number,
            )
            .await
            {
                println!("Fail to generate user [{user_index}] because of error: {e:?}");
                continue;
            }
            println!("Success to generate user [{user_index}] seed data.");
        }
        Ok(())
    }
}
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        if let Err(e) = Self::initialize_table(manager).await {
            println!("Fail to initialize table because of error: {e:?}");
        };
        if let Err(e) = Self::initialize_seed_data(manager, 100, 100, 20, 20, 10).await {
            println!("Fail to initialize seed data because of error: {e:?}");
        };
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(UserLabel::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(BlogLabel::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(PostLabel::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Blog::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Label::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Label {
    Table,
    Id,
    Text,
    CreateDate,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Username,
    DisplayName,
    Password,
    RegisterDate,
}

#[derive(DeriveIden)]
enum UserLabel {
    Table,
    UserId,
    LabelId,
    CreateDate,
}

#[derive(DeriveIden)]
enum Blog {
    Table,
    Id,
    Token,
    Title,
    Summary,
    CreateDate,
    UpdateDate,
    UserId,
}

#[derive(DeriveIden)]
enum BlogLabel {
    Table,
    BlogId,
    LabelId,
    CreateDate,
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    Token,
    Title,
    Summary,
    Content,
    BlogId,
    CreateDate,
    UpdateDate,
}

#[derive(DeriveIden)]
enum PostLabel {
    Table,
    PostId,
    LabelId,
    CreateDate,
}
