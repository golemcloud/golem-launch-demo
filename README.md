# Golem 1.0 Launch Demo

## Description
In this demo we implement a **collaborative list editor application** using Golem. The application can handle an arbitrary number of simultaneously open **lists** - each list consists of a list of string items. These list items can be appended, inserted and deleted simultaneoulsy by multiple users; the current list state can be queried any time, as well as the active "editor" connections. Modification is only allowed for connected editors, and there is a `poll` function available for them which only returns the new changes since the last call.

Lists can be archived, in which case they are no longer editable and their contents are saved in a separate **list archive**. Then the list can be deleted, it's last state remains forever in archive.

An additional feature is that if a list is **not archived** and there are no changes for a certain period of time, all the connected editors are notified by sending an email to them. (Note: the demo does not actually implement the email sending, just prints a log line where it would do so.)

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

### Phase 3
In this step we impement the **archive functionality**.

First let's create a new component, now using the Go language:

```zsh
golem-cloud-cli new --lang go --package-name demo:archive archive
```

compile the initial version:

```zsh
cd archive
make build
```

Write the `archive` component's WIT definition (`prepared/phase-3/archive/wit/archive.wit`) and regenerate the bindings:

```zsh
make bindings
```

Then implement it (`prepared/phase-3/archive/src/main.go`) and compile again:

```zsh
make build
```

Generate a **stub** for the `archive` component:

```zsh
cd ..
golem-cloud-cli stubgen generate --source-wit-root archive/wit --dest-crate-root archive-stub
```

Build it:

```zsh
cd archive-stub
cargo component build --release
cd ..
```

And add the stub as a dependency to `lst`:

```zsh
golem-cloud-cli stubgen add-stub-dependency --stub-wit-root archive-stub/wit --dest-wit-root lst/wit --overwrite
```

See how the `wit/deps` directory now contains `demo-archive-stub`. Modify the `lst` WIT definition to include the stub, and to export archive functionality (`prepared/phase-3/lst/wit/main.wit`).

Regenerate bindings for `lst`:

```zsh
cd lst
npm run componentize
```

Implement the archive feature (`prepared/phase-3/lst/src/main.ts`):
- Add an `archive` flag to `State`
- Modify `add`, `delete` and `insert` to check it
- Implement `archive` and `isArchived`

Compile the `lst` component:

```zsh
npm run componentize
```

Get back to the root and compose the `lst.wasm` with the `archive-stub.wasm`:

```zsh
cd ..
golem-cloud-cli stubgen compose --source-wasm lst/out/lst.wasm --stub-wasm archive-stub/target/wasm32-wasi/release/archive_stub.wasm --dest-wasm lst/out/lst-composed.wasm
```

Before trying it out, first upload the new archive component and save it's URN and ID:

```zsh
golem-cloud-cli component add --project $PRJ --component-name archive archive/archive.wasm
export ARCHIVE=urn:component:c95c8c49-db39-4221-8721-f1f2b7e02a9d
export ARCHIVE_ID=c95c8c49-db39-4221-8721-f1f2b7e02a9d
```

Then update the list component with the new, composed version:

```zsh
golem-cloud-cli component update --component $LST lst/out/lst-composed.wasm
```

And try it out!
First we explitly create a new list, passing the archive component's ID:

```zsh
golem-cloud-cli worker start --component $LST --worker-name test3 --env "ARCHIVE_COMPONENT_ID=$ARCHIVE_ID"
```

Then invoke it a few times, then query if it's archived:

```zsh
golem-cloud-cli worker invoke-and-await --component $LST --worker-name test3 --function 'demo:lst/api.{connect}' --arg '"vigoo@golem.cloud"'
golem-cloud-cli worker invoke-and-await --component $LST --worker-name test3 --function 'demo:lst/api.{add}' --arg '{id: 1}' --arg '"item 1"'
golem-cloud-cli worker invoke-and-await --component $LST --worker-name test3 --function 'demo:lst/api.{add}' --arg '{id: 1}' --arg '"item 3"'
golem-cloud-cli worker invoke-and-await --component $LST --worker-name test3 --function 'demo:lst/api.{get}'
golem-cloud-cli worker invoke-and-await --component $LST --worker-name test3 --function 'demo:lst/api.{is-archived}'
```

At this point the archive worker does not exist yet:

```zsh
golem-cloud-cli worker list --component $ARCHIVE
```

Let's archive our list:

```zsh
golem-cloud-cli worker invoke-and-await --component $LST --worker-name test3 --function 'demo:lst/api.{archive}'
golem-cloud-cli worker invoke-and-await --component $LST --worker-name test3 --function 'demo:lst/api.{is-archived}'
```

And see the archive worker:

```zsh
golem-cloud-cli worker list --component $ARCHIVE
```

Try to query it:

```zsh
golem-cloud-cli worker invoke-and-await --component $ARCHIVE --worker-name archive --function 'demo:archive/api.{get-all}'
```
