To notify a client of other nearby clients who are alerting, you need a system that continuously tracks the location of clients and detects when an alert is triggered. The logic for such a system involves several steps:

### 1. **Tracking and Alerts Overview**

1. **Client Location Tracking**: Each client periodically sends its location to the backend.
2. **Alert Triggering**: Clients can trigger alerts based on certain conditions.
3. **Notification of Nearby Clients**: When a client triggers an alert, nearby clients are notified.

### 2. **Client-Side Logic**

**A. Location Tracking and Reporting**

1. **Start Location Tracking**:
   - When the user enables tracking, start sending periodic location updates to the backend.

2. **Send Location Data**:
   - Periodically send location data (latitude, longitude) to the backend server.
   - Include a timestamp and movement status.

3. **Check for Alerts**:
   - Continuously monitor for any alerts triggered on the client-side (e.g., stop moving, reporting issues).

**B. Triggering Alerts**

1. **Trigger Alert**:
   - If an alert condition is met (e.g., no movement detected), trigger an alert.
   - Send an alert message to the backend indicating the alert type and client ID.

**C. Handling Notifications from the Backend**

1. **Receive Notifications**:
   - Implement a mechanism to receive push notifications or background updates from the backend about nearby alerts.

### 3. **Backend Logic**

**A. Storing and Processing Locations**

1. **Store Location Data**:
   - Store incoming location data in the database.
   - Update the client’s latest location.

2. **Detect Alerts**:
   - Process incoming alert messages from clients.
   - Update the alert status in the database.

**B. Determining Nearby Clients**

1. **Calculate Distance**:
   - For each alert, calculate the distance between the alerting client and all other clients using their latitude and longitude.

2. **Find Nearby Clients**:
   - Determine which clients are within a predefined threshold distance of the alerting client.

**C. Notify Nearby Clients**

1. **Generate Notifications**:
   - For each nearby client, generate a notification about the alert.
   - Include details such as the alerting client’s ID, location, and alert type.

2. **Send Notifications**:
   - Use a notification service (e.g., push notifications, SMS) to send the alert notifications to nearby clients.

### 4. **Sample Logic Flow**

Here’s a sample flow of logic for notifying nearby clients of alerts:

**Client-Side Implementation**

```rust
// 1. Periodically send location data
fn send_location_update(user_id: &str, latitude: f64, longitude: f64, is_moving: bool) {
    // Code to send location data to backend
}

// 2. Trigger an alert if conditions are met
fn trigger_alert(user_id: &str, alert_type: &str) {
    // Code to send alert to backend
}

// 3. Handle incoming notifications
fn handle_notification(notification: &str) {
    // Code to display notification to the user
}
```

**Backend Implementation**

```rust
// 1. Store incoming location data
fn store_location(user_id: &str, latitude: f64, longitude: f64, is_moving: bool) {
    // Code to store location data in the database
}

// 2. Process incoming alert
fn process_alert(user_id: &str, alert_type: &str) {
    // Store alert in the database
    // Find nearby clients
    let nearby_clients = find_nearby_clients(user_id);
    // Notify nearby clients
    notify_clients(nearby_clients, user_id, alert_type);
}

// 3. Calculate distance between two geographical points
fn calculate_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    // Use Haversine formula or similar
}

// 4. Find clients within a certain distance
fn find_nearby_clients(alerting_client_id: &str) -> Vec<String> {
    // Get the location of the alerting client
    let alerting_client_location = get_location(alerting_client_id);
    // Get all clients' locations
    let all_clients_locations = get_all_clients_locations();
    let nearby_clients = all_clients_locations
        .into_iter()
        .filter(|client_location| {
            let distance = calculate_distance(
                alerting_client_location.latitude,
                alerting_client_location.longitude,
                client_location.latitude,
                client_location.longitude,
            );
            distance < ALERT_RADIUS // predefined radius for alert
        })
        .map(|client_location| client_location.user_id)
        .collect();
    nearby_clients
}

// 5. Notify clients about the alert
fn notify_clients(client_ids: Vec<String>, alerting_client_id: &str, alert_type: &str) {
    for client_id in client_ids {
        // Code to send notification to each client
    }
}
```

### 5. **Detailed Flow**

1. **Client Triggers Alert**:
   - Client detects an alert condition and sends an alert to the backend.

2. **Backend Processes Alert**:
   - Backend stores the alert and determines the location of the alerting client.
   - Backend calculates the distance between the alerting client and other clients.

3. **Identify Nearby Clients**:
   - Backend identifies clients within the predefined alert radius.

4. **Notify Nearby Clients**:
   - Backend generates and sends notifications to all identified nearby clients.

5. **Client Receives Notification**:
   - Nearby clients receive a notification about the alert and can act accordingly.

### Conclusion

This logic ensures that clients who are near an alerting client are informed about the alert, allowing for timely responses. Make sure to optimize distance calculations and notification delivery to handle scalability and performance efficiently.