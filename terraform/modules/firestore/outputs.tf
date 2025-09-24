output "database_name" {
  description = "Firestore database name"
  value       = google_firestore_database.database.name
}

output "database_id" {
  description = "Firestore database ID"
  value       = google_firestore_database.database.name
}

output "location_id" {
  description = "Firestore database location"
  value       = google_firestore_database.database.location_id
}

output "database_type" {
  description = "Firestore database type"
  value       = google_firestore_database.database.type
}

output "database_url" {
  description = "Firestore database URL for client connections"
  value       = "https://firestore.googleapis.com/v1/projects/${var.project}/databases/${google_firestore_database.database.name}"
}