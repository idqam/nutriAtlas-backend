use std::{
    collections::HashSet,
    env,
    error::Error,
    fs::File,
    io,
    path::{Path, PathBuf},
};

use bytes::Bytes;
use csv::{ReaderBuilder, StringRecord, WriterBuilder};
use futures_util::SinkExt;
use postgres_native_tls::MakeTlsConnector;
use tokio_postgres::Client;

type PipelineResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

const COPY_CHUNK_ROWS: u64 = 1_000;

struct ImportSpec {
    file_name: &'static str,
    table_name: &'static str,
}

struct ColumnMapping {
    source_index: usize,
    target_name: String,
}

struct ImportStats {
    file_name: &'static str,
    staged_rows: u64,
    inserted_rows: u64,
}

const IMPORT_SPECS: &[ImportSpec] = &[
    ImportSpec {
        file_name: "food_category.csv",
        table_name: "food_category",
    },
    ImportSpec {
        file_name: "nutrient.csv",
        table_name: "nutrient",
    },
    ImportSpec {
        file_name: "measure_unit.csv",
        table_name: "measure_unit",
    },
    ImportSpec {
        file_name: "food_attribute_type.csv",
        table_name: "food_attribute_type",
    },
    ImportSpec {
        file_name: "food.csv",
        table_name: "food",
    },
    ImportSpec {
        file_name: "food copy.csv",
        table_name: "food",
    },
    ImportSpec {
        file_name: "branded_food.csv",
        table_name: "branded_food",
    },
    ImportSpec {
        file_name: "food_nutrient.csv",
        table_name: "food_nutrient",
    },
    ImportSpec {
        file_name: "food_nutrient copy.csv",
        table_name: "food_nutrient",
    },
    ImportSpec {
        file_name: "food_portion.csv",
        table_name: "food_portion",
    },
    ImportSpec {
        file_name: "food_attribute.csv",
        table_name: "food_attribute",
    },
    ImportSpec {
        file_name: "food_attribute copy.csv",
        table_name: "food_attribute",
    },
    ImportSpec {
        file_name: "input_food.csv",
        table_name: "input_food",
    },
];

#[tokio::main]
async fn main() {
    if let Err(error) = run().await {
        eprintln!("ETL failed: {error}");
        std::process::exit(1);
    }
}

async fn run() -> PipelineResult<()> {
    let database_url = env::var("DATABASE_URL")?;
    let data_dir = env::var_os("DATA_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("src/data"));

    validate_input_files(&data_dir)?;

    let tls_connector = native_tls::TlsConnector::builder().build()?;
    let (mut client, connection) =
        tokio_postgres::connect(&database_url, MakeTlsConnector::new(tls_connector)).await?;

    tokio::spawn(async move {
        if let Err(error) = connection.await {
            eprintln!("PostgreSQL connection error: {error}");
        }
    });

    verify_schema(&client).await?;

    let mut total_staged = 0;
    let mut total_inserted = 0;

    for spec in IMPORT_SPECS {
        let path = data_dir.join(spec.file_name);
        let stats = import_file(&mut client, spec, &path)
            .await
            .map_err(|error| io::Error::other(format!("{}: {error}", spec.file_name)))?;

        println!(
            "{}: staged {}, inserted {} (existing keys skipped)",
            stats.file_name, stats.staged_rows, stats.inserted_rows
        );
        total_staged += stats.staged_rows;
        total_inserted += stats.inserted_rows;
    }

    refresh_food_search(&client).await?;

    println!("Import complete: {total_inserted} rows inserted from {total_staged} staged rows.");
    Ok(())
}

fn validate_input_files(data_dir: &Path) -> PipelineResult<()> {
    if !data_dir.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("CSV directory does not exist: {}", data_dir.display()),
        )
        .into());
    }

    let missing: Vec<_> = IMPORT_SPECS
        .iter()
        .filter(|spec| !data_dir.join(spec.file_name).is_file())
        .map(|spec| spec.file_name)
        .collect();

    if !missing.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("missing CSV files: {}", missing.join(", ")),
        )
        .into());
    }

    Ok(())
}

async fn verify_schema(client: &Client) -> PipelineResult<()> {
    let schema_exists: bool = client
        .query_one(
            "SELECT to_regclass('public.food') IS NOT NULL\
             AND to_regclass('public.food_search') IS NOT NULL",
            &[],
        )
        .await?
        .get(0);

    if !schema_exists {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "food schema is missing; apply the Supabase/PostgreSQL migration first",
        )
        .into());
    }

    Ok(())
}

