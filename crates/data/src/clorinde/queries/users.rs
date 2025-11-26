// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct CreateUserParams<T1: crate::clorinde::StringSql, T2: crate::clorinde::StringSql, T3: crate::clorinde::StringSql> {
    pub id: uuid::Uuid,
    pub email: T1,
    pub username: T2,
    pub pwd: T3,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}
#[derive(Debug)]
pub struct UpdateUserParams<
    T1: crate::clorinde::StringSql,
    T2: crate::clorinde::StringSql,
    T3: crate::clorinde::StringSql,
    T4: crate::clorinde::StringSql,
    T5: crate::clorinde::StringSql,
> {
    pub email: T1,
    pub username: T2,
    pub pwd: T3,
    pub img: T4,
    pub bio: T5,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
    pub id: uuid::Uuid,
}
#[derive(Clone, Copy, Debug)]
pub struct FollowUserParams {
    pub follower_id: uuid::Uuid,
    pub followee_id: uuid::Uuid,
}
#[derive(Clone, Copy, Debug)]
pub struct UnfollowUserParams {
    pub follower_id: uuid::Uuid,
    pub followee_id: uuid::Uuid,
}
#[derive(Clone, Copy, Debug)]
pub struct IsFollowingParams {
    pub follower_id: uuid::Uuid,
    pub followee_id: uuid::Uuid,
}
#[derive(Debug, Clone, PartialEq)]
pub struct CreateUser {
    pub id: uuid::Uuid,
    pub email: String,
    pub username: String,
    pub pwd: String,
    pub img: String,
    pub bio: String,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
pub struct CreateUserBorrowed<'a> {
    pub id: uuid::Uuid,
    pub email: &'a str,
    pub username: &'a str,
    pub pwd: &'a str,
    pub img: &'a str,
    pub bio: &'a str,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
impl<'a> From<CreateUserBorrowed<'a>> for CreateUser {
    fn from(
        CreateUserBorrowed {
            id,
            email,
            username,
            pwd,
            img,
            bio,
            created_at,
            updated_at,
        }: CreateUserBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            email: email.into(),
            username: username.into(),
            pwd: pwd.into(),
            img: img.into(),
            bio: bio.into(),
            created_at,
            updated_at,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetUserByEmail {
    pub id: uuid::Uuid,
    pub email: String,
    pub username: String,
    pub pwd: String,
    pub img: String,
    pub bio: String,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
pub struct GetUserByEmailBorrowed<'a> {
    pub id: uuid::Uuid,
    pub email: &'a str,
    pub username: &'a str,
    pub pwd: &'a str,
    pub img: &'a str,
    pub bio: &'a str,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
impl<'a> From<GetUserByEmailBorrowed<'a>> for GetUserByEmail {
    fn from(
        GetUserByEmailBorrowed {
            id,
            email,
            username,
            pwd,
            img,
            bio,
            created_at,
            updated_at,
        }: GetUserByEmailBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            email: email.into(),
            username: username.into(),
            pwd: pwd.into(),
            img: img.into(),
            bio: bio.into(),
            created_at,
            updated_at,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetUserByUsername {
    pub id: uuid::Uuid,
    pub email: String,
    pub username: String,
    pub pwd: String,
    pub img: String,
    pub bio: String,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
pub struct GetUserByUsernameBorrowed<'a> {
    pub id: uuid::Uuid,
    pub email: &'a str,
    pub username: &'a str,
    pub pwd: &'a str,
    pub img: &'a str,
    pub bio: &'a str,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
impl<'a> From<GetUserByUsernameBorrowed<'a>> for GetUserByUsername {
    fn from(
        GetUserByUsernameBorrowed {
            id,
            email,
            username,
            pwd,
            img,
            bio,
            created_at,
            updated_at,
        }: GetUserByUsernameBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            email: email.into(),
            username: username.into(),
            pwd: pwd.into(),
            img: img.into(),
            bio: bio.into(),
            created_at,
            updated_at,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetUserById {
    pub id: uuid::Uuid,
    pub email: String,
    pub username: String,
    pub pwd: String,
    pub img: String,
    pub bio: String,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
pub struct GetUserByIdBorrowed<'a> {
    pub id: uuid::Uuid,
    pub email: &'a str,
    pub username: &'a str,
    pub pwd: &'a str,
    pub img: &'a str,
    pub bio: &'a str,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
impl<'a> From<GetUserByIdBorrowed<'a>> for GetUserById {
    fn from(
        GetUserByIdBorrowed {
            id,
            email,
            username,
            pwd,
            img,
            bio,
            created_at,
            updated_at,
        }: GetUserByIdBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            email: email.into(),
            username: username.into(),
            pwd: pwd.into(),
            img: img.into(),
            bio: bio.into(),
            created_at,
            updated_at,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct UpdateUser {
    pub id: uuid::Uuid,
    pub email: String,
    pub username: String,
    pub pwd: String,
    pub img: String,
    pub bio: String,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
pub struct UpdateUserBorrowed<'a> {
    pub id: uuid::Uuid,
    pub email: &'a str,
    pub username: &'a str,
    pub pwd: &'a str,
    pub img: &'a str,
    pub bio: &'a str,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
impl<'a> From<UpdateUserBorrowed<'a>> for UpdateUser {
    fn from(
        UpdateUserBorrowed {
            id,
            email,
            username,
            pwd,
            img,
            bio,
            created_at,
            updated_at,
        }: UpdateUserBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            email: email.into(),
            username: username.into(),
            pwd: pwd.into(),
            img: img.into(),
            bio: bio.into(),
            created_at,
            updated_at,
        }
    }
}
use crate::clorinde::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct CreateUserQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<CreateUserBorrowed, tokio_postgres::Error>,
    mapper: fn(CreateUserBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> CreateUserQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(CreateUserBorrowed) -> R,
    ) -> CreateUserQuery<'c, 'a, 's, C, R, N> {
        CreateUserQuery {
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
pub struct GetUserByEmailQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<GetUserByEmailBorrowed, tokio_postgres::Error>,
    mapper: fn(GetUserByEmailBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> GetUserByEmailQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(GetUserByEmailBorrowed) -> R,
    ) -> GetUserByEmailQuery<'c, 'a, 's, C, R, N> {
        GetUserByEmailQuery {
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
pub struct GetUserByUsernameQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<GetUserByUsernameBorrowed, tokio_postgres::Error>,
    mapper: fn(GetUserByUsernameBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> GetUserByUsernameQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(GetUserByUsernameBorrowed) -> R,
    ) -> GetUserByUsernameQuery<'c, 'a, 's, C, R, N> {
        GetUserByUsernameQuery {
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
pub struct GetUserByIdQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<GetUserByIdBorrowed, tokio_postgres::Error>,
    mapper: fn(GetUserByIdBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> GetUserByIdQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(GetUserByIdBorrowed) -> R,
    ) -> GetUserByIdQuery<'c, 'a, 's, C, R, N> {
        GetUserByIdQuery {
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
pub struct UpdateUserQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<UpdateUserBorrowed, tokio_postgres::Error>,
    mapper: fn(UpdateUserBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> UpdateUserQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(UpdateUserBorrowed) -> R,
    ) -> UpdateUserQuery<'c, 'a, 's, C, R, N> {
        UpdateUserQuery {
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
pub struct CreateUserStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn create_user() -> CreateUserStmt {
    CreateUserStmt(
        "INSERT INTO appuser (id, email, username, pwd, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $5) RETURNING *",
        None,
    )
}
impl CreateUserStmt {
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
        T1: crate::clorinde::StringSql,
        T2: crate::clorinde::StringSql,
        T3: crate::clorinde::StringSql,
    >(
        &'s self,
        client: &'c C,
        id: &'a uuid::Uuid,
        email: &'a T1,
        username: &'a T2,
        pwd: &'a T3,
        created_at: &'a chrono::DateTime<chrono::FixedOffset>,
    ) -> CreateUserQuery<'c, 'a, 's, C, CreateUser, 5> {
        CreateUserQuery {
            client,
            params: [id, email, username, pwd, created_at],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<CreateUserBorrowed, tokio_postgres::Error> {
                    Ok(CreateUserBorrowed {
                        id: row.try_get(0)?,
                        email: row.try_get(1)?,
                        username: row.try_get(2)?,
                        pwd: row.try_get(3)?,
                        img: row.try_get(4)?,
                        bio: row.try_get(5)?,
                        created_at: row.try_get(6)?,
                        updated_at: row.try_get(7)?,
                    })
                },
            mapper: |it| CreateUser::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::clorinde::StringSql, T2: crate::clorinde::StringSql, T3: crate::clorinde::StringSql>
    crate::clorinde::client::async_::Params<
        'c,
        'a,
        's,
        CreateUserParams<T1, T2, T3>,
        CreateUserQuery<'c, 'a, 's, C, CreateUser, 5>,
        C,
    > for CreateUserStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a CreateUserParams<T1, T2, T3>,
    ) -> CreateUserQuery<'c, 'a, 's, C, CreateUser, 5> {
        self.bind(
            client,
            &params.id,
            &params.email,
            &params.username,
            &params.pwd,
            &params.created_at,
        )
    }
}
pub struct GetUserByEmailStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_user_by_email() -> GetUserByEmailStmt {
    GetUserByEmailStmt("SELECT * FROM appuser WHERE email = $1", None)
}
impl GetUserByEmailStmt {
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
        email: &'a T1,
    ) -> GetUserByEmailQuery<'c, 'a, 's, C, GetUserByEmail, 1> {
        GetUserByEmailQuery {
            client,
            params: [email],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<GetUserByEmailBorrowed, tokio_postgres::Error> {
                Ok(GetUserByEmailBorrowed {
                    id: row.try_get(0)?,
                    email: row.try_get(1)?,
                    username: row.try_get(2)?,
                    pwd: row.try_get(3)?,
                    img: row.try_get(4)?,
                    bio: row.try_get(5)?,
                    created_at: row.try_get(6)?,
                    updated_at: row.try_get(7)?,
                })
            },
            mapper: |it| GetUserByEmail::from(it),
        }
    }
}
pub struct GetUserByUsernameStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_user_by_username() -> GetUserByUsernameStmt {
    GetUserByUsernameStmt("SELECT * FROM appuser WHERE username = $1", None)
}
impl GetUserByUsernameStmt {
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
        username: &'a T1,
    ) -> GetUserByUsernameQuery<'c, 'a, 's, C, GetUserByUsername, 1> {
        GetUserByUsernameQuery {
            client,
            params: [username],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<GetUserByUsernameBorrowed, tokio_postgres::Error> {
                Ok(GetUserByUsernameBorrowed {
                    id: row.try_get(0)?,
                    email: row.try_get(1)?,
                    username: row.try_get(2)?,
                    pwd: row.try_get(3)?,
                    img: row.try_get(4)?,
                    bio: row.try_get(5)?,
                    created_at: row.try_get(6)?,
                    updated_at: row.try_get(7)?,
                })
            },
            mapper: |it| GetUserByUsername::from(it),
        }
    }
}
pub struct GetUserByIdStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_user_by_id() -> GetUserByIdStmt {
    GetUserByIdStmt("SELECT * FROM appuser WHERE id = $1", None)
}
impl GetUserByIdStmt {
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
    ) -> GetUserByIdQuery<'c, 'a, 's, C, GetUserById, 1> {
        GetUserByIdQuery {
            client,
            params: [id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<GetUserByIdBorrowed, tokio_postgres::Error> {
                    Ok(GetUserByIdBorrowed {
                        id: row.try_get(0)?,
                        email: row.try_get(1)?,
                        username: row.try_get(2)?,
                        pwd: row.try_get(3)?,
                        img: row.try_get(4)?,
                        bio: row.try_get(5)?,
                        created_at: row.try_get(6)?,
                        updated_at: row.try_get(7)?,
                    })
                },
            mapper: |it| GetUserById::from(it),
        }
    }
}
pub struct UpdateUserStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn update_user() -> UpdateUserStmt {
    UpdateUserStmt(
        "UPDATE appuser SET email = COALESCE($1, email), username = COALESCE($2, username), pwd = COALESCE($3, pwd), img = COALESCE($4, img), bio = COALESCE($5, bio), updated_at = $6 WHERE id = $7 RETURNING *",
        None,
    )
}
impl UpdateUserStmt {
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
        T1: crate::clorinde::StringSql,
        T2: crate::clorinde::StringSql,
        T3: crate::clorinde::StringSql,
        T4: crate::clorinde::StringSql,
        T5: crate::clorinde::StringSql,
    >(
        &'s self,
        client: &'c C,
        email: &'a T1,
        username: &'a T2,
        pwd: &'a T3,
        img: &'a T4,
        bio: &'a T5,
        updated_at: &'a chrono::DateTime<chrono::FixedOffset>,
        id: &'a uuid::Uuid,
    ) -> UpdateUserQuery<'c, 'a, 's, C, UpdateUser, 7> {
        UpdateUserQuery {
            client,
            params: [email, username, pwd, img, bio, updated_at, id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<UpdateUserBorrowed, tokio_postgres::Error> {
                    Ok(UpdateUserBorrowed {
                        id: row.try_get(0)?,
                        email: row.try_get(1)?,
                        username: row.try_get(2)?,
                        pwd: row.try_get(3)?,
                        img: row.try_get(4)?,
                        bio: row.try_get(5)?,
                        created_at: row.try_get(6)?,
                        updated_at: row.try_get(7)?,
                    })
                },
            mapper: |it| UpdateUser::from(it),
        }
    }
}
impl<
    'c,
    'a,
    's,
    C: GenericClient,
    T1: crate::clorinde::StringSql,
    T2: crate::clorinde::StringSql,
    T3: crate::clorinde::StringSql,
    T4: crate::clorinde::StringSql,
    T5: crate::clorinde::StringSql,
>
    crate::clorinde::client::async_::Params<
        'c,
        'a,
        's,
        UpdateUserParams<T1, T2, T3, T4, T5>,
        UpdateUserQuery<'c, 'a, 's, C, UpdateUser, 7>,
        C,
    > for UpdateUserStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a UpdateUserParams<T1, T2, T3, T4, T5>,
    ) -> UpdateUserQuery<'c, 'a, 's, C, UpdateUser, 7> {
        self.bind(
            client,
            &params.email,
            &params.username,
            &params.pwd,
            &params.img,
            &params.bio,
            &params.updated_at,
            &params.id,
        )
    }
}
pub struct FollowUserStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn follow_user() -> FollowUserStmt {
    FollowUserStmt(
        "INSERT INTO appuser_follows (follower_id, followee_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
        None,
    )
}
impl FollowUserStmt {
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
        follower_id: &'a uuid::Uuid,
        followee_id: &'a uuid::Uuid,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[follower_id, followee_id]).await
    }
}
impl<'a, C: GenericClient + Send + Sync>
    crate::clorinde::client::async_::Params<
        'a,
        'a,
        'a,
        FollowUserParams,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for FollowUserStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a FollowUserParams,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.follower_id, &params.followee_id))
    }
}
pub struct UnfollowUserStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn unfollow_user() -> UnfollowUserStmt {
    UnfollowUserStmt(
        "DELETE FROM appuser_follows WHERE follower_id = $1 AND followee_id = $2",
        None,
    )
}
impl UnfollowUserStmt {
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
        follower_id: &'a uuid::Uuid,
        followee_id: &'a uuid::Uuid,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[follower_id, followee_id]).await
    }
}
impl<'a, C: GenericClient + Send + Sync>
    crate::clorinde::client::async_::Params<
        'a,
        'a,
        'a,
        UnfollowUserParams,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for UnfollowUserStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a UnfollowUserParams,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.follower_id, &params.followee_id))
    }
}
pub struct IsFollowingStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn is_following() -> IsFollowingStmt {
    IsFollowingStmt(
        "SELECT EXISTS( SELECT 1 FROM appuser_follows WHERE follower_id = $1 AND followee_id = $2 )",
        None,
    )
}
impl IsFollowingStmt {
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
        follower_id: &'a uuid::Uuid,
        followee_id: &'a uuid::Uuid,
    ) -> BoolQuery<'c, 'a, 's, C, bool, 2> {
        BoolQuery {
            client,
            params: [follower_id, followee_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it,
        }
    }
}
impl<'c, 'a, 's, C: GenericClient>
    crate::clorinde::client::async_::Params<
        'c,
        'a,
        's,
        IsFollowingParams,
        BoolQuery<'c, 'a, 's, C, bool, 2>,
        C,
    > for IsFollowingStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a IsFollowingParams,
    ) -> BoolQuery<'c, 'a, 's, C, bool, 2> {
        self.bind(client, &params.follower_id, &params.followee_id)
    }
}
