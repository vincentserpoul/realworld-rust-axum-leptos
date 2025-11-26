// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct CreateCommentParams<T1: crate::clorinde::StringSql> {
    pub body: T1,
    pub article_id: uuid::Uuid,
    pub author_id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct CreateComment {
    pub id: i32,
    pub body: String,
    pub article_id: uuid::Uuid,
    pub author_id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
pub struct CreateCommentBorrowed<'a> {
    pub id: i32,
    pub body: &'a str,
    pub article_id: uuid::Uuid,
    pub author_id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
impl<'a> From<CreateCommentBorrowed<'a>> for CreateComment {
    fn from(
        CreateCommentBorrowed {
            id,
            body,
            article_id,
            author_id,
            created_at,
            updated_at,
        }: CreateCommentBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            body: body.into(),
            article_id,
            author_id,
            created_at,
            updated_at,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetCommentsByArticle {
    pub id: i32,
    pub body: String,
    pub article_id: uuid::Uuid,
    pub author_id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
pub struct GetCommentsByArticleBorrowed<'a> {
    pub id: i32,
    pub body: &'a str,
    pub article_id: uuid::Uuid,
    pub author_id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
impl<'a> From<GetCommentsByArticleBorrowed<'a>> for GetCommentsByArticle {
    fn from(
        GetCommentsByArticleBorrowed {
            id,
            body,
            article_id,
            author_id,
            created_at,
            updated_at,
        }: GetCommentsByArticleBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            body: body.into(),
            article_id,
            author_id,
            created_at,
            updated_at,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetCommentById {
    pub id: i32,
    pub body: String,
    pub article_id: uuid::Uuid,
    pub author_id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
pub struct GetCommentByIdBorrowed<'a> {
    pub id: i32,
    pub body: &'a str,
    pub article_id: uuid::Uuid,
    pub author_id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
impl<'a> From<GetCommentByIdBorrowed<'a>> for GetCommentById {
    fn from(
        GetCommentByIdBorrowed {
            id,
            body,
            article_id,
            author_id,
            created_at,
            updated_at,
        }: GetCommentByIdBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            body: body.into(),
            article_id,
            author_id,
            created_at,
            updated_at,
        }
    }
}
use crate::clorinde::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct CreateCommentQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<CreateCommentBorrowed, tokio_postgres::Error>,
    mapper: fn(CreateCommentBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> CreateCommentQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(CreateCommentBorrowed) -> R,
    ) -> CreateCommentQuery<'c, 'a, 's, C, R, N> {
        CreateCommentQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::clorinde::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::clorinde::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::clorinde::client::async_::raw(
            self.client,
            self.query,
            crate::clorinde::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct GetCommentsByArticleQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor:
        fn(&tokio_postgres::Row) -> Result<GetCommentsByArticleBorrowed, tokio_postgres::Error>,
    mapper: fn(GetCommentsByArticleBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> GetCommentsByArticleQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(GetCommentsByArticleBorrowed) -> R,
    ) -> GetCommentsByArticleQuery<'c, 'a, 's, C, R, N> {
        GetCommentsByArticleQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::clorinde::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::clorinde::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::clorinde::client::async_::raw(
            self.client,
            self.query,
            crate::clorinde::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct GetCommentByIdQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<GetCommentByIdBorrowed, tokio_postgres::Error>,
    mapper: fn(GetCommentByIdBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> GetCommentByIdQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(GetCommentByIdBorrowed) -> R,
    ) -> GetCommentByIdQuery<'c, 'a, 's, C, R, N> {
        GetCommentByIdQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::clorinde::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::clorinde::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::clorinde::client::async_::raw(
            self.client,
            self.query,
            crate::clorinde::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct CreateCommentStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn create_comment() -> CreateCommentStmt {
    CreateCommentStmt(
        "INSERT INTO comment (body, article_id, author_id, created_at, updated_at) VALUES ($1, $2, $3, $4, $4) RETURNING *",
        None,
    )
}
impl CreateCommentStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::clorinde::StringSql>(
        &'s self,
        client: &'c C,
        body: &'a T1,
        article_id: &'a uuid::Uuid,
        author_id: &'a uuid::Uuid,
        created_at: &'a chrono::DateTime<chrono::FixedOffset>,
    ) -> CreateCommentQuery<'c, 'a, 's, C, CreateComment, 4> {
        CreateCommentQuery {
            client,
            params: [body, article_id, author_id, created_at],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<CreateCommentBorrowed, tokio_postgres::Error> {
                    Ok(CreateCommentBorrowed {
                        id: row.try_get(0)?,
                        body: row.try_get(1)?,
                        article_id: row.try_get(2)?,
                        author_id: row.try_get(3)?,
                        created_at: row.try_get(4)?,
                        updated_at: row.try_get(5)?,
                    })
                },
            mapper: |it| CreateComment::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::clorinde::StringSql>
    crate::clorinde::client::async_::Params<
        'c,
        'a,
        's,
        CreateCommentParams<T1>,
        CreateCommentQuery<'c, 'a, 's, C, CreateComment, 4>,
        C,
    > for CreateCommentStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a CreateCommentParams<T1>,
    ) -> CreateCommentQuery<'c, 'a, 's, C, CreateComment, 4> {
        self.bind(
            client,
            &params.body,
            &params.article_id,
            &params.author_id,
            &params.created_at,
        )
    }
}
pub struct GetCommentsByArticleStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_comments_by_article() -> GetCommentsByArticleStmt {
    GetCommentsByArticleStmt(
        "SELECT * FROM comment WHERE article_id = $1 ORDER BY created_at DESC",
        None,
    )
}
impl GetCommentsByArticleStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
        article_id: &'a uuid::Uuid,
    ) -> GetCommentsByArticleQuery<'c, 'a, 's, C, GetCommentsByArticle, 1> {
        GetCommentsByArticleQuery {
            client,
            params: [article_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<GetCommentsByArticleBorrowed, tokio_postgres::Error> {
                Ok(GetCommentsByArticleBorrowed {
                    id: row.try_get(0)?,
                    body: row.try_get(1)?,
                    article_id: row.try_get(2)?,
                    author_id: row.try_get(3)?,
                    created_at: row.try_get(4)?,
                    updated_at: row.try_get(5)?,
                })
            },
            mapper: |it| GetCommentsByArticle::from(it),
        }
    }
}
pub struct DeleteCommentStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn delete_comment() -> DeleteCommentStmt {
    DeleteCommentStmt("DELETE FROM comment WHERE id = $1", None)
}
impl DeleteCommentStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub async fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
        id: &'a i32,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[id]).await
    }
}
pub struct GetCommentByIdStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_comment_by_id() -> GetCommentByIdStmt {
    GetCommentByIdStmt("SELECT * FROM comment WHERE id = $1", None)
}
impl GetCommentByIdStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
        id: &'a i32,
    ) -> GetCommentByIdQuery<'c, 'a, 's, C, GetCommentById, 1> {
        GetCommentByIdQuery {
            client,
            params: [id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<GetCommentByIdBorrowed, tokio_postgres::Error> {
                Ok(GetCommentByIdBorrowed {
                    id: row.try_get(0)?,
                    body: row.try_get(1)?,
                    article_id: row.try_get(2)?,
                    author_id: row.try_get(3)?,
                    created_at: row.try_get(4)?,
                    updated_at: row.try_get(5)?,
                })
            },
            mapper: |it| GetCommentById::from(it),
        }
    }
}
