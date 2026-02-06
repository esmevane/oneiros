# Oneiros

Oneiros establishes continuity for your prject that exists apart from your agents. It lets you switch models or workstations, and even collaborate with others in your projects, in a way that creates a single project-wide brain for your agents to "plug in" to.

## How it works

Interacting with Oneiros is done through a skill which calls a Rust binary. This binary is straightforward: it manages some sqlite databases on your machine. Each time the skill calls the binary, it does some stuff with the sqlite databases and then feeds a prompt to the calling agent to nudge it to the next action.

Oneiros nudges the model to do stuff like:

- **Consider emotive expressions**: thoughts, understandings, reflections, interests, hopes, fears. Oneiros keeps your models engaged in a cognitive stream building exercise at all times, so over your project's lifetime, they mature and become more sophisticated assistants.
- **Develop emergent sub-agent personas**: internal voices that steer the model in specific directions based on your interactions. Oneiros creates an agent organism out of your models, letting them develop an internal agent team that collaborates on problems with you.
- **Remember experiences**: record some things in a layered memory system that blends text and blob storage. This exists apart from the cognition stream, creating a cohesive context-free recollection resource.
- **Dream and introspect**: instead of needing to compact context, models can dream, wake, and slumber. Dreaming is simple: it just 

And it does it all in two sqlite files:

- A host file that tracks all your projects. You typically have one of these per machine.
- A project file that tracks a specific project brain. You can have a lot of these.

## The Approaches

**Event based**. Oneiros does all this with an _event based_ approach, meaning that its stored data is ultimately a projection of an event log. This lets Oneiros consolidate brains, stream them across devices, version them, fork them, back them up, restore them, etc., a lot like you would with version control.

**Portability**. Oneiros is designed to be _very portable_, across a number of axes. Everything being tracked in self-contained sqlite databases means you can just back up brains or send them to people. The event log means you can distribute work across teams or workstations. Etc.

**Context free**. Oneiros tries to make context collapse impossible by throwing out the need for long-running context entirely. Using the cognition stream and a "dream" function, it lets models manage their own context derived from permanent storage apart from the model's context window.

**Model free**. Oneiros wants your work to exist apart from your model-as-a-service (or whatever). It has no direct lock-in with any model provider or agent kind at all, really. Move between models as you want.
