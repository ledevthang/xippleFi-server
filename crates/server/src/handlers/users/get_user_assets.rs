use crate::{
    error::ServerError::{self},
    extractors::{security::Auth, states::Postgres},
};

#[utoipa::path(
    tag = "Users",
    get,
    security(
        ("BearerAuth" = []),
    ),
    path = "/api/user/assets",
    responses(
        (status = 200, body = ())
    )
)]
pub async fn get_user_assets(
    Auth(_claims): Auth,
    Postgres(_db): Postgres,
) -> Result<(), ServerError> {
    Ok(())
}
