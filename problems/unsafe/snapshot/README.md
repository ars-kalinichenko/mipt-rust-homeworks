# Snapshot

In this problem, you'll implement an object graph using the raw pointers.

## Preparation

You'll need unsafe Rust to solve this problem. Before solving, [install Miri](../../../docs/additional-setup.md#miri-interpreter).

## Description

There will be 4 entities in problem: `Node`, `NodeSegment`, `Pod` and `PodSet`.

- Each `Node` belongs to some `NodeSegment`.
- Each `PodSet` belongs to some `NodeSegment`.
- Each `Pod` belongs to some `PodSet`.
- Each `Pod` _can belong_ to some `Node`.

These entities are from Kubernetes, if you're interested in what they are exactly, you can [read here](https://kubernetesbootcamp.github.io/kubernetes-bootcamp/3-1.html).

Your task is to implement the `Snapshot` of such objects. Method `Snapshot::new` accepts a dump of objects as input - a set of structures that describe each object and its relationship to other objects. Using this dump you need to construct a snapshot - an object that is much more convenient to use.

## `Snapshot` API

- `.nodes()`, `.node_segments()`, `.pods()` and `.pod_sets()` return an iterator over all relevant snapshot objects.
- `.get_node(id)`, `.get_node_segment(id)`, `.get_pod(id)`, `.get_pod_set(id)` - return an object of the appropriate type by identifier (or `None`, if such object not found).

Entities methods:

- For `NodeSegment`:
  - `.nodes()` - iterator over all nodes of this segment.
  - `.pod_sets()` - iterator over all `PodSets` of this segment.
- For `Node`:
  - `.pods()` - iterator over all `Pods` of this node.
  - `.node_segment()` - `NodeSegment` of this node.
- For `PodSet`:
  - `.pods()` - iterator over all `Pods` of this pod set.
  - `.node_segment()` - `NodeSegment` of this subset.
- For `Pod`:
  - `.pod_set()` - `PodSet` of this pod.
  - `.node()` - `Node` of this pod, if any.

Each of these methods returns a reference (or iterator over references) to another snapshot object **with the same lifetime as** `&self`. Thus, having a snapshot, you can navigate over it: all references you receive have a snapshot lifetime.

## Object metrics

For each object you should calculate some metrics and put them into the attributes. Namely:

- For `PodSet`:
  - `resource_requests` - total count of resource requests for pods of this subset.
- For `Node`:
  - `resource_usage` - total resource usage of pods in this node.
- For `NodeSegment`:
  - `resource_usage` - total resource usage of nodes in this segment.
  - `resource_requests` - total count of resource requests of pod sets in this segment.
  - `resource_totals` - the sum of resource totals of nodes in this segment.
