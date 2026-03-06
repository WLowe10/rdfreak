# rdfreak

Library for mapping rust types to RDF entities.

## Install

```sh
cargo add rdfreak
cargo add rdfreak_derive
```

## Usage

### Declaring Entities

The easiest way to declare an entity is using the derive macro from `rdfreak_derive`. When using the derive macro, one field of your entity struct must be marked with `#[rdf(subject)]`, and the rest must be marked with `#[rdf(predicate = "")]`.

```rust
use rdfreak_derive::Entity;

#[derive(Debug, Entity)]
#[rdf(type = "http://example.com/Animal")]
struct Animal {
    #[rdf(subject)]
    subject: NamedOrBlankNode,

    #[rdf(predicate = "http://example.com/name")]
    name: String,
}

#[derive(Debug, Entity)]
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

### Serializing & Deserializing Entities

```rs
// serialize entity into RDF

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

person.serialize(&mut graph);

// deserialize entity from RDF
let deserialized_person = Person::deserialize(&graph, &person.subject).unwrap();

let deserialized_rufus = Animal::deserialize(&graph, &NamedNode::new_unchecked("http://example.com/animals/Rufus").into())

// you can also deserialize all of a certain entity from a graph at once
let all_people = Person::deserialize_all(&graph).unwrap();
```