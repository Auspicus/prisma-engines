//! Creation of a describer instance.

use psl::builtin_connectors::POSTGRES;
use quaint::prelude::{ConnectionInfo, Queryable, SqlFamily};
use sql_schema_describer::{postgres::Circumstances, SqlSchemaDescriberBackend};

/// Create a correct describer instance for the given database.
pub async fn load_describer<'a>(
    connection: &'a quaint::single::Quaint,
    connection_info: &ConnectionInfo,
    provider: Option<&str>,
) -> Result<Box<dyn SqlSchemaDescriberBackend + 'a>, crate::SqlError> {
    let version = connection.version().await?;

    Ok(match connection_info.sql_family() {
        SqlFamily::Postgres => {
            let mut circumstances = Default::default();

            if version.map(|version| version.contains("CockroachDB")).unwrap_or(false) {
                circumstances |= Circumstances::Cockroach;

                if provider == Some(POSTGRES.provider_name()) {
                    circumstances |= Circumstances::CockroachWithPostgresNativeTypes;
                }
            } else {
                let pgversion_result = connection
                    .query_raw("select current_setting('server_version_num')::integer as version;", &[])
                    .await?;
                let pgversion = pgversion_result
                    .get(0)
                    .and_then(|r| r.get("version"))
                    .and_then(|v| v.as_integer());

                match pgversion {
                    Some(version) if version >= 100000 => circumstances |= Circumstances::CanPartitionTables,
                    _ => (),
                }
            }

            Box::new(sql_schema_describer::postgres::SqlSchemaDescriber::new(
                connection,
                circumstances,
            )) as Box<dyn SqlSchemaDescriberBackend>
        }
        SqlFamily::Mysql => Box::new(sql_schema_describer::mysql::SqlSchemaDescriber::new(connection)),
        SqlFamily::Sqlite => Box::new(sql_schema_describer::sqlite::SqlSchemaDescriber::new(connection)),
        SqlFamily::Mssql => Box::new(sql_schema_describer::mssql::SqlSchemaDescriber::new(connection)),
    })
}
