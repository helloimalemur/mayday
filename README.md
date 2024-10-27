# mayday

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

