# modules/artifact-registry/outputs.tf
output "repository_name" {
  description = "Name of the created Artifact Registry repository"
  value       = google_artifact_registry_repository.docker_repo.repository_id
}

output "repository_url" {
  description = "Full URL of the Artifact Registry repository"
  value       = "${var.region}-docker.pkg.dev/${var.project}/${google_artifact_registry_repository.docker_repo.repository_id}"
}

output "repository_id" {
  description = "ID of the created repository"
  value       = google_artifact_registry_repository.docker_repo.id
}