# abstract

This is an easy actix-web project.Just lottery hero for HOK. And same logic project [clean architecture](https://github.com/weiraneve/actix-clean-architecture)

I have kotlin SpringBoot project as some logic [hok-lottery](https://github.com/weiraneve/hok-lottery)

## apis

See [the API documentation pages](./apis) for more info.

## sql

See [lottery sql info](./sql) for more sql info

## usages

1. Create database and tables, you can see it by [lottery sql](./sql).
   this can be done in the terminal, then input your mysql root password four times.:
     ```shell
     ./sql/import_databases.sh
     ```
   
   If you don't want to test sql and use the shell script, you can create mysql database and use:
   ```shell
   mysql -u root -p lottery < ./sql/lottery.sql
   ```
   
2. Create a `.env` file in this directory:
    ```ini
    SERVER_ADDR=127.0.0.1:8034
    DATABASE_URL=mysql://<username>:<password>@localhost:3306/lottery
    DATABASE_URL_TEST=mysql://<username>:<password>@localhost:3306/lottery_test
    ```

3. Run the server

## architecture overview

my actix `The Layered Architecture`:

```
src
├── app_state.rs
├── controller
│  ├── mod.rs
│  ├── pick.rs
│  └── reset.rs
├── creat_app.rs
├── lib.rs
├── main.rs
├── model
│  ├── hero.rs
│  ├── log.rs
│  ├── mod.rs
│  ├── my_result.rs
│  ├── post_param.rs
│  ├── team.rs
│  └── team_query.rs
├── repository
│  ├── hero.rs
│  ├── log.rs
│  ├── mod.rs
│  └── team.rs
├── service
│  ├── mod.rs
│  ├── pick.rs
│  └── reset.rs
└── test
    ├── mod.rs
    ├── test_pick_controller.rs
    └── test_team_repository.rs
```

## testing support

To run the tests, you can go `src/test` and use the following command:
```bash
cargo test
```
