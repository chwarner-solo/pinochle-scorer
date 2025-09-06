# modules/iam/main.tf - Centralized IAM management

# Workload Identity Federation for GitHub Actions
resource "google_iam_workload_identity_pool" "github_pool" {
  project                   = var.project
  workload_identity_pool_id = "github-pool"
  display_name              = "GitHub Actions Pool"
  description               = "Identity pool for GitHub Actions workflows"
  
  lifecycle {
    prevent_destroy = true
  }
}

resource "google_iam_workload_identity_pool_provider" "github_provider" {
  project                            = var.project
  workload_identity_pool_id          = google_iam_workload_identity_pool.github_pool.workload_identity_pool_id
  workload_identity_pool_provider_id = "github-provider"
  display_name                       = "GitHub Provider"

  attribute_mapping = {
    "google.subject"             = "assertion.sub"
    "attribute.actor"            = "assertion.actor"
    "attribute.repository"       = "assertion.repository"
    "attribute.repository_owner" = "assertion.repository_owner"
  }

  # Add condition to restrict to your repository for security
  attribute_condition = "attribute.repository == '${var.github_repository}'"

  oidc {
    issuer_uri = "https://token.actions.githubusercontent.com"
  }
}

# Service Account for GitHub Actions (Deploy permissions)
resource "google_service_account" "github_actions_sa" {
  project      = var.project
  account_id   = "github-actions"
  display_name = "GitHub Actions Service Account"
  description  = "Service account for GitHub Actions deployments"
}

# Allow GitHub repository to impersonate the service account
resource "google_service_account_iam_binding" "github_actions_wif" {
  service_account_id = google_service_account.github_actions_sa.name
  role               = "roles/iam.workloadIdentityUser"

  members = [
    "principalSet://iam.googleapis.com/${google_iam_workload_identity_pool.github_pool.name}/attribute.repository/${var.github_repository}"
  ]
}

# Permissions for GitHub Actions to deploy
resource "google_project_iam_member" "github_actions_permissions" {
  for_each = toset([
    "roles/run.admin",                    # Deploy to CloudRUN
    "roles/storage.admin",               # Deploy to GCS
    "roles/artifactregistry.writer",    # Push images
    "roles/iam.serviceAccountUser",      # Use service accounts
    "roles/secretmanager.secretAccessor" # Access secrets if needed
  ])

  project = var.project
  role    = each.value
  member  = "serviceAccount:${google_service_account.github_actions_sa.email}"
}

# Service Account for CloudRUN Application
resource "google_service_account" "cloudrun_sa" {
  project      = var.project
  account_id   = "${var.service_name}-sa"
  display_name = "CloudRUN Service Account - ${var.service_name}"
  description  = "Runtime service account for CloudRUN application"
}

# CloudRUN Application Permissions
resource "google_project_iam_member" "cloudrun_permissions" {
  for_each = toset([
    "roles/cloudsql.client",             # Connect to database
    "roles/secretmanager.secretAccessor", # Access secrets
    "roles/logging.logWriter",           # Write logs
    "roles/monitoring.metricWriter",     # Write metrics
  ])

  project = var.project
  role    = each.value
  member  = "serviceAccount:${google_service_account.cloudrun_sa.email}"
}

# Service Account for Frontend Deployment
resource "google_service_account" "frontend_deploy_sa" {
  project      = var.project
  account_id   = "frontend-deploy"
  display_name = "Frontend Deployment Service Account"
  description  = "Service account for frontend CI/CD"
}

# MOVED: Bucket IAM should depend on bucket creation
# This will be handled after the storage module creates the bucket
resource "google_storage_bucket_iam_member" "frontend_deploy_bucket_access" {
  # Add explicit dependency to ensure bucket exists first
  depends_on = [var.frontend_bucket_dependency]

  bucket = var.frontend_bucket_name
  role   = "roles/storage.objectAdmin"
  member = "serviceAccount:${google_service_account.frontend_deploy_sa.email}"
}