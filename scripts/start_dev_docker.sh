#!/bin/bash
export DOCKER_MARIADB_DBHOST="127.0.0.1";
export DOCKER_MARIADB_DBPORT="3306";
export DOCKER_MARIADB_DBNAME="mdb";
export DOCKER_MARIADB_DBHOSTPW="Password123!";
export DOCKER_MARIADB_DATABASE="mydatabase";
export DOCKER_MARIADB_USER="'devuser'";
export DOCKER_MARIADB_TABLE="mytable";
docker run -p "$DOCKER_MARIADB_DBHOST":"$DOCKER_MARIADB_DBPORT":"$DOCKER_MARIADB_DBPORT"  --name "$DOCKER_MARIADB_DBNAME" -e MARIADB_ROOT_PASSWORD="$DOCKER_MARIADB_DBHOSTPW" -d mariadb:latest &
sleep 30s;
mariadb -h "$DOCKER_MARIADB_DBHOST" -u root -p"$DOCKER_MARIADB_DBHOSTPW" -e "CREATE DATABASE $DOCKER_MARIADB_DATABASE;";
mariadb -D "$DOCKER_MARIADB_DATABASE" -h "$DOCKER_MARIADB_DBHOST" -uroot -p"$DOCKER_MARIADB_DBHOSTPW" -e 'CREATE TABLE '"$DOCKER_MARIADB_TABLE"' ('id' int(11) NOT NULL AUTO_INCREMENT,'method' varchar(255) NOT NULL,'host' varchar(255) NOT NULL,'port' varchar(255) NOT NULL,'uri' varchar(255) NOT NULL,'headers' varchar(255) NOT NULL,'body' varchar(6255) NOT NULL,PRIMARY KEY ('id'));';
mariadb -h "$DOCKER_MARIADB_DBHOST" -uroot -p"$DOCKER_MARIADB_DBHOSTPW" -e "CREATE USER $DOCKER_MARIADB_USER@'%' IDENTIFIED BY 'password';";
mariadb -h "$DOCKER_MARIADB_DBHOST" -uroot -p"$DOCKER_MARIADB_DBHOSTPW" -e "GRANT ALL PRIVILEGES ON *.* TO $DOCKER_MARIADB_USER@'%';";
mariadb -h "$DOCKER_MARIADB_DBHOST" -uroot -p"$DOCKER_MARIADB_DBHOSTPW" -e "FLUSH PRIVILEGES;";
mariadb -h "$DOCKER_MARIADB_DBHOST" -uroot -p"$DOCKER_MARIADB_DBHOSTPW" -e 'CREATE DATABASE mayday;';
mariadb -h "$DOCKER_MARIADB_DBHOST" -uroot -p"$DOCKER_MARIADB_DBHOSTPW" -e 'USE mayday;';
mariadb -h "$DOCKER_MARIADB_DBHOST" -uroot -p"$DOCKER_MARIADB_DBHOSTPW" -e 'CREATE TABLE `user` (`userid` INT NOT NULL,`name` VARCHAR(255) NOT NULL,`email` VARCHAR(255) NOT NULL,`password` VARCHAR(255) NOT NULL,PRIMARY KEY (`userid`)) ENGINE=InnoDB;' mayday;
mariadb -h "$DOCKER_MARIADB_DBHOST" -uroot -p"$DOCKER_MARIADB_DBHOSTPW" -e 'CREATE TABLE `tank` (`userid` INT NOT NULL,`tankid` INT NOT NULL,`name` VARCHAR(255) NOT NULL,`size_gallons` INT NOT NULL,`height` INT,`length` INT,`width` INT,`volume` INT,`weight` FLOAT,PRIMARY KEY (`userid`)) ENGINE=InnoDB;' mayday;
mariadb -h "$DOCKER_MARIADB_DBHOST" -uroot -p"$DOCKER_MARIADB_DBHOSTPW" -e 'CREATE TABLE `parameter` (`userid` INT NOT NULL,`ph` FLOAT,`kh` FLOAT,`ammonia` FLOAT,`nitrite` FLOAT,`nitrate` FLOAT,PRIMARY KEY (`userid`)) ENGINE=InnoDB;' mayday;
mariadb -h "$DOCKER_MARIADB_DBHOST" -uroot -p"$DOCKER_MARIADB_DBHOSTPW" -e 'CREATE TABLE `fish` (`userid` INT NOT NULL,`tankid` INT NOT NULL,`fishid` INT NOT NULL,`name` VARCHAR(255) NOT NULL,`species` VARCHAR(255) NOT NULL,`qty` INT,PRIMARY KEY (`fishid`)) ENGINE=InnoDB;' mayday;
mariadb -h "$DOCKER_MARIADB_DBHOST" -uroot -p"$DOCKER_MARIADB_DBHOSTPW" -e 'CREATE TABLE `session` (`userid` INT NOT NULL,`name` VARCHAR(255) NOT NULL,`email` VARCHAR(255) NOT NULL,`sessionid` VARCHAR(255) NOT NULL,PRIMARY KEY (`sessionid`)) ENGINE=InnoDB;' mayday;
mariadb -h "$DOCKER_MARIADB_DBHOST" -uroot -p"$DOCKER_MARIADB_DBHOSTPW" -e 'SHOW TABLES;' mayday;
