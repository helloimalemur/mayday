Certainly! Below, I'll provide the Rust struct definitions for the mobile app and the backend service, as well as a corresponding database schema. The goal is to ensure that the data model is well-structured for location tracking, alert management, and integration handling.

### Rust Structs

#### **Client-Side**

1. **LocationData**
   ```rust
   #[derive(Debug, Serialize, Deserialize)]
   pub struct LocationData {
       pub user_id: String, // User identifier
       pub latitude: f64,
       pub longitude: f64,
       pub timestamp: String, // ISO 8601 format
       pub is_moving: bool,
   }
   ```

2. **TrackingStatus**
   ```rust
   #[derive(Debug, Serialize, Deserialize)]
   pub struct TrackingStatus {
       pub user_id: String,
       pub is_enabled: bool,
       pub last_update: String, // ISO 8601 format
   }
   ```

3. **AlertConfig**
   ```rust
   #[derive(Debug, Serialize, Deserialize)]
   pub struct AlertConfig {
       pub user_id: String,
       pub integration_contacts: Vec<String>, // List of contact identifiers
       pub nearby_user_threshold: f64, // Distance threshold to trigger alerts
       pub movement_threshold: f64, // Distance threshold to determine if user has stopped moving
   }
   ```

#### **Backend Service**

1. **User**
   ```rust
   #[derive(Debug, Serialize, Deserialize)]
   pub struct User {
       pub id: String, // Unique user identifier
       pub name: String,
       pub email: String,
       pub phone_number: Option<String>,
   }
   ```

2. **Location**
   ```rust
   #[derive(Debug, Serialize, Deserialize)]
   pub struct Location {
       pub id: i32, // Auto-incrementing primary key
       pub user_id: String,
       pub latitude: f64,
       pub longitude: f64,
       pub timestamp: String, // ISO 8601 format
       pub is_moving: bool,
   }
   ```

3. **Alert**
   ```rust
   #[derive(Debug, Serialize, Deserialize)]
   pub struct Alert {
       pub id: i32, // Auto-incrementing primary key
       pub user_id: String,
       pub alert_type: String, // e.g., "movement_stop", "no_reporting"
       pub timestamp: String, // ISO 8601 format
       pub details: Option<String>,
   }
   ```

4. **Integration**
   ```rust
   #[derive(Debug, Serialize, Deserialize)]
   pub struct Integration {
       pub id: i32, // Auto-incrementing primary key
       pub name: String,
       pub type_: String, // e.g., "SMS", "Email", "Emergency_Service"
       pub contact_info: String,
   }
   ```

### Database Schema

#### **User Table**

```sql
CREATE TABLE users (
    id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    phone_number VARCHAR(20)
);
```

#### **Location Table**

```sql
CREATE TABLE locations (
    id SERIAL PRIMARY KEY,
    user_id VARCHAR(255) REFERENCES users(id),
    latitude DOUBLE PRECISION NOT NULL,
    longitude DOUBLE PRECISION NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL,
    is_moving BOOLEAN NOT NULL
);
```

#### **Alert Table**

```sql
CREATE TABLE alerts (
    id SERIAL PRIMARY KEY,
    user_id VARCHAR(255) REFERENCES users(id),
    alert_type VARCHAR(50) NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL,
    details TEXT
);
```

#### **Integration Table**

```sql
CREATE TABLE integrations (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    type VARCHAR(50) NOT NULL,
    contact_info VARCHAR(255) NOT NULL
);
```

#### **AlertConfig Table** (Optional, if you want to store configuration in the database)

```sql
CREATE TABLE alert_configs (
    user_id VARCHAR(255) PRIMARY KEY REFERENCES users(id),
    integration_contacts TEXT[], -- Array of contact identifiers
    nearby_user_threshold DOUBLE PRECISION NOT NULL,
    movement_threshold DOUBLE PRECISION NOT NULL
);
```

### Summary

- **Rust Structs**: Define the data structures for handling location updates, tracking statuses, alert configurations, and integrations.
- **Database Schema**: Create tables to store user information, location data, alerts, and integration configurations. Optionally, an `alert_configs` table can be used to manage per-user alert settings.

This structure should give you a solid foundation to implement your mobile app and backend service with Rust and a relational database.