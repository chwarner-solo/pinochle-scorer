# modules/artifact-registry/variables.tf
variable "project" {
  description = "The GCP project ID"
  type        = string
}

variable "region" {
  description = "The GCP region"
  type        = string
}

variable "repository_name" {
  description = "Name of the Artifact Registry repository"
  type        = string
  default     = "docker-repo"
}

variable "labels" {
  description = "Labels to apply to the repository"
  type        = map(string)
  default     = {}
}

variable "github_actions_service_account" {
  description = "Service account email for GitHub Actions"
  type        = string
}

variable "cloudrun_service_account" {
  description = "Service account email for Cloud Run"
  type        = string
}