Nutriledger backend
===================

Local services
--------------

Requirements: Docker and Terraform 1.5 or later.

Set a local PostgreSQL password and start the development services:

	export TF_VAR_postgres_password='choose-a-local-password'
	terraform -chdir=terraform init
	terraform -chdir=terraform apply

Terraform builds the upload_pipeline image and starts PostgreSQL and Redpanda.
On a new PostgreSQL volume, the SQL files in supabase/migrations are applied
by the official PostgreSQL image. The importer is not started automatically.

Run the CSV importer manually after PostgreSQL has initialized. The local CSV
directory is mounted read-only instead of being copied into the image:

	docker run --rm --network nutriledger \
	--volume "$PWD/src/data:/data:ro" \
	  -e "DATABASE_URL=postgresql://nutriledger:${TF_VAR_postgres_password}@nutriledger-postgres:5432/nutriledger?sslmode=disable" \
	  nutriledger-backend:local

PostgreSQL is available on localhost:5432. Redpanda's Kafka API is available
to host clients at localhost:19092 and to containers on the nutriledger
network at nutriledger-redpanda:9092.

The PostgreSQL volume persists across container restarts. The migration
scripts run only when that volume is first initialized. Running
`terraform destroy` also removes the Terraform-managed volume and its data.

CI
--

GitHub Actions checks Rust formatting, Clippy, and tests; validates Terraform;
and builds the Docker image on pull requests and pushes to main. It does not
apply infrastructure or publish the image.
