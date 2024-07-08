use sea_orm_migration::prelude::*;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
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
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(User::DisplayName)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .col(ColumnDef::new(User::RegisterDate).date_time().not_null())
                    .col(ColumnDef::new(User::AdditionalInfo).json())
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
                    .col(ColumnDef::new(Blog::Token).string().unique_key().not_null())
                    .col(ColumnDef::new(Blog::Title).string().not_null())
                    .col(ColumnDef::new(Blog::Summary).string())
                    .col(ColumnDef::new(Blog::CreateDate).date_time().not_null())
                    .col(ColumnDef::new(Blog::UpdateDate).date_time().not_null())
                    .col(ColumnDef::new(Blog::UserId).big_unsigned().not_null())
                    .col(ColumnDef::new(Blog::AdditionalInfo).json())
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
                    .col(ColumnDef::new(Post::Title).string().not_null())
                    .col(ColumnDef::new(Post::Token).string().not_null().unique_key())
                    .col(ColumnDef::new(Post::Content).string())
                    .col(ColumnDef::new(Post::CreateDate).date_time().not_null())
                    .col(ColumnDef::new(Post::UpdateDate).date_time().not_null())
                    .col(ColumnDef::new(Post::BlogId).big_unsigned().not_null())
                    .col(ColumnDef::new(Post::AdditionalInfo).json())
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Blog::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Username,
    DisplayName,
    Password,
    RegisterDate,
    AdditionalInfo,
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
    AdditionalInfo,
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    Token,
    Title,
    Content,
    BlogId,
    CreateDate,
    UpdateDate,
    AdditionalInfo,
}
