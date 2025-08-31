variable "project" {
  description = "GCP Project Id"
  type = string
  default = "pinochle-scorer"
}

variable "region" {
  description = "GCP Region"
  type = string
  default = "us-central1"
}

variable "github_repo" {
  description = "GitHub repository in format 'owner/repo'"
  type = string
  default = "chwarner-solo/pinochle-scorer"
}

variable "repository_name" {
  description = "Artifact Registry repository name"
  type = string
  default = "docker-registry"
}

variable "app_version" {
  description = "Application version to deploy"
  type = string
  default = "hello"
}
