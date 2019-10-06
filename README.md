Augorama (for rust)
==========

# UNDER CONSTRUCTION

# UNDER CONSTRUCTION

# UNDER CONSTRUCTION

```console
# generate docs
cargo doc --no-deps  

# generate bin
cargo build
./target/debug/augorama

# run in developer mode
RUST_LOG=debug cargo watch -i "examples/**" -x run 
```

# Overview

The Augorama module implements the entry point to the Augorama actor system.

Augorama is a server hosting a graph of digital twins (Augorama actors) of real and virtual
things in the world.  An actor can answer questions about its twin.  Answers can include
facts about its twin's current state, explanations about current state referencing past state
(explainability), and predictions about the twin's future state (AI).

The actors maintain their state by watching telemetry collected about their twins.  Their state
consists of counts and sums and statistics.  Their state is backed by event sourcing
persistence.

The telemetry driving the advancement over time of actor state is collected from an HTTP(S)
endpoint.  The data posted to the endpoint is structured - json by default - of any structure.
Telemetry is extracted from the structure with path specifications - `jsonpath` by default.
The separation of incoming data schema from telemetry via path means any structured data can be
posted to the endpoint, that there is no "Augorama data schema".  If a source can emit json, it
can be used to create telemetry necessary to maintain a digital twin (Augorama actor).
