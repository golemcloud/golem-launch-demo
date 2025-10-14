# Golem 1.3.0 launch demo

## Description

In this demo we implement a collaborative list editor application using Golem. The application can handle an arbitrary
number of simultaneously open lists—each list consists of a list of string items. These list items can be appended,
inserted and deleted simultaneously by multiple users; the current list state can be queried any time, as well as the
active "editor" connections. Modification is only allowed for connected editors, and there is a poll function available
for them that is only going to return the new changes since the last call.

Lists can be archived, in which case they are no longer editable and their contents are saved in a separate list
archive. Then the list can be deleted; its last state remains forever in the archive.

An additional feature is that if a list is not archived and there are no changes for a certain period of time, all the
connected editors are notified by sending an email to them.

## Demo

### Phase 1

First, we create a new Golem agent, and write the simplest possible version of the above - no connections, just an agent
representing a list, with add/insert/delete/get operations.

Let's create the new app:

```shell
golem app new
> demo
> TypeScript
> default
> demo:colist
> Create application
```

Build it:

```shell
cd demo
golem app build
```

Let's see what directories we have.

- Note that `common-ts` is for code shared between components
- Note that multiple components can belong to an application for advanced use cases

Our new component is in `components-ts/demo-colist`.

- Check `src/main.ts`

Change it to have a first version of our list agent.

Before trying it out, check `golem.yaml` of the component. Explain the example HTTP API mapping and comment it out for
now.

```shell
golem app build
```

Simplest way to try out: Golem REPL

```shell
golem repl
```

Shows the following sheet:

```
╔═
║ Started Rib REPL for component demo:colist using version 0
║
║ Component name:    demo:colist
║ Component ID:      34ad07d6-3a4b-4a3e-b517-4f1a5d99e942
║ Component type:    Durable
║ Component version: 0
║ Project ID:        4b692549-e034-4ebf-b7b2-fbbfa0a3e15e
║ Component size:    4.72 MiB
║ Created at:        2025-10-14 12:16:50.331931 UTC
║ Exports:
║   list-agent(name: string) agent constructor
║   list-agent.add(item: string) ->
║   list-agent.insert(after: string, item: string) ->
║   list-agent.delete(item: string) ->
║   list-agent.list() -> return-value: list<string>
╚═
```

Try it out:

```
>>> let l1 = list-agent("test1")
()
>>> l1.add("item1")
()
>>> l1.add("item2")
()
>>> l1.insert("item1", "item1b")
()
>>> l1.list()
[
  "item1",
  "item1b",
  "item2"
]
>>>
```

### Phase 2

We have seen how to write simple code and run it with golem. Let's make it more realistic by requiring users to connect,
only allow connected users to modify the list, and also collect a per-user list of changes that can be polled.

Try it out:

```shell
golem app build
golem app deploy --reset
golem repl
```

```
>>> let l1 = list-agent("test1")
()
>>> let vigoo = l1.connect("vigoo@golem.cloud")
()
>>> let john = l1.connect("john@golem.cloud")

()
>>>
>>> l1.add(vigoo, "item 1")
[2025-10-14T13:20:07.284Z] [STDOUT  ] Adding event { tag: 'added', val: 'item 1' }
ok(1)
>>> l1.add(vigoo, "item 3")
[2025-10-14T13:20:11.176Z] [STDOUT  ] Adding event { tag: 'added', val: 'item 3' }
ok(2)
>>> l1.insert(john, "item 1", "item 2")
[2025-10-14T13:20:13.324Z] [STDOUT  ] Adding event { tag: 'inserted', val: { after: 'item 1', value: 'item 2' } }
ok(3)
>>> l1.get()
[
  "item 1",
  "item 2",
  "item 3"
]
>>> l1.poll(vigoo)
ok([
  added("item 1"),
  added("item 3"),
  inserted({
    after: "item 1",
    value: "item 2"
  })
])
>>> l1.poll(john)
ok([
  added("item 1"),
  added("item 3"),
  inserted({
    after: "item 1",
    value: "item 2"
  })
])
>>> l1.disconnect(john)
true
>>> l1.poll(vigoo)
ok([])
>>> l1.add("item 4")
[compilation error]
[position] 1
[expression] l1.add("item 4")
[cause] invalid argument size for function `add`. expected 2 arguments, found 1
>>> l1.add(vigoo, "item 4")
[2025-10-14T13:20:42.236Z] [STDOUT  ] Adding event { tag: 'added', val: 'item 4' }
ok(4)
>>> l1.poll(vigoo)
ok([
  added("item 4")
])
>>>
```

### Phase 3

Now that we have a working collaborative list editor, let's add the archiving feature. 

Let's add a new cluster level singleton agent first.

Then we add an `archive` method to our list agent, and some additional checks to prevent modifications after archiving.

```shell
golem app build
golem app deploy --update-agents auto
golem repl
```

Then in REPL:

```shell
>>> let l1 = list-agent("test1")
()
>>> l1.get()
[
  "item 1",
  "item 2",
  "item 3"
]
>>> l1.archive()
>>> let archive = archive-agent()
()
>>> archive.get-all()
[
  {
    name: "test1",
    items: [
      "item 1",
      "item 2",
      "item 3"
    ]
  }
]
>>>
>>> let vigoo = l1.connect("vigoo@golem.cloud")
()
>>> l1.add(vigoo, "test 4")
err(archived)
>>>
```

Then outside of REPL, we can see we have two agents running:

```shell
golem agent list
```

```
+----------------+---------------------+-----------+--------+-----------------------------+
| Component name | Agent name          | Component | Status | Created at                  |
|                |                     | version   |        |                             |
+----------------+---------------------+-----------+--------+-----------------------------+
| demo:colist    | archive-agent()     |         3 |   Idle | 2025-10-14 13:42:04.211 UTC |
+----------------+---------------------+-----------+--------+-----------------------------+
| demo:colist    | list-agent("test1") |         3 |   Idle | 2025-10-14 13:41:46.400 UTC |
+----------------+---------------------+-----------+--------+-----------------------------+
```

### Phase 4
The final missing piece is sending an email notification after a certain period of time.
Because Golem agents are single-threaded and do have background async work outside their invoked methods, we need to start a background agent to do this.

When we create a new list, we will always start a list-notification agent as well for each. This will execute a long-running method that sleeps for a given time, checks the list's last change time, and if necessary sends an email to all connected editors,
otherwise sleeps again.

