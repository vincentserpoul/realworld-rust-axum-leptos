// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct CreateArticleParams<
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
> {
    pub id: uuid::Uuid,
    pub slug: T1,
    pub title: T2,
    pub description: T3,
    pub body: T4,
    pub author_id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}
#[derive(Debug)]
pub struct UpdateArticleParams<
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
> {
    pub slug: T1,
    pub title: T2,
    pub description: T3,
    pub body: T4,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
    pub id: uuid::Uuid,
}
#[derive(Clone, Copy, Debug)]
pub struct FavoriteArticleParams {
    pub user_id: uuid::Uuid,
    pub article_id: uuid::Uuid,
}
#[derive(Clone, Copy, Debug)]
pub struct UnfavoriteArticleParams {
    pub user_id: uuid::Uuid,
    pub article_id: uuid::Uuid,
}
#[derive(Clone, Copy, Debug)]
pub struct IsFavoritedParams {
    pub user_id: uuid::Uuid,
    pub article_id: uuid::Uuid,
}
#[derive(Debug)]
pub struct CreateTagParams<T1: crate::StringSql> {
    pub id: uuid::Uuid,
    pub name: T1,
}
#[derive(Clone, Copy, Debug)]
pub struct AddTagToArticleParams {
    pub article_id: uuid::Uuid,
    pub tag_id: uuid::Uuid,
}
#[derive(Debug)]
pub struct ListArticlesParams<T1: crate::StringSql, T2: crate::StringSql, T3: crate::StringSql> {
    pub viewer_id: uuid::Uuid,
    pub author: T1,
    pub tag: T2,
    pub favorited: T3,
    pub limit: i64,
    pub offset: i64,
}
#[derive(Debug)]
pub struct CountArticlesParams<T1: crate::StringSql, T2: crate::StringSql, T3: crate::StringSql> {
    pub author: T1,
    pub tag: T2,
    pub favorited: T3,
}
#[derive(Clone, Copy, Debug)]
pub struct FeedArticlesParams {
    pub viewer_id: uuid::Uuid,
    pub limit: i64,
    pub offset: i64,
}
#[derive(Debug, Clone, PartialEq)]
pub struct CreateArticle {
    pub id: uuid::Uuid,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub author_id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
pub struct CreateArticleBorrowed<'a> {
    pub id: uuid::Uuid,
    pub slug: &'a str,
    pub title: &'a str,
    pub description: &'a str,
    pub body: &'a str,
    pub author_id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
impl<'a> From<CreateArticleBorrowed<'a>> for CreateArticle {
    fn from(
        CreateArticleBorrowed {
            id,
            slug,
            title,
            description,
            body,
            author_id,
            created_at,
            updated_at,
        }: CreateArticleBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            slug: slug.into(),
            title: title.into(),
            description: description.into(),
            body: body.into(),
            author_id,
            created_at,
            updated_at,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetArticleBySlug {
    pub id: uuid::Uuid,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub author_id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
pub struct GetArticleBySlugBorrowed<'a> {
    pub id: uuid::Uuid,
    pub slug: &'a str,
    pub title: &'a str,
    pub description: &'a str,
    pub body: &'a str,
    pub author_id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
impl<'a> From<GetArticleBySlugBorrowed<'a>> for GetArticleBySlug {
    fn from(
        GetArticleBySlugBorrowed {
            id,
            slug,
            title,
            description,
            body,
            author_id,
            created_at,
            updated_at,
        }: GetArticleBySlugBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            slug: slug.into(),
            title: title.into(),
            description: description.into(),
            body: body.into(),
            author_id,
            created_at,
            updated_at,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetArticleById {
    pub id: uuid::Uuid,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub author_id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
pub struct GetArticleByIdBorrowed<'a> {
    pub id: uuid::Uuid,
    pub slug: &'a str,
    pub title: &'a str,
    pub description: &'a str,
    pub body: &'a str,
    pub author_id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
impl<'a> From<GetArticleByIdBorrowed<'a>> for GetArticleById {
    fn from(
        GetArticleByIdBorrowed {
            id,
            slug,
            title,
            description,
            body,
            author_id,
            created_at,
            updated_at,
        }: GetArticleByIdBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            slug: slug.into(),
            title: title.into(),
            description: description.into(),
            body: body.into(),
            author_id,
            created_at,
            updated_at,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct UpdateArticle {
    pub id: uuid::Uuid,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub author_id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
pub struct UpdateArticleBorrowed<'a> {
    pub id: uuid::Uuid,
    pub slug: &'a str,
    pub title: &'a str,
    pub description: &'a str,
    pub body: &'a str,
    pub author_id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
impl<'a> From<UpdateArticleBorrowed<'a>> for UpdateArticle {
    fn from(
        UpdateArticleBorrowed {
            id,
            slug,
            title,
            description,
            body,
            author_id,
            created_at,
            updated_at,
        }: UpdateArticleBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            slug: slug.into(),
            title: title.into(),
            description: description.into(),
            body: body.into(),
            author_id,
            created_at,
            updated_at,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetTagByName {
    pub id: uuid::Uuid,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}
pub struct GetTagByNameBorrowed<'a> {
    pub id: uuid::Uuid,
    pub name: &'a str,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}
impl<'a> From<GetTagByNameBorrowed<'a>> for GetTagByName {
    fn from(
        GetTagByNameBorrowed {
            id,
            name,
            created_at,
        }: GetTagByNameBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            created_at,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ListArticles {
    pub id: uuid::Uuid,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub author_id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
    pub author_username: String,
    pub author_bio: String,
    pub author_image: String,
    pub following_author: bool,
    pub favorited: bool,
    pub favorites_count: i64,
    pub tag_list: Vec<String>,
}
pub struct ListArticlesBorrowed<'a> {
    pub id: uuid::Uuid,
    pub slug: &'a str,
    pub title: &'a str,
    pub description: &'a str,
    pub body: &'a str,
    pub author_id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
    pub author_username: &'a str,
    pub author_bio: &'a str,
    pub author_image: &'a str,
    pub following_author: bool,
    pub favorited: bool,
    pub favorites_count: i64,
    pub tag_list: crate::ArrayIterator<'a, &'a str>,
}
impl<'a> From<ListArticlesBorrowed<'a>> for ListArticles {
    fn from(
        ListArticlesBorrowed {
            id,
            slug,
            title,
            description,
            body,
            author_id,
            created_at,
            updated_at,
            author_username,
            author_bio,
            author_image,
            following_author,
            favorited,
            favorites_count,
            tag_list,
        }: ListArticlesBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            slug: slug.into(),
            title: title.into(),
            description: description.into(),
            body: body.into(),
            author_id,
            created_at,
            updated_at,
            author_username: author_username.into(),
            author_bio: author_bio.into(),
            author_image: author_image.into(),
            following_author,
            favorited,
            favorites_count,
            tag_list: tag_list.map(|v| v.into()).collect(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct FeedArticles {
    pub id: uuid::Uuid,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub author_id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
    pub author_username: String,
    pub author_bio: String,
    pub author_image: String,
    pub following_author: bool,
    pub favorited: bool,
    pub favorites_count: i64,
    pub tag_list: Vec<String>,
}
pub struct FeedArticlesBorrowed<'a> {
    pub id: uuid::Uuid,
    pub slug: &'a str,
    pub title: &'a str,
    pub description: &'a str,
    pub body: &'a str,
    pub author_id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
    pub author_username: &'a str,
    pub author_bio: &'a str,
    pub author_image: &'a str,
    pub following_author: bool,
    pub favorited: bool,
    pub favorites_count: i64,
    pub tag_list: crate::ArrayIterator<'a, &'a str>,
}
impl<'a> From<FeedArticlesBorrowed<'a>> for FeedArticles {
    fn from(
        FeedArticlesBorrowed {
            id,
            slug,
            title,
            description,
            body,
            author_id,
            created_at,
            updated_at,
            author_username,
            author_bio,
            author_image,
            following_author,
            favorited,
            favorites_count,
            tag_list,
        }: FeedArticlesBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            slug: slug.into(),
            title: title.into(),
            description: description.into(),
            body: body.into(),
            author_id,
            created_at,
            updated_at,
            author_username: author_username.into(),
            author_bio: author_bio.into(),
            author_image: author_image.into(),
            following_author,
            favorited,
            favorites_count,
            tag_list: tag_list.map(|v| v.into()).collect(),
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct CreateArticleQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<CreateArticleBorrowed, tokio_postgres::Error>,
    mapper: fn(CreateArticleBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> CreateArticleQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(CreateArticleBorrowed) -> R,
    ) -> CreateArticleQuery<'c, 'a, 's, C, R, N> {
        CreateArticleQuery {
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
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
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
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
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
pub struct GetArticleBySlugQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<GetArticleBySlugBorrowed, tokio_postgres::Error>,
    mapper: fn(GetArticleBySlugBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> GetArticleBySlugQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(GetArticleBySlugBorrowed) -> R,
    ) -> GetArticleBySlugQuery<'c, 'a, 's, C, R, N> {
        GetArticleBySlugQuery {
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
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
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
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
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
pub struct GetArticleByIdQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<GetArticleByIdBorrowed, tokio_postgres::Error>,
    mapper: fn(GetArticleByIdBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> GetArticleByIdQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(GetArticleByIdBorrowed) -> R,
    ) -> GetArticleByIdQuery<'c, 'a, 's, C, R, N> {
        GetArticleByIdQuery {
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
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
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
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
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
pub struct UpdateArticleQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<UpdateArticleBorrowed, tokio_postgres::Error>,
    mapper: fn(UpdateArticleBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> UpdateArticleQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(UpdateArticleBorrowed) -> R,
    ) -> UpdateArticleQuery<'c, 'a, 's, C, R, N> {
        UpdateArticleQuery {
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
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
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
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
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
pub struct BoolQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<bool, tokio_postgres::Error>,
    mapper: fn(bool) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> BoolQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(bool) -> R) -> BoolQuery<'c, 'a, 's, C, R, N> {
        BoolQuery {
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
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
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
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
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
pub struct I64Query<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<i64, tokio_postgres::Error>,
    mapper: fn(i64) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> I64Query<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(i64) -> R) -> I64Query<'c, 'a, 's, C, R, N> {
        I64Query {
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
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
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
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
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
pub struct StringQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<&str, tokio_postgres::Error>,
    mapper: fn(&str) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> StringQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(&str) -> R) -> StringQuery<'c, 'a, 's, C, R, N> {
        StringQuery {
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
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
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
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
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
pub struct GetTagByNameQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<GetTagByNameBorrowed, tokio_postgres::Error>,
    mapper: fn(GetTagByNameBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> GetTagByNameQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(GetTagByNameBorrowed) -> R,
    ) -> GetTagByNameQuery<'c, 'a, 's, C, R, N> {
        GetTagByNameQuery {
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
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
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
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
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
pub struct ListArticlesQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<ListArticlesBorrowed, tokio_postgres::Error>,
    mapper: fn(ListArticlesBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ListArticlesQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ListArticlesBorrowed) -> R,
    ) -> ListArticlesQuery<'c, 'a, 's, C, R, N> {
        ListArticlesQuery {
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
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
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
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
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
pub struct FeedArticlesQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<FeedArticlesBorrowed, tokio_postgres::Error>,
    mapper: fn(FeedArticlesBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> FeedArticlesQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(FeedArticlesBorrowed) -> R,
    ) -> FeedArticlesQuery<'c, 'a, 's, C, R, N> {
        FeedArticlesQuery {
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
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
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
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
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
pub struct CreateArticleStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn create_article() -> CreateArticleStmt {
    CreateArticleStmt(
        "INSERT INTO article (id, slug, title, description, body, author_id, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $7) RETURNING *",
        None,
    )
}
impl CreateArticleStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<
        'c,
        'a,
        's,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
        T4: crate::StringSql,
    >(
        &'s self,
        client: &'c C,
        id: &'a uuid::Uuid,
        slug: &'a T1,
        title: &'a T2,
        description: &'a T3,
        body: &'a T4,
        author_id: &'a uuid::Uuid,
        created_at: &'a chrono::DateTime<chrono::FixedOffset>,
    ) -> CreateArticleQuery<'c, 'a, 's, C, CreateArticle, 7> {
        CreateArticleQuery {
            client,
            params: [id, slug, title, description, body, author_id, created_at],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<CreateArticleBorrowed, tokio_postgres::Error> {
                    Ok(CreateArticleBorrowed {
                        id: row.try_get(0)?,
                        slug: row.try_get(1)?,
                        title: row.try_get(2)?,
                        description: row.try_get(3)?,
                        body: row.try_get(4)?,
                        author_id: row.try_get(5)?,
                        created_at: row.try_get(6)?,
                        updated_at: row.try_get(7)?,
                    })
                },
            mapper: |it| CreateArticle::from(it),
        }
    }
}
impl<
    'c,
    'a,
    's,
    C: GenericClient,
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        CreateArticleParams<T1, T2, T3, T4>,
        CreateArticleQuery<'c, 'a, 's, C, CreateArticle, 7>,
        C,
    > for CreateArticleStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a CreateArticleParams<T1, T2, T3, T4>,
    ) -> CreateArticleQuery<'c, 'a, 's, C, CreateArticle, 7> {
        self.bind(
            client,
            &params.id,
            &params.slug,
            &params.title,
            &params.description,
            &params.body,
            &params.author_id,
            &params.created_at,
        )
    }
}
pub struct GetArticleBySlugStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_article_by_slug() -> GetArticleBySlugStmt {
    GetArticleBySlugStmt("SELECT * FROM article WHERE slug = $1", None)
}
impl GetArticleBySlugStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s self,
        client: &'c C,
        slug: &'a T1,
    ) -> GetArticleBySlugQuery<'c, 'a, 's, C, GetArticleBySlug, 1> {
        GetArticleBySlugQuery {
            client,
            params: [slug],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<GetArticleBySlugBorrowed, tokio_postgres::Error> {
                Ok(GetArticleBySlugBorrowed {
                    id: row.try_get(0)?,
                    slug: row.try_get(1)?,
                    title: row.try_get(2)?,
                    description: row.try_get(3)?,
                    body: row.try_get(4)?,
                    author_id: row.try_get(5)?,
                    created_at: row.try_get(6)?,
                    updated_at: row.try_get(7)?,
                })
            },
            mapper: |it| GetArticleBySlug::from(it),
        }
    }
}
pub struct GetArticleByIdStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_article_by_id() -> GetArticleByIdStmt {
    GetArticleByIdStmt("SELECT * FROM article WHERE id = $1", None)
}
impl GetArticleByIdStmt {
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
        id: &'a uuid::Uuid,
    ) -> GetArticleByIdQuery<'c, 'a, 's, C, GetArticleById, 1> {
        GetArticleByIdQuery {
            client,
            params: [id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<GetArticleByIdBorrowed, tokio_postgres::Error> {
                Ok(GetArticleByIdBorrowed {
                    id: row.try_get(0)?,
                    slug: row.try_get(1)?,
                    title: row.try_get(2)?,
                    description: row.try_get(3)?,
                    body: row.try_get(4)?,
                    author_id: row.try_get(5)?,
                    created_at: row.try_get(6)?,
                    updated_at: row.try_get(7)?,
                })
            },
            mapper: |it| GetArticleById::from(it),
        }
    }
}
pub struct UpdateArticleStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn update_article() -> UpdateArticleStmt {
    UpdateArticleStmt(
        "UPDATE article SET slug = COALESCE($1, slug), title = COALESCE($2, title), description = COALESCE($3, description), body = COALESCE($4, body), updated_at = $5 WHERE id = $6 RETURNING *",
        None,
    )
}
impl UpdateArticleStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<
        'c,
        'a,
        's,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
        T4: crate::StringSql,
    >(
        &'s self,
        client: &'c C,
        slug: &'a T1,
        title: &'a T2,
        description: &'a T3,
        body: &'a T4,
        updated_at: &'a chrono::DateTime<chrono::FixedOffset>,
        id: &'a uuid::Uuid,
    ) -> UpdateArticleQuery<'c, 'a, 's, C, UpdateArticle, 6> {
        UpdateArticleQuery {
            client,
            params: [slug, title, description, body, updated_at, id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<UpdateArticleBorrowed, tokio_postgres::Error> {
                    Ok(UpdateArticleBorrowed {
                        id: row.try_get(0)?,
                        slug: row.try_get(1)?,
                        title: row.try_get(2)?,
                        description: row.try_get(3)?,
                        body: row.try_get(4)?,
                        author_id: row.try_get(5)?,
                        created_at: row.try_get(6)?,
                        updated_at: row.try_get(7)?,
                    })
                },
            mapper: |it| UpdateArticle::from(it),
        }
    }
}
impl<
    'c,
    'a,
    's,
    C: GenericClient,
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        UpdateArticleParams<T1, T2, T3, T4>,
        UpdateArticleQuery<'c, 'a, 's, C, UpdateArticle, 6>,
        C,
    > for UpdateArticleStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a UpdateArticleParams<T1, T2, T3, T4>,
    ) -> UpdateArticleQuery<'c, 'a, 's, C, UpdateArticle, 6> {
        self.bind(
            client,
            &params.slug,
            &params.title,
            &params.description,
            &params.body,
            &params.updated_at,
            &params.id,
        )
    }
}
pub struct DeleteArticleStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn delete_article() -> DeleteArticleStmt {
    DeleteArticleStmt("DELETE FROM article WHERE id = $1", None)
}
impl DeleteArticleStmt {
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
        id: &'a uuid::Uuid,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[id]).await
    }
}
pub struct FavoriteArticleStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn favorite_article() -> FavoriteArticleStmt {
    FavoriteArticleStmt(
        "INSERT INTO article_favorite (appuser_id, article_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
        None,
    )
}
impl FavoriteArticleStmt {
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
        user_id: &'a uuid::Uuid,
        article_id: &'a uuid::Uuid,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[user_id, article_id]).await
    }
}
impl<'a, C: GenericClient + Send + Sync>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        FavoriteArticleParams,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for FavoriteArticleStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a FavoriteArticleParams,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.user_id, &params.article_id))
    }
}
pub struct UnfavoriteArticleStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn unfavorite_article() -> UnfavoriteArticleStmt {
    UnfavoriteArticleStmt(
        "DELETE FROM article_favorite WHERE appuser_id = $1 AND article_id = $2",
        None,
    )
}
impl UnfavoriteArticleStmt {
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
        user_id: &'a uuid::Uuid,
        article_id: &'a uuid::Uuid,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[user_id, article_id]).await
    }
}
impl<'a, C: GenericClient + Send + Sync>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        UnfavoriteArticleParams,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for UnfavoriteArticleStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a UnfavoriteArticleParams,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.user_id, &params.article_id))
    }
}
pub struct IsFavoritedStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn is_favorited() -> IsFavoritedStmt {
    IsFavoritedStmt(
        "SELECT EXISTS( SELECT 1 FROM article_favorite WHERE appuser_id = $1 AND article_id = $2 )",
        None,
    )
}
impl IsFavoritedStmt {
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
        user_id: &'a uuid::Uuid,
        article_id: &'a uuid::Uuid,
    ) -> BoolQuery<'c, 'a, 's, C, bool, 2> {
        BoolQuery {
            client,
            params: [user_id, article_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it,
        }
    }
}
impl<'c, 'a, 's, C: GenericClient>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        IsFavoritedParams,
        BoolQuery<'c, 'a, 's, C, bool, 2>,
        C,
    > for IsFavoritedStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a IsFavoritedParams,
    ) -> BoolQuery<'c, 'a, 's, C, bool, 2> {
        self.bind(client, &params.user_id, &params.article_id)
    }
}
pub struct GetFavoritesCountStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_favorites_count() -> GetFavoritesCountStmt {
    GetFavoritesCountStmt(
        "SELECT COUNT(*) as count FROM article_favorite WHERE article_id = $1",
        None,
    )
}
impl GetFavoritesCountStmt {
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
    ) -> I64Query<'c, 'a, 's, C, i64, 1> {
        I64Query {
            client,
            params: [article_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it,
        }
    }
}
pub struct GetTagsStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_tags() -> GetTagsStmt {
    GetTagsStmt("SELECT name FROM tag", None)
}
impl GetTagsStmt {
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
    ) -> StringQuery<'c, 'a, 's, C, String, 0> {
        StringQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it.into(),
        }
    }
}
pub struct CreateTagStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn create_tag() -> CreateTagStmt {
    CreateTagStmt(
        "INSERT INTO tag (id, name) VALUES ($1, $2) ON CONFLICT (name) DO NOTHING",
        None,
    )
}
impl CreateTagStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s self,
        client: &'c C,
        id: &'a uuid::Uuid,
        name: &'a T1,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[id, name]).await
    }
}
impl<'a, C: GenericClient + Send + Sync, T1: crate::StringSql>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        CreateTagParams<T1>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for CreateTagStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a CreateTagParams<T1>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.id, &params.name))
    }
}
pub struct GetTagByNameStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_tag_by_name() -> GetTagByNameStmt {
    GetTagByNameStmt("SELECT * FROM tag WHERE name = $1", None)
}
impl GetTagByNameStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s self,
        client: &'c C,
        name: &'a T1,
    ) -> GetTagByNameQuery<'c, 'a, 's, C, GetTagByName, 1> {
        GetTagByNameQuery {
            client,
            params: [name],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<GetTagByNameBorrowed, tokio_postgres::Error> {
                    Ok(GetTagByNameBorrowed {
                        id: row.try_get(0)?,
                        name: row.try_get(1)?,
                        created_at: row.try_get(2)?,
                    })
                },
            mapper: |it| GetTagByName::from(it),
        }
    }
}
pub struct AddTagToArticleStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn add_tag_to_article() -> AddTagToArticleStmt {
    AddTagToArticleStmt(
        "INSERT INTO article_tag (article_id, tag_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
        None,
    )
}
impl AddTagToArticleStmt {
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
        article_id: &'a uuid::Uuid,
        tag_id: &'a uuid::Uuid,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[article_id, tag_id]).await
    }
}
impl<'a, C: GenericClient + Send + Sync>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        AddTagToArticleParams,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for AddTagToArticleStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a AddTagToArticleParams,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.article_id, &params.tag_id))
    }
}
pub struct RemoveTagsFromArticleStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn remove_tags_from_article() -> RemoveTagsFromArticleStmt {
    RemoveTagsFromArticleStmt("DELETE FROM article_tag WHERE article_id = $1", None)
}
impl RemoveTagsFromArticleStmt {
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
        article_id: &'a uuid::Uuid,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[article_id]).await
    }
}
pub struct GetArticleTagsStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_article_tags() -> GetArticleTagsStmt {
    GetArticleTagsStmt(
        "SELECT t.name FROM tag t JOIN article_tag at ON t.id = at.tag_id WHERE at.article_id = $1",
        None,
    )
}
impl GetArticleTagsStmt {
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
    ) -> StringQuery<'c, 'a, 's, C, String, 1> {
        StringQuery {
            client,
            params: [article_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it.into(),
        }
    }
}
pub struct ListArticlesStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn list_articles() -> ListArticlesStmt {
    ListArticlesStmt(
        "SELECT a.id, a.slug, a.title, a.description, a.body, a.author_id, a.created_at, a.updated_at, u.username as author_username, u.bio as author_bio, u.img as author_image, EXISTS(SELECT 1 FROM appuser_follows WHERE follower_id = $1 AND followee_id = a.author_id) as following_author, EXISTS(SELECT 1 FROM article_favorite WHERE appuser_id = $1 AND article_id = a.id) as favorited, (SELECT COUNT(*) FROM article_favorite WHERE article_id = a.id) as favorites_count, ARRAY(SELECT t.name FROM tag t JOIN article_tag at ON t.id = at.tag_id WHERE at.article_id = a.id ORDER BY t.name) as tag_list FROM article a JOIN appuser u ON a.author_id = u.id WHERE ($2::text IS NULL OR a.author_id = (SELECT id FROM appuser WHERE username = $2)) AND ($3::text IS NULL OR EXISTS(SELECT 1 FROM article_tag at JOIN tag t ON at.tag_id = t.id WHERE at.article_id = a.id AND t.name = $3)) AND ($4::text IS NULL OR EXISTS(SELECT 1 FROM article_favorite af JOIN appuser u2 ON af.appuser_id = u2.id WHERE af.article_id = a.id AND u2.username = $4)) ORDER BY a.created_at DESC LIMIT $5 OFFSET $6",
        None,
    )
}
impl ListArticlesStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<
        'c,
        'a,
        's,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
    >(
        &'s self,
        client: &'c C,
        viewer_id: &'a uuid::Uuid,
        author: &'a T1,
        tag: &'a T2,
        favorited: &'a T3,
        limit: &'a i64,
        offset: &'a i64,
    ) -> ListArticlesQuery<'c, 'a, 's, C, ListArticles, 6> {
        ListArticlesQuery {
            client,
            params: [viewer_id, author, tag, favorited, limit, offset],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<ListArticlesBorrowed, tokio_postgres::Error> {
                    Ok(ListArticlesBorrowed {
                        id: row.try_get(0)?,
                        slug: row.try_get(1)?,
                        title: row.try_get(2)?,
                        description: row.try_get(3)?,
                        body: row.try_get(4)?,
                        author_id: row.try_get(5)?,
                        created_at: row.try_get(6)?,
                        updated_at: row.try_get(7)?,
                        author_username: row.try_get(8)?,
                        author_bio: row.try_get(9)?,
                        author_image: row.try_get(10)?,
                        following_author: row.try_get(11)?,
                        favorited: row.try_get(12)?,
                        favorites_count: row.try_get(13)?,
                        tag_list: row.try_get(14)?,
                    })
                },
            mapper: |it| ListArticles::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql, T3: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        ListArticlesParams<T1, T2, T3>,
        ListArticlesQuery<'c, 'a, 's, C, ListArticles, 6>,
        C,
    > for ListArticlesStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a ListArticlesParams<T1, T2, T3>,
    ) -> ListArticlesQuery<'c, 'a, 's, C, ListArticles, 6> {
        self.bind(
            client,
            &params.viewer_id,
            &params.author,
            &params.tag,
            &params.favorited,
            &params.limit,
            &params.offset,
        )
    }
}
pub struct CountArticlesStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn count_articles() -> CountArticlesStmt {
    CountArticlesStmt(
        "SELECT COUNT(*) FROM article a WHERE ($1::text IS NULL OR a.author_id = (SELECT id FROM appuser WHERE username = $1)) AND ($2::text IS NULL OR EXISTS(SELECT 1 FROM article_tag at JOIN tag t ON at.tag_id = t.id WHERE at.article_id = a.id AND t.name = $2)) AND ($3::text IS NULL OR EXISTS(SELECT 1 FROM article_favorite af JOIN appuser u2 ON af.appuser_id = u2.id WHERE af.article_id = a.id AND u2.username = $3))",
        None,
    )
}
impl CountArticlesStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<
        'c,
        'a,
        's,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
    >(
        &'s self,
        client: &'c C,
        author: &'a T1,
        tag: &'a T2,
        favorited: &'a T3,
    ) -> I64Query<'c, 'a, 's, C, i64, 3> {
        I64Query {
            client,
            params: [author, tag, favorited],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it,
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql, T3: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        CountArticlesParams<T1, T2, T3>,
        I64Query<'c, 'a, 's, C, i64, 3>,
        C,
    > for CountArticlesStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a CountArticlesParams<T1, T2, T3>,
    ) -> I64Query<'c, 'a, 's, C, i64, 3> {
        self.bind(client, &params.author, &params.tag, &params.favorited)
    }
}
pub struct FeedArticlesStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn feed_articles() -> FeedArticlesStmt {
    FeedArticlesStmt(
        "SELECT a.id, a.slug, a.title, a.description, a.body, a.author_id, a.created_at, a.updated_at, u.username as author_username, u.bio as author_bio, u.img as author_image, true as following_author, EXISTS(SELECT 1 FROM article_favorite WHERE appuser_id = $1 AND article_id = a.id) as favorited, (SELECT COUNT(*) FROM article_favorite WHERE article_id = a.id) as favorites_count, ARRAY(SELECT t.name FROM tag t JOIN article_tag at ON t.id = at.tag_id WHERE at.article_id = a.id ORDER BY t.name) as tag_list FROM article a JOIN appuser u ON a.author_id = u.id JOIN appuser_follows af ON af.followee_id = a.author_id WHERE af.follower_id = $1 ORDER BY a.created_at DESC LIMIT $2 OFFSET $3",
        None,
    )
}
impl FeedArticlesStmt {
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
        viewer_id: &'a uuid::Uuid,
        limit: &'a i64,
        offset: &'a i64,
    ) -> FeedArticlesQuery<'c, 'a, 's, C, FeedArticles, 3> {
        FeedArticlesQuery {
            client,
            params: [viewer_id, limit, offset],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<FeedArticlesBorrowed, tokio_postgres::Error> {
                    Ok(FeedArticlesBorrowed {
                        id: row.try_get(0)?,
                        slug: row.try_get(1)?,
                        title: row.try_get(2)?,
                        description: row.try_get(3)?,
                        body: row.try_get(4)?,
                        author_id: row.try_get(5)?,
                        created_at: row.try_get(6)?,
                        updated_at: row.try_get(7)?,
                        author_username: row.try_get(8)?,
                        author_bio: row.try_get(9)?,
                        author_image: row.try_get(10)?,
                        following_author: row.try_get(11)?,
                        favorited: row.try_get(12)?,
                        favorites_count: row.try_get(13)?,
                        tag_list: row.try_get(14)?,
                    })
                },
            mapper: |it| FeedArticles::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        FeedArticlesParams,
        FeedArticlesQuery<'c, 'a, 's, C, FeedArticles, 3>,
        C,
    > for FeedArticlesStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a FeedArticlesParams,
    ) -> FeedArticlesQuery<'c, 'a, 's, C, FeedArticles, 3> {
        self.bind(client, &params.viewer_id, &params.limit, &params.offset)
    }
}
pub struct CountFeedArticlesStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn count_feed_articles() -> CountFeedArticlesStmt {
    CountFeedArticlesStmt(
        "SELECT COUNT(*) FROM article a JOIN appuser_follows af ON af.followee_id = a.author_id WHERE af.follower_id = $1",
        None,
    )
}
impl CountFeedArticlesStmt {
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
        viewer_id: &'a uuid::Uuid,
    ) -> I64Query<'c, 'a, 's, C, i64, 1> {
        I64Query {
            client,
            params: [viewer_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it,
        }
    }
}
