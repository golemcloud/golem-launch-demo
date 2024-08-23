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

## Cloud
* Pre-deployed API is list-editor-v1: v1.0.0, under subdomain list-editor-v2
* Clone this and deploy into another subdomain list-editor-v3
