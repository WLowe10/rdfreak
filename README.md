# rdfreak

Library for mapping rust types to RDF resources. rdfreak is built on top of types in the [oxigraph](https://github.com/oxigraph/oxigraph) ecosystem.

## Install

```sh
cargo add rdfreak
cargo add rdfreak_derive
```

## Usage

### Declaring Entities

The easiest way to declare a resource is using the derive macro from `rdfreak_derive`. When using the derive macro, one field of your resource struct must be marked with `#[rdf(subject)]`, and the rest must be marked with `#[rdf(predicate = "")]`.

```rust
use oxrdf::NamedOrBlankNode;

use rdfreak::{Resource, ToRdf, FromRdf};
use rdfreak_derive::{Resource, ToRdf, FromRdf};

#[derive(Debug, Resource, ToRdf, FromRdf)]
#[rdf(type = "http://example.com/Animal")]
struct Animal {
    #[rdf(subject)]
    subject: NamedOrBlankNode,

    #[rdf(predicate = "http://example.com/name")]
    name: String,
}

#[derive(Debug, Resource, ToRdf, FromRdf)]
#[rdf(type = "http://example.com/Person")]
struct Person {
    #[rdf(subject)]
    subject: NamedOrBlankNode,

    #[rdf(predicate = "http://example.com/name")]
    name: String,

    #[rdf(predicate = "http://example.com/hasPet")]
    pets: Vec<Animal>,
}
```

### Serializing & Deserializing Resources

```rs
// serialize resource into RDF

let person = Person {
	subject: NamedNode::new_unchecked("http://example.com/people/Alice").into(),
	name: "Alice".to_owned(),
	pets: vec![
		Animal {
			subject: NamedNode::new_unchecked("http://example.com/animals/Fluffy").into(),
			name: "Fluffy".to_owned(),
		},
		Animal {
			subject: NamedNode::new_unchecked("http://example.com/animals/Rufus").into(),
			name: "Rufus".to_owned(),
		},
	],
};

let mut graph = oxrdf::Graph::new();

person.to_rdf(&mut graph);

// deserialize resource from RDF
let deserialized_person = Person::from_rdf(&graph, &person.subject).unwrap();
```