# modules/cloudrun/variables.tf

variable "project" {
  description = "GCP project ID"
  type        = string
}

variable "region" {
  description = "GCP region"
  type        = string
}

variable "service_name" {
  description = "Name of the CloudRUN service"
  type        = string
}

variable "container_image" {
  description = "Container image to deploy (empty = use placeholder)"
  type        = string
  default     = ""
}

variable "port" {
  description = "Container port"
  type        = number
  default     = 8080
}

variable "cpu_limit" {
  description = "CPU limit"
  type        = string
  default     = "2"
}

variable "memory_limit" {
  description = "Memory limit"
  type        = string
  default     = "2Gi"
}

variable "min_instances" {
  description = "Minimum number of instances"
  type        = number
  default     = 0
}

variable "max_instances" {
  description = "Maximum number of instances"
  type        = number
  default     = 10
}

variable "health_path" {
  description = "Health check endpoint path"
  type        = string
  default     = "/api/health"
}

variable "rust_log_level" {
  description = "Rust log level"
  type        = string
  default     = "info"
}

variable "allow_unauthenticated" {
  description = "Allow unauthenticated requests"
  type        = bool
  default     = true
}

# VPC Integration
variable "vpc_connector_name" {
  description = "VPC connector name for private networking"
  type        = string
  default     = null
}

# Environment variables
variable "env_vars" {
  description = "Environment variables as key-value pairs"
  type        = map(string)
  default     = {}
}

variable "firestore_database_url" {
  description = "Firestore database URL"
  type        = string
  default     = ""
  sensitive   = true
}

variable "secret_manager_secrets" {
  description = "Map of environment variable names to Secret Manager secret names"
  type        = map(string)
  default     = {}
}

variable "service_account_email" {
  description = "Cloud  Run Service Account"
  type = string
}