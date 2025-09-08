# Reserve IP range for private services access
resource "google_compute_global_address" "private_ip_address" {
  project       = var.project
  name          = "private-ip-address"
  purpose       = "VPC_PEERING"
  address_type  = "INTERNAL"
  prefix_length = 16
  network       = var.network
}

# Create private connection for CloudSQL
resource "google_service_networking_connection" "private_vpc_connection" {
  network                 = var.network
  service                 = "servicenetworking.googleapis.com"
  reserved_peering_ranges = [google_compute_global_address.private_ip_address.name]
}

# Generate random password for database user
resource "random_password" "db_password" {
  length  = 16
  special = true
}

# CloudSQL PostgreSQL instance
resource "google_sql_database_instance" "main" {
  project             = var.project
  name                = "pinochle-scorer-db"
  database_version    = var.database_version
  region              = var.region
  deletion_protection = var.deletion_protection

  depends_on = [google_service_networking_connection.private_vpc_connection]

  settings {
    tier              = var.instance_tier
    availability_type = "ZONAL"
    disk_size         = 10
    disk_type         = "PD_SSD"

    backup_configuration {
      enabled                        = true
      start_time                     = "03:00"
      location                       = var.region
      point_in_time_recovery_enabled = false
      transaction_log_retention_days = 1
      backup_retention_settings {
        retained_backups = 1
        retention_unit   = "COUNT"
      }
    }

    ip_configuration {
      ipv4_enabled                                  = false
      private_network                               = var.network
      enable_private_path_for_google_cloud_services = true
    }

    database_flags {
      name  = "log_connections"
      value = "on"
    }

    database_flags {
      name  = "log_disconnections"
      value = "on"
    }
  }
}

# Create database
resource "google_sql_database" "database" {
  project  = var.project
  name     = var.database_name
  instance = google_sql_database_instance.main.name
}

# Create database user
resource "google_sql_user" "user" {
  project  = var.project
  name     = var.database_user
  instance = google_sql_database_instance.main.name
  password = random_password.db_password.result
}

# Create IAM database user for service account authentication
resource "google_sql_user" "iam_user" {
  project  = var.project
  name     = trimsuffix(var.cloudrun_service_account, ".gserviceaccount.com")
  instance = google_sql_database_instance.main.name
  type     = "CLOUD_IAM_SERVICE_ACCOUNT"
}

# Grant CloudRun service account permission to connect to CloudSQL
resource "google_project_iam_member" "cloudsql_client" {
  project = var.project
  role    = "roles/cloudsql.client"
  member  = "serviceAccount:${var.cloudrun_service_account}"
}

# Grant CloudRun service account permission to access Secret Manager
resource "google_project_iam_member" "secret_accessor" {
  project = var.project
  role    = "roles/secretmanager.secretAccessor" 
  member  = "serviceAccount:${var.cloudrun_service_account}"
}

# Store database password in Secret Manager
resource "google_secret_manager_secret" "db_password" {
  project   = var.project
  secret_id = "db-password"

  replication {
    auto {}
  }
}

resource "google_secret_manager_secret_version" "db_password" {
  secret      = google_secret_manager_secret.db_password.id
  secret_data = random_password.db_password.result
}