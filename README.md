# mayday
### ~ **work in progress** ~
primary goal of the app is to save lives when people are outside alone, protection, and vigilance without being too overwhelming or limiting the scope of use.

### 1. **Tracking and Alerts Overview**

1. **Client Location Tracking**: Each client periodically sends its location to the backend, using "what3words" the location can be easily communicated verbally.
2. **Alert Triggering**: Clients can trigger alerts based on certain conditions.
3. **Notification of Nearby Clients**: When a client triggers an alert, notify nearby clients and integrations such custom contacts or EMS services.

## prerequisites
    docker
    make
    rust
    npm/node

## Development workflow
    # start db in docker and run rust code 
    make dev
    # build and start both db and mayday in docker containers
    make init
    # build images only
    make build
    # build and up the new images
    make up 

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
## install sea-orm cli
    cargo install sea-orm-cli@1.0.0-rc.5

