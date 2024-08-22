curl -H "Accept: application/json" -X GET http://localhost:9006/v3/connect/vigoo
curl -H "Accept: application/json" -X GET http://localhost:9006/v3/connect/john
curl -H "Accept: application/json" -X GET http://localhost:9006/v3/connect/nico
curl -H "Accept: application/json" -X GET http://localhost:9006/v3/connect/david
curl -H "Accept: application/json" -X GET http://localhost:9006/v3/connect/peter
curl -H "Accept: application/json" -X GET http://localhost:9006/v3/connect/afsal
curl -H "Accept: application/json" -X POST http://localhost:9006/v3/add -d @add-request-body.json
curl -H "Accept: application/json" -X POST http://localhost:9006/v3/insert -d @insert-request-body.json
curl -H "Accept: application/json" -X GET http://localhost:9006/v3/connected-editors
curl -H "Accept: application/json" -X GET http://localhost:9006/v3/is-archived
curl -H "Accept: application/json" -X POST http://localhost:9006/v3/archive


# Cloud
curl -H "Accept: application/json" -X GET https://listeditor.api.golem.cloud/v3/connect/vigoo
curl -H "Accept: application/json" -X GET https://listeditor.api.golem.cloud/v3/connect/john
curl -H "Accept: application/json" -X GET https://listeditor.api.golem.cloud/v3/connect/nico
curl -H "Accept: application/json" -X GET https://listeditor.api.golem.cloud/v3/connect/david
curl -H "Accept: application/json" -X GET https://listeditor.api.golem.cloud/v3/connect/peter
curl -H "Accept: application/json" -X GET https://listeditor.api.golem.cloud/v3/connect/afsal
curl -H "Accept: application/json" -X POST https://listeditor.api.golem.cloud/v3/add -d @add-request-body.json
curl -H "Accept: application/json" -X POST https://listeditor.api.golem.cloud/v3/insert -d @insert-request-body.json
curl -H "Accept: application/json" -X GET https://listeditor.api.golem.cloud/v3/connected-editors
curl -H "Accept: application/json" -X GET https://listeditor.api.golem.cloud/v3/is-archived
curl -H "Accept: application/json" -X POST https://listeditor.api.golem.cloud/v3/archive