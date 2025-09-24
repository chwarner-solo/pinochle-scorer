variable "project" {
  description = "GCP project ID"
  type        = string
}

variable "database_id" {
  description = "Firestore database ID"
  type        = string
  default     = "(default)"
}

variable "location_id" {
  description = "Location for Firestore database (e.g., us-central, nam5, eur3)"
  type        = string
  default     = "nam5"

  validation {
    condition = contains([
      "nam5", "us-central", "us-east1", "us-east4", "us-west1", "us-west2", "us-west3", "us-west4",
      "eur3", "europe-west1", "europe-west2", "europe-west3", "europe-west6",
      "asia-northeast1", "asia-south1", "asia-southeast1", "australia-southeast1"
    ], var.location_id)
    error_message = "Location must be a valid Firestore location."
  }
}

variable "database_type" {
  description = "Database type (FIRESTORE_NATIVE or DATASTORE_MODE)"
  type        = string
  default     = "FIRESTORE_NATIVE"

  validation {
    condition     = contains(["FIRESTORE_NATIVE", "DATASTORE_MODE"], var.database_type)
    error_message = "Database type must be FIRESTORE_NATIVE or DATASTORE_MODE."
  }
}

variable "delete_protection" {
  description = "Enable delete protection for the database"
  type        = bool
  default     = true
}

variable "deletion_policy" {
  description = "Deletion policy for the database (ABANDON or DELETE)"
  type        = string
  default     = "ABANDON"

  validation {
    condition     = contains(["ABANDON", "DELETE"], var.deletion_policy)
    error_message = "Deletion policy must be ABANDON or DELETE."
  }
}

variable "security_rules" {
  description = "Firestore security rules as a map"
  type        = any
  default     = null
}

variable "security_rules_collection" {
  description = "Collection name for storing security rules"
  type        = string
  default     = "_firestore_rules"
}

variable "security_rules_document_id" {
  description = "Document ID for security rules"
  type        = string
  default     = "rules"
}

variable "service_account_email" {
  description = "Service account email that needs Firestore permissions"
  type        = string
}