use sqlx::{PgPool, Row};
use std::env;
use bcrypt::{hash, verify, DEFAULT_COST};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file
    dotenv::from_filename(".env").ok();
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;
    
    // Check if admin user exists
    let row = sqlx::query("SELECT id, username, password_hash FROM admins WHERE username = $1")
        .bind("admin")
        .fetch_optional(&pool)
        .await?;
    
    if let Some(row) = row {
        let id: i32 = row.get(0);
        let username: &str = row.get(1);
        let password_hash: &str = row.get(2);
        
        println!("Admin user found:");
        println!("  ID: {}", id);
        println!("  Username: {}", username);
        println!("  Password Hash: {}", password_hash);
        
        // Check if it's a valid bcrypt hash
        if password_hash.starts_with("$2b$") || password_hash.starts_with("$2a$") || password_hash.starts_with("$2y$") {
            // Test password verification
            let default_password = std::env::var("DEFAULT_ADMIN_PASSWORD")
                .unwrap_or_else(|_| "admin123".to_string());
            
            match verify(&default_password, password_hash) {
                Ok(true) => println!("Password verification: SUCCESS"),
                Ok(false) => {
                    println!("Password verification: FAILED");
                    // Update with proper hash
                    update_admin_password(&pool, id, &default_password).await?;
                },
                Err(e) => {
                    println!("Password verification error: {}", e);
                    // Update with proper hash
                    update_admin_password(&pool, id, &default_password).await?;
                },
            }
        } else {
            println!("Invalid password hash format, updating with proper bcrypt hash");
            let default_password = std::env::var("DEFAULT_ADMIN_PASSWORD")
                .unwrap_or_else(|_| "admin123".to_string());
            update_admin_password(&pool, id, &default_password).await?;
        }
    } else {
        println!("Admin user not found");
        
        // Create admin user with hashed password
        let default_password = std::env::var("DEFAULT_ADMIN_PASSWORD")
            .unwrap_or_else(|_| "admin123".to_string());
        
        let hashed_password = hash(&default_password, DEFAULT_COST)?;
        
        sqlx::query(
            "INSERT INTO admins (username, password_hash) VALUES ($1, $2)"
        )
        .bind("admin")
        .bind(&hashed_password)
        .execute(&pool)
        .await?;
        
        println!("Admin user created with hashed password");
    }
    
    Ok(())
}

async fn update_admin_password(pool: &PgPool, admin_id: i32, password: &str) -> Result<(), Box<dyn std::error::Error>> {
    use bcrypt::{hash, DEFAULT_COST};
    
    let hashed_password = hash(password, DEFAULT_COST)?;
    
    sqlx::query(
        "UPDATE admins SET password_hash = $1 WHERE id = $2"
    )
    .bind(&hashed_password)
    .bind(admin_id)
    .execute(pool)
    .await?;
    
    println!("Admin password updated with proper bcrypt hash");
    Ok(())
}