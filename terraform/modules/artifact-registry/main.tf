# modules/artifact-registry/main.tf
resource "google_artifact_registry_repository" "docker_repo" {
  location      = var.region
  repository_id = var.repository_name
  description   = "Docker repository for ${var.project} containers"
  format        = "DOCKER"

  labels = var.labels

  cleanup_policies {
    id     = "delete-prerelease"
    action = "DELETE"
    condition {
      tag_state  = "TAGGED"
      tag_prefixes = ["alpha", "beta", "rc"]
      older_than = "2592000s" # 30 days
    }
  }

  cleanup_policies {
    id     = "keep-minimum-versions"
    action = "KEEP"
    most_recent_versions {
      keep_count = 5
    }
  }
}

# IAM binding to allow GitHub Actions to push/pull
resource "google_artifact_registry_repository_iam_member" "github_actions" {
  project    = var.project
  location   = google_artifact_registry_repository.docker_repo.location
  repository = google_artifact_registry_repository.docker_repo.name
  role       = "roles/artifactregistry.writer"
  member     = "serviceAccount:${var.github_actions_service_account}"
}

resource "google_artifact_registry_repository_iam_member" "cloudrun_puller" {
  project    = var.project
  location   = google_artifact_registry_repository.docker_repo.location
  repository = google_artifact_registry_repository.docker_repo.name
  role       = "roles/artifactregistry.reader"
  member     = "serviceAccount:${var.cloudrun_service_account}"
}