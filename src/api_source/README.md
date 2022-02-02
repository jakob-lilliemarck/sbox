# source api
Http API to manage source resources.

## Localhost
`cargo run --bin api`

### Test with curl

**Create**
`curl -X POST http://localhost:8000/source -H "Content-Type: application/json" -d '{"src": "const fn = () => console.log('running!')", "lang": "javascript"}'`

**Update**
`curl -X PUT http://localhost:8000/source/<id>` -H "Content-Type: application/json" -d '{"src": "const fn = () => console.log('wroom wroom!')", "lang": "javascript"}'

**Delete**
`curl -X DELETE http://localhost:8000/source/<id>` -H "Content-Type: application/json"
