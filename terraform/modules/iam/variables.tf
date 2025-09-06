# modules/iam/variables.tf

variable "project" {
  description = "GCP Project ID"
  type        = string
}

variable "github_repository" {
  description = "GitHub repository in format 'owner/repo'"
  type        = string
}

variable "service_name" {
  description = "Name of the service"
  type        = string
}

variable "frontend_bucket_name" {
  description = "Name of the frontend storage bucket"
  type        = string
}

# Add this variable for dependency management
variable "frontend_bucket_dependency" {
  description = "Dependency to ensure bucket is created before IAM"
  type        = any
  default     = null
}