# mayday

primary goal of the app is to save lives when people are outside alone, protection, and vigilance without being too overwhelming or limiting the scope of use.

### 1. **Tracking and Alerts Overview**

1. **Client Location Tracking**: Each client periodically sends its location to the backend.
2. **Alert Triggering**: Clients can trigger alerts based on certain conditions.
3. **Notification of Nearby Clients**: When a client triggers an alert, nearby clients are notified.

## docker/.env
```dotenv
MARIADB_HOST=127.0.0.1
MARIADB_PORT=3306
MARIADB_USER=maydayapp
MARIADB_PASS=password
MARIADB_APP_PASS=password
MARIADB_DB=mayday
DATABASE_URL="mysql://$(MARIADB_USER):$(MARIADB_PASS)@$(MARIADB_HOST):$(MARIADB_PORT)/$(MARIADB_DB)"
```

cargo install sea-orm-cli@1.0.0-rc.5

