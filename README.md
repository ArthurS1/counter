# counter
A poc http server in rust connected to a MongoDB backend.

## test with curl

- ```curl -i -X POST -H 'Content-Type: application/json' -d '{"modifier": 3}' http://localhost:8080/mod```
- ```curl -i -X GET http://localhost:8080```
