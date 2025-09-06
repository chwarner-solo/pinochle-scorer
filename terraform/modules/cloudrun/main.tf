# modules/cloudrun/main.tf

resource "google_cloud_run_v2_service" "api" {
  project  = var.project
  name     = var.service_name
  location = var.region
  ingress  = "INGRESS_TRAFFIC_ALL"
  deletion_protection = false

  # Ignore changes to the container image - managed by CI/CD
  lifecycle {
    ignore_changes = [
      template[0].containers[0].image
    ]
  }

  template {
    service_account = var.service_account_email

    scaling {
      min_instance_count = var.min_instances
      max_instance_count = var.max_instances
    }

    dynamic "vpc_access" {
      for_each = var.vpc_connector_name != null ? [1] : []
      content {
        connector = "projects/${var.project}/locations/${var.region}/connectors/${var.vpc_connector_name}"
        egress    = "PRIVATE_RANGES_ONLY"
      }
    }

    containers {
      # Start with placeholder image - GitHub Actions will replace this
      image = var.container_image != "" ? var.container_image : "gcr.io/cloudrun/hello"

      ports {
        container_port = var.port
      }

      resources {
        limits = {
          cpu    = var.cpu_limit
          memory = var.memory_limit
        }
      }

      # Health check probes for your Rust application
      startup_probe {
        http_get {
          path = var.health_path
          port = var.port
        }
        initial_delay_seconds = 10
        timeout_seconds = 5
        period_seconds = 10
        failure_threshold = 5
      }

      liveness_probe {
        http_get {
          path = var.health_path
          port = var.port
        }
        initial_delay_seconds = 30
        timeout_seconds = 5
        period_seconds = 30
        failure_threshold = 3
      }

      dynamic "env" {
        for_each = var.env_vars
        content {
          name  = env.key
          value = env.value
        }
      }

      env {
        name  = "RUST_LOG"
        value = var.rust_log_level
      }

      # Database URL (if provided)
      dynamic "env" {
        for_each = var.database_url != "" ? [1] : []
        content {
          name  = "DATABASE_URL"
          value = var.database_url
        }
      }

      # Secret Manager secrets
      dynamic "env" {
        for_each = var.secret_manager_secrets
        content {
          name = env.key
          value_source {
            secret_key_ref {
              secret  = env.value
              version = "latest"
            }
          }
        }
      }
    }
  }

}