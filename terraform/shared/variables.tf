variable "project" {
  description = "GCP Project Id"
  type = string
}

variable "region" {
  description = "GCP Region"
  type = string
}

variable "github_repo" {
  description = "GitHub repository in format 'owner/repo'"
  type = string
}

variable "repository_name" {
  description = "Artifact Registry repository name"
  type = string
}
