resource "google_service_account" "github_actions_sa" {
  project = var.project
  account_id = "github-actions-sa"
  display_name = "Service Account for GitHub Actions"
}

resource "google_project_iam_member" "artifact_registry_writer_binding" {
  project = "${var.project}"
  role = "roles/artifactregistry.writer"
  member = "serviceAccount:${google_service_account.github_actions_sa.email}"
}

resource "google_iam_workload_identity_pool" "github_pool" {
  project = var.project
  workload_identity_pool_id = "github-actions-pool"
  display_name = "GitHub Actions Pool"
}

resource "google_iam_workload_identity_pool_provider" "github_provider" {
  project                            = var.project
  workload_identity_pool_id          = google_iam_workload_identity_pool.github_pool.workload_identity_pool_id
  workload_identity_pool_provider_id = "github-actions-provider"
  display_name                       = "GitHub Actions Provider"
  attribute_condition = <<EOT
    attribute.repository == "${var.github_repo}" &&
    assertion.ref == "refs/heads/main" &&
    assertion.ref_type == "branch"
EOT
  oidc {
    issuer_uri = "https://token.actions.githubusercontent.com"
  }
  attribute_mapping = {
    "google.subject" = "assertion.sub"
    "attribute.actor" = "assertion.actor"
    "attribute.aud" = "assertion.aud"
    "attribute.repository" = "assertion.repository"
  }
}

resource "google_service_account_iam_member" "workload_identity_user" {
  service_account_id = google_service_account.github_actions_sa.name
  role = "roles/iam.workloadIdentityUser"
  member = "principalSet://iam.googleapis.com/${google_iam_workload_identity_pool.github_pool.name}/attribute.repository/${var.github_repo}"
}

resource "google_artifact_registry_repository" "pinochle_scorer" {
  project = var.project
  location = var.region
  repository_id = var.repository_name
  description = "Docker 'pinochle-scorer' for GitHub Actions"
  format = "DOCKER"
}