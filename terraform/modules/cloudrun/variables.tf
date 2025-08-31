variable "project" {
  description = "GCP Project ID"
  type        = string
}

variable "region" {
  description = "GCP Region"
  type        = string
}

variable "service_name" {
  description = "CloudRun service name"
  type        = string
  default     = "pinochle-scorer-api"
}

variable "container_image" {
  description = "Container image URL from Artifact Registry"
  type        = string
  default     = "us-docker.pkg.dev/cloudrun/container/hello"
}

variable "port" {
  description = "Container port"
  type        = number
  default     = 3000
}

variable "cpu_limit" {
  description = "CPU limit for the container"
  type        = string
  default     = "1000m"
}

variable "memory_limit" {
  description = "Memory limit for the container"
  type        = string
  default     = "512Mi"
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

variable "env_vars" {
  description = "Environment variables for the container"
  type        = map(string)
  default     = {}
}

variable "allow_unauthenticated" {
  description = "Allow unauthenticated requests"
  type        = bool
  default     = true
}

variable "vpc_connector_name" {
  description = "VPC connector name for private network access"
  type        = string
  default     = null
}

variable "database_url" {
  description = "Database connection URL"
  type        = string
  default     = ""
}

variable "secret_manager_secrets" {
  description = "Secret Manager secrets to mount as environment variables"
  type        = map(string)
  default     = {}
}