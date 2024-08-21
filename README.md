# Golem 1.0 Launch Demo

## Description
TODO

## Demo script

### Phase 1
In phase 1 we create the first version of the `lst` component, where each worker represents a stateful list and provides some basic manipulation and query functionalities.

Create the new component:

```zsh
golem-cloud-cli new --lang ts --package-name demo:lst lst
```

Compile the initial version:

```zsh
cd lst
npm install
npm run componentize
```

We have a Golem component in `out/lst.wasm.`.

Apply the required changes to the WIT file (`prepared/phase-1/lst/wit/main.wit`) and regenerate the bindings:

```zsh
npm run componentize
```

Then implement the first version (`prepared/phase-1/lst/src/main.ts`) and compile again:

```zsh
npm run componentize
```

Let's set the **project ID** we are working with (Golem Cloud only) to an environment variable:

```zsh
export PRJ=urn:project:5e12b227-a084-4d31-8fea-02da49c1f9c7 # Preparation test project
export PRJ=urn:project:b17d7bbf-9704-4578-bc25-8b1ad22f3f3a # Live demo project
```

Deploy the component and store it's ID in an environment variable:

```zsh
golem-cloud-cli component add --project $PRJ --component-name lst out/lst.wasm
export LST=urn:component:4a3d6c13-9086-43d6-88c6-be4faeedc1f7
```

Try it out:

```zsh
golem-cloud-cli worker invoke-and-await --component $LST --worker-name test1 --function 'demo:lst/api.{add}' --arg '"item 1"'
golem-cloud-cli worker invoke-and-await --component $LST --worker-name test1 --function 'demo:lst/api.{add}' --arg '"item 3"'
golem-cloud-cli worker invoke-and-await --component $LST --worker-name test1 --function 'demo:lst/api.{insert}' --arg '"item 1"' --arg '"item 2"'
golem-cloud-cli worker invoke-and-await --component $LST --worker-name test1 --function 'demo:lst/api.{get}'
```

### Phase 2
In phase 2 we add the concept of a `connection` and polling.

First we modify the WIT file (`prepared/phase-2/lst/wit/main.wit`) and regenerate the bindings:

```zsh
npm run componentize
```

Then implement the changes (`prepared/phase-2/lst/src/main.ts`)
- Define `EditorState`
- Add a `connected` map and a `lastConnectionId` variable to `State`
- Write two helper functions: `isConnected` and `addEvent`
- Modify the existing exported functions
- Write the new ones

and compile again:

```zsh
npm run componentize
```

Update the project:

```zsh
golem-cloud-cli component update --component $LST out/lst.wasm
```

Note that this did not update the existing worker, but new workers will use the new version.

Try it out (using a new worker name to use the updated version):

```zsh
golem-cloud-cli worker invoke-and-await --component $LST --worker-name test2 --function 'demo:lst/api.{connect}' --arg '"vigoo@golem.cloud"'
golem-cloud-cli worker invoke-and-await --component $LST --worker-name test2 --function 'demo:lst/api.{connect}' --arg '"john@golem.cloud"'
golem-cloud-cli worker invoke-and-await --component $LST --worker-name test2 --function 'demo:lst/api.{add}' --arg '{id: 1}' --arg '"item 1"'
golem-cloud-cli worker invoke-and-await --component $LST --worker-name test2 --function 'demo:lst/api.{add}' --arg '{id: 1}' --arg '"item 3"'
golem-cloud-cli worker invoke-and-await --component $LST --worker-name test2 --function 'demo:lst/api.{insert}' --arg '{id: 2}' --arg '"item 1"' --arg '"item 2"'
golem-cloud-cli worker invoke-and-await --component $LST --worker-name test2 --function 'demo:lst/api.{get}'
golem-cloud-cli worker invoke-and-await --component $LST --worker-name test2 --function 'demo:lst/api.{connected-editors}'
golem-cloud-cli worker invoke-and-await --component $LST --worker-name test2 --function 'demo:lst/api.{poll}' --arg '{id: 1}'
golem-cloud-cli worker invoke-and-await --component $LST --worker-name test2 --function 'demo:lst/api.{poll}' --arg '{id: 2}'
golem-cloud-cli worker invoke-and-await --component $LST --worker-name test2 --function 'demo:lst/api.{add}' --arg '{id: 1}' --arg '"item 4"'
golem-cloud-cli worker invoke-and-await --component $LST --worker-name test2 --function 'demo:lst/api.{poll}' --arg '{id: 1}'
golem-cloud-cli worker invoke-and-await --component $LST --worker-name test2 --function 'demo:lst/api.{add}' --arg '{id: 1}' --arg '"item 5"'
golem-cloud-cli worker invoke-and-await --component $LST --worker-name test2 --function 'demo:lst/api.{poll}' --arg '{id: 1}'
golem-cloud-cli worker invoke-and-await --component $LST --worker-name test2 --function 'demo:lst/api.{poll}' --arg '{id: 2}'

```
