# Firestore Database Configuration
resource "google_firestore_database" "database" {
  project     = var.project
  name        = var.database_id
  location_id = var.location_id
  type        = var.database_type

  # Delete protection
  delete_protection_state = var.delete_protection ? "DELETE_PROTECTION_ENABLED" : "DELETE_PROTECTION_DISABLED"
  deletion_policy         = var.deletion_policy

  depends_on = [google_project_service.firestore_api]
}

# Enable required APIs
resource "google_project_service" "firestore_api" {
  project = var.project
  service = "firestore.googleapis.com"

  disable_dependent_services = false
  disable_on_destroy         = false
}

resource "google_project_service" "appengine_api" {
  project = var.project
  service = "appengine.googleapis.com"

  disable_dependent_services = false
  disable_on_destroy         = false
}

# Firestore security rules (optional)
resource "google_firestore_document" "security_rules" {
  count       = var.security_rules != null ? 1 : 0
  project     = var.project
  database    = google_firestore_database.database.name
  collection  = var.security_rules_collection
  document_id = var.security_rules_document_id

  fields = jsonencode(var.security_rules)

  depends_on = [google_firestore_database.database]
}