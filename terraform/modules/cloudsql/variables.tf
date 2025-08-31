variable "project" {
  description = "GCP Project ID"
  type        = string
}

variable "region" {
  description = "GCP Region"
  type        = string
}

variable "network" {
  description = "VPC network for private IP"
  type        = string
}

variable "database_name" {
  description = "Database name"
  type        = string
  default     = "pinochle_scorer"
}

variable "database_user" {
  description = "Database user"
  type        = string
  default     = "api_user"
}

variable "instance_tier" {
  description = "CloudSQL instance tier"
  type        = string
  default     = "db-g1-small"
}

variable "database_version" {
  description = "PostgreSQL version"
  type        = string
  default     = "POSTGRES_15"
}

variable "deletion_protection" {
  description = "Enable deletion protection"
  type        = bool
  default     = false
}