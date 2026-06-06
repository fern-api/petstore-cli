# Reference
## Pets
<details><summary><code>client.pets.<a href="/src/api/resources/pets/client.rs">list_pets</a>(limit: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;Vec&lt;Pet&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use petstore_api_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = PetstoreApiClient::new(config).expect("Failed to build client");
    client
        .pets
        .list_pets(
            &ListPetsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**limit:** `Option<i64>` — Maximum number of pets to return.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.pets.<a href="/src/api/resources/pets/client.rs">create_pet</a>(request: CreatePetRequest) -> Result&lt;Pet, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use petstore_api_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = PetstoreApiClient::new(config).expect("Failed to build client");
    client
        .pets
        .create_pet(
            &CreatePetRequest {
                name: "name".to_string(),
                tag: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**name:** `String` — Name of the pet.
    
</dd>
</dl>

<dl>
<dd>

**tag:** `Option<String>` — Optional tag for the pet.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.pets.<a href="/src/api/resources/pets/client.rs">get_pet</a>(pet_id: String) -> Result&lt;Pet, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use petstore_api_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = PetstoreApiClient::new(config).expect("Failed to build client");
    client.pets.get_pet(&"petId".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pet_id:** `String` — The ID of the pet to retrieve.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Auth
<details><summary><code>client.auth.<a href="/src/api/resources/auth/client.rs">get_token</a>(request: GetTokenAuthRequest) -> Result&lt;TokenResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use petstore_api_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = PetstoreApiClient::new(config).expect("Failed to build client");
    client
        .auth
        .get_token(
            &GetTokenAuthRequest {
                client_id: "client_id".to_string(),
                client_secret: "client_secret".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**client_id:** `String` — OAuth2 client ID.
    
</dd>
</dl>

<dl>
<dd>

**client_secret:** `String` — OAuth2 client secret.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

