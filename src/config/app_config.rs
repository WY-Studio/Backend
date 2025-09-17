use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub oauth: OAuthConfig,
    pub jwt: JwtConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OAuthConfig {
    pub google: ProviderConfig,
    pub naver: ProviderConfig,
    pub kakao: ProviderConfig,
    pub apple: AppleProviderConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ProviderConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppleProviderConfig {
    pub client_id: String,
    pub redirect_uri: String,
    pub team_id: String,
    pub key_id: String,
    pub private_key: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub issuer: String,
    pub access_ttl_minutes: i64,
    pub refresh_ttl_days: i64,
}

pub fn load_config() -> Result<AppConfig, String> {
    // config.yaml를 우선 읽고, 환경변수로 override
    let builder = config::Config::builder()
        .add_source(config::File::with_name("config").required(true))
        .add_source(config::Environment::with_prefix("APP").separator("__"));

    let cfg = builder.build().map_err(|e| e.to_string())?;
    let app: AppConfig = cfg.try_deserialize().map_err(|e| e.to_string())?;
    Ok(app)
}
