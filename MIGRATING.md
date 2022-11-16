# Migrating
This document covers how to migrate from the Python-based MPR implementation to this Rust-based one.

## Prerequisites
- The database from the Python implementation.
- An empty database for the Rust implementation.

## Setting up the new Rust database
First you'll want to set up the Rust database:

```sh
sea-orm-cli migrate up -n 1
```

## Preparing the Python database
To prepare the database from the Python implementation to be migrated into the Rust implementation, run the following SQL statement on the database:

```sql
ALTER TABLE AcceptedTerms ADD COLUMN `ID` INT UNSIGNED PRIMARY KEY AUTO_INCREMENT FIRST;
ALTER TABLE PackageComaintainers ADD COLUMN `ID` INT UNSIGNED PRIMARY KEY AUTO_INCREMENT FIRST;
ALTER TABLE PackageDepends ADD COLUMN `ID` INT UNSIGNED PRIMARY KEY AUTO_INCREMENT FIRST;
ALTER TABLE PackageNotifications ADD COLUMN `ID` INT UNSIGNED PRIMARY KEY AUTO_INCREMENT FIRST;
ALTER TABLE PackageRelations ADD COLUMN `ID` INT UNSIGNED PRIMARY KEY AUTO_INCREMENT FIRST;
ALTER TABLE PackageSources ADD COLUMN `ID` INT UNSIGNED PRIMARY KEY AUTO_INCREMENT FIRST;
ALTER TABLE PackageVotes ADD COLUMN `ID` INT UNSIGNED PRIMARY KEY AUTO_INCREMENT FIRST;
ALTER TABLE Sessions ADD COLUMN `ID` INT UNSIGNED PRIMARY KEY AUTO_INCREMENT FIRST;
ALTER TABLE TU_Votes ADD COLUMN `ID` INT UNSIGNED PRIMARY KEY AUTO_INCREMENT FIRST;
```

Then export your database as shown below (replacing `{python_db_name}` with the name of the Python implementation's database):

```sh
mysqldump --no-create-info --ignore-table='{python_db_name}.alembic_version' '{python_db_name}' > mprweb.db
```

## Importing the data into the Rust database
Now that your data is exported, all that's needed is to import it into the Python implementation (replacing `{rust_db_name}` with the name of the Rust implementation's database):

```sh
mysql '{rust_db_name}' < mprweb.db
```