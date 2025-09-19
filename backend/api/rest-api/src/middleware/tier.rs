// Universal access middleware - no tier restrictions
// All features are available to every authenticated user

use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;

/// Universal access middleware that provides authentication without feature restrictions
/// All users have access to all features regardless of account status
pub async fn universal_access_middleware(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    // Validate JWT token for security only
    // No tier checking or feature gating
    
    // Set user context for authenticated request
    req.extensions_mut().insert(UserContext {
        user_id: extract_user_id_from_token(credentials.token())?,
        // No tier field - universal access
        authenticated: true,
    });
    
    Ok(req)
}

#[derive(Clone)]
pub struct UserContext {
    pub user_id: String,
    pub authenticated: bool,
    // No tier or subscription fields
}

fn extract_user_id_from_token(token: &str) -> Result<String, Error> {
    // JWT validation logic here
    // Return user ID from validated token
    todo!("Implement JWT validation")
}
