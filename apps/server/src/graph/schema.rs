use neo4j::{Graph, query};

pub async fn init_schema(graph: &Graph) -> Result<(), neo4j::Error> {
    // Уникальность Person.id
    graph
        .run(query(
            "
        CREATE CONSTRAINT IF NOT EXISTS
        FOR (p:Person)
        REQUIRE p.id IS UNIQUE
    ",
        ))
        .await?;

    // Индекс по created_by_user_id для быстрого поиска всех персон пользователя
    graph
        .run(query(
            "
        CREATE INDEX IF NOT EXISTS
        FOR (p:Person)
        ON (p.created_by_user_id)
    ",
        ))
        .await?;

    Ok(())
}
