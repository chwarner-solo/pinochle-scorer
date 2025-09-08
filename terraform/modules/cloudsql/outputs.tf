output "instance_name" {
  description = "CloudSQL instance name"
  value       = google_sql_database_instance.main.name
}

output "instance_connection_name" {
  description = "CloudSQL instance connection name"
  value       = google_sql_database_instance.main.connection_name
}

output "private_ip_address" {
  description = "Private IP address of CloudSQL instance"
  value       = google_sql_database_instance.main.private_ip_address
}

output "database_name" {
  description = "Database name"
  value       = google_sql_database.database.name
}

output "database_user" {
  description = "Database user"
  value       = google_sql_user.user.name
}

output "database_url" {
  description = "PostgreSQL connection URL (without password)"
  value       = "postgresql://${google_sql_user.user.name}@${google_sql_database_instance.main.private_ip_address}:5432/${google_sql_database.database.name}"
  sensitive   = false
}

output "secret_name" {
  description = "Secret Manager secret name for database password"
  value       = google_secret_manager_secret.db_password.secret_id
}

output "iam_user" {
  description = "IAM database user for service account authentication"
  value       = google_sql_user.iam_user.name
}

output "connection_string_iam" {
  description = "PostgreSQL connection string for IAM authentication"
  value       = "postgresql://${google_sql_user.iam_user.name}@${google_sql_database_instance.main.private_ip_address}:5432/${google_sql_database.database.name}"
  sensitive   = false
}