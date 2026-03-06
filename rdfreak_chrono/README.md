# rdfreak_chrono

Types for representing [chrono](https://github.com/chronotope/chrono) dates with [rdfreak](https://github.com/WLowe10/rdfreak).

## Install

```sh
cargo add rdfreak_chrono
```

## Usage

```rs
use oxrdf::NamedOrBlankNode;
use rdfreak_derive::Entity;
use rdfreak_chrono::DateTime;

// you can use DateTime as an entity property

#[derive(Debug, Entity)]
#[rdf(type = "http://example.com/User")]
struct User {
    #[rdf(subject)]
    subject: NamedOrBlankNode,

    #[rdf(predicate = "http://example.com/name")]
    name: String,

    #[rdf(predicate = "http://example.com/signedUpAt")]
	signedUpAt: DateTime,
}

// instantiate a DateTime
let my_date_time = DateTime::new(my_chrono_date_time);
```