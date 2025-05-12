use neo4j::{Graph, query};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub id: String,
    pub name: String,
    pub birth_date: String,       // можно сделать chrono::NaiveDate
    pub gender: String,           // male / female / other
    pub created_by_user_id: Uuid, // user_id из Postgres
}

impl Person {
    pub async fn create(graph: &Graph, person: &Person) -> Result<(), neo4j::Error> {
        let q = query(
            "
            CREATE (:Person {
                id: $id,
                name: $name,
                birth_date: $birth_date,
                gender: $gender,
                created_by_user_id: $created_by_user_id
            })
        ",
        )
        .param("id", &person.id)
        .param("name", &person.name)
        .param("birth_date", &person.birth_date)
        .param("gender", &person.gender)
        .param("created_by_user_id", &person.created_by_user_id);

        graph.run(q).await?;
        Ok(())
    }

    pub async fn link_parent(
        graph: &Graph,
        parent_id: &str,
        child_id: &str,
    ) -> Result<(), neo4j::Error> {
        let q = query(
            "
            MATCH (parent:Person {id: $parent_id}), (child:Person {id: $child_id})
            CREATE (parent)-[:PARENT_OF]->(child)
        ",
        )
        .param("parent_id", parent_id)
        .param("child_id", child_id);

        graph.run(q).await?;
        Ok(())
    }

    pub async fn link_marriage(
        graph: &Graph,
        person1_id: &str,
        person2_id: &str,
    ) -> Result<(), neo4j::Error> {
        let q = query(
            "
            MATCH (p1:Person {id: $person1_id}), (p2:Person {id: $person2_id})
            CREATE (p1)-[:MARRIED_TO]->(p2),
                   (p2)-[:MARRIED_TO]->(p1)
        ",
        )
        .param("person1_id", person1_id)
        .param("person2_id", person2_id);

        graph.run(q).await?;
        Ok(())
    }

    pub async fn link_siblings(
        graph: &Graph,
        person1_id: &str,
        person2_id: &str,
    ) -> Result<(), neo4j::Error> {
        let q = query(
            "
            MATCH (p1:Person {id: $person1_id}), (p2:Person {id: $person2_id})
            CREATE (p1)-[:SIBLING_OF]->(p2),
                   (p2)-[:SIBLING_OF]->(p1)
        ",
        )
        .param("person1_id", person1_id)
        .param("person2_id", person2_id);

        graph.run(q).await?;
        Ok(())
    }
}
