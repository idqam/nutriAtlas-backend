terraform {
  required_version = ">= 1.5.0"

  required_providers {
    docker = {
      source  = "kreuzwerker/docker"
      version = "~> 3.0"
    }
  }
}

provider "docker" {}

variable "postgres_password" {
  description = "Password for the local development PostgreSQL container."
  type        = string
  sensitive   = true
}

resource "docker_network" "nutriledger" {
  name = "nutriledger"
}

resource "docker_volume" "postgres_data" {
  name = "nutriledger-postgres-data"
}

resource "docker_image" "postgres" {
  name         = "postgres:16-alpine"
  keep_locally = true
}

resource "docker_image" "redpanda" {
  name         = "redpandadata/redpanda:v25.1.1"
  keep_locally = true
}

resource "docker_image" "backend" {
  name         = "nutriledger-backend:local"
  keep_locally = true

  build {
    context    = "${path.module}/.."
    dockerfile = "Dockerfile"
  }
}

resource "docker_container" "postgres" {
  name  = "nutriledger-postgres"
  image = docker_image.postgres.image_id

  env = [
    "POSTGRES_DB=nutriledger",
    "POSTGRES_USER=nutriledger",
    "POSTGRES_PASSWORD=${var.postgres_password}",
  ]

  ports {
    internal = 5432
    external = 5432
  }

  volumes {
    volume_name    = docker_volume.postgres_data.name
    container_path = "/var/lib/postgresql/data"
  }

  volumes {
    host_path      = abspath("${path.module}/../supabase/migrations")
    container_path = "/docker-entrypoint-initdb.d"
    read_only      = true
  }

  networks_advanced {
    name = docker_network.nutriledger.name
  }
}

resource "docker_container" "redpanda" {
  name  = "nutriledger-redpanda"
  image = docker_image.redpanda.image_id

  command = [
    "redpanda", "start",
    "--overprovisioned",
    "--smp", "1",
    "--memory", "1G",
    "--reserve-memory", "0M",
    "--node-id", "0",
    "--check=false",
    "--kafka-addr", "internal://0.0.0.0:9092,external://0.0.0.0:19092",
    "--advertise-kafka-addr", "internal://nutriledger-redpanda:9092,external://localhost:19092",
  ]

  ports {
    internal = 19092
    external = 19092
  }

  networks_advanced {
    name = docker_network.nutriledger.name
  }
}