async fn import_file(
    client: &mut Client,
    spec: &ImportSpec,
    path: &Path,
) -> PipelineResult<ImportStats> {
    let file = File::open(path)?;
    let mut reader = ReaderBuilder::new().flexible(false).from_reader(file);
    let headers = reader.headers()?.clone();
    let target_columns = fetch_table_columns(client, spec.table_name).await?;
    let mappings = map_columns(spec.table_name, &headers, &target_columns)?;

    let tx = client.transaction().await?;
    tx.batch_execute(&format!(
        "CREATE TEMP TABLE etl_stage (LIKE public.{}) ON COMMIT DROP",
        quote_identifier(spec.table_name)
    ))
    .await?;

    let copy_columns = mappings
        .iter()
        .map(|mapping| quote_identifier(&mapping.target_name))
        .collect::<Vec<_>>()
        .join(", ");
    let copy_sql = format!("COPY etl_stage ({copy_columns}) FROM STDIN WITH (FORMAT CSV, NULL '')");
    let mut copy_sink = std::pin::pin!(tx.copy_in(&copy_sql).await?);
    let mut csv_writer = WriterBuilder::new()
        .has_headers(false)
        .from_writer(Vec::with_capacity(64 * 1024));
    let mut source_rows = 0;

    for result in reader.records() {
        let record = result?;
        let values = mappings
            .iter()
            .map(|mapping| record.get(mapping.source_index).unwrap_or_default())
            .collect::<Vec<_>>();
        csv_writer.write_record(values)?;
        source_rows += 1;

        if source_rows % COPY_CHUNK_ROWS == 0 {
            csv_writer.flush()?;
            let finished_writer = std::mem::replace(
                &mut csv_writer,
                WriterBuilder::new()
                    .has_headers(false)
                    .from_writer(Vec::with_capacity(64 * 1024)),
            );
            let chunk = finished_writer
                .into_inner()
                .map_err(|error| error.into_error())?;
            copy_sink.as_mut().send(Bytes::from(chunk)).await?;
        }
    }

    csv_writer.flush()?;
    let final_chunk = csv_writer
        .into_inner()
        .map_err(|error| error.into_error())?;
    if !final_chunk.is_empty() {
        copy_sink.as_mut().send(Bytes::from(final_chunk)).await?;
    }
    let staged_rows = copy_sink.as_mut().finish().await?;

    let insert_columns = mappings
        .iter()
        .map(|mapping| quote_identifier(&mapping.target_name))
        .collect::<Vec<_>>()
        .join(", ");
    let insert_sql = format!(
        "INSERT INTO public.{} ({insert_columns}) \
         SELECT {insert_columns} FROM etl_stage \
         ON CONFLICT DO NOTHING",
        quote_identifier(spec.table_name)
    );
    let inserted_rows = tx.execute(&insert_sql, &[]).await?;
    tx.commit().await?;

    Ok(ImportStats {
        file_name: spec.file_name,
        staged_rows,
        inserted_rows,
    })
}

async fn fetch_table_columns(client: &Client, table_name: &str) -> PipelineResult<Vec<String>> {
    let rows = client
        .query(
            "SELECT column_name FROM information_schema.columns\
             WHERE table_schema = 'public' AND table_name = $1\
             ORDER BY ordinal_position",
            &[&table_name],
        )
        .await?;

    let columns = rows
        .iter()
        .map(|row| row.get::<_, String>(0))
        .collect::<Vec<_>>();

    if columns.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("destination table public.{table_name} does not exist"),
        )
        .into());
    }

    Ok(columns)
}

fn map_columns(
    table_name: &str,
    headers: &StringRecord,
    target_columns: &[String],
) -> PipelineResult<Vec<ColumnMapping>> {
    let mut mappings = Vec::with_capacity(headers.len());
    let mut mapped_targets = HashSet::new();

    for (source_index, source_header) in headers.iter().enumerate() {
        let clean_header = source_header
            .trim_start_matches('\u{feff}')
            .trim()
            .to_ascii_lowercase();
        if table_name == "food" && clean_header == "microbe_data" {
            continue;
        }
        let target_name = match (table_name, clean_header.as_str()) {
            _ => clean_header,
        };

        if !target_columns.iter().any(|column| column == &target_name) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("CSV header '{source_header}' has no destination in public.{table_name}"),
            )
            .into());
        }
        if !mapped_targets.insert(target_name.clone()) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("multiple CSV headers map to {target_name} in public.{table_name}"),
            )
            .into());
        }

        mappings.push(ColumnMapping {
            source_index,
            target_name,
        });
    }

    if mappings.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("CSV has no usable columns for public.{table_name}"),
        )
        .into());
    }

    Ok(mappings)
}

async fn refresh_food_search(client: &Client) -> PipelineResult<()> {
    client
        .batch_execute("REFRESH MATERIALIZED VIEW public.food_search")
        .await?;
    Ok(())
}

fn quote_identifier(identifier: &str) -> String {
    format!("\"{}\"", identifier.replace('"', "\"\""))
}
