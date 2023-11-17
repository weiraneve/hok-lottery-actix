# Clear API

All examples show cURL snippets.

## Clearing a team by id

```shel
curl -X GET 'http://localhost:8034/clear?id=9' -H "Content-Type: application/json; charset=utf-8"
```

You should clear a team choice hero by id.

## Clearing all teams

```shell
curl -X GET 'http://localhost:8034/clear/team' -H "Content-Type: application/json; charset=utf-8"
```

You should clear all teams.

## Clearing all heroes

```shell
curl -X GET 'http://localhost:8034/clear/hero' -H "Content-Type: application/json; charset=utf-8"
```

You should clear all heroes.
