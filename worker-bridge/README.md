## Demo with OSS

### Setup

```bash
 git clone https://github.com/golemcloud/golem-launch-demo.git
 cd precompiled
 golem-cli component add --component-name lst-composed lst-composed.wasm
 golem-cli component add --component-name email-composed email-composed.wasm
 golem-cli component add --component-name archive archive.wasm
 
```

Note down the component ids, and run

```bash

golem-cli worker start --component urn:component:90986bb5-c4c6-4980-ae9a-3086f36b360a --worker-name test4 --env 'ARCHIVE_COMPONENT_ID=5f634fe6-026b-4c0b-937b-17fde4da9aa3' --env 'EMAIL_COMPONENT_ID=4b1dc597-01aa-4a53-b902-e9a1586a5c8d'

```

### Deploy APIs

```bash
cd worker-bridge/definitions-and-scripts
./set_up.sh
./try_out.sh
```

To Reset

```bash

./reset.sh

```

## Cloud Demo of Worker Bridge
* Pre-deployed API is list-editor-v1: v1.0.0, under subdomain list-editor-v2: https://console.golem.cloud/projects/2056ef7f-d4ba-4c85-8c8a-db4af1f14884/apis/list-editor-v1/1.0.0
* There is a back up of this API too: v0.0.1, under subdmain listeditor: https://console.golem.cloud/projects/2056ef7f-d4ba-4c85-8c8a-db4af1f14884/apis/list-editor-v1/0.0.1
* Clone v1.0.0 and deploy into another subdomain list-editor-v3. If things goes wrong, we could show previously deployed APIs working.
