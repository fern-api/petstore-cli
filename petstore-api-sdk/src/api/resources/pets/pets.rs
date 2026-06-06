use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct PetsClient {
    pub http_client: HttpClient,
}

impl PetsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn list_pets(
        &self,
        request: &ListPetsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<Pet>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "pets",
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn create_pet(
        &self,
        request: &CreatePetRequest,
        options: Option<RequestOptions>,
    ) -> Result<Pet, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "pets",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn get_pet(
        &self,
        pet_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Pet, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("pets/{}", pet_id),
                None,
                None,
                options,
            )
            .await
    }
}
