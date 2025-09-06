# modules/iam/outputs.tf

# Workload Identity Federation
output "wif_provider_name" {
  description = "Full name of the Workload Identity Provider"
  value       = google_iam_workload_identity_pool_provider.github_provider.name
}

output "wif_service_account_email" {
  description = "Email of the GitHub Actions service account"
  value       = google_service_account.github_actions_sa.email
}

# CloudRUN Service Account
output "cloudrun_service_account_email" {
  description = "Email of the CloudRUN service account"
  value       = google_service_account.cloudrun_sa.email
}

output "cloudrun_service_account_name" {
  description = "Name of the CloudRUN service account"
  value       = google_service_account.cloudrun_sa.name
}

# Frontend Deployment Service Account
output "frontend_deploy_service_account_email" {
  description = "Email of the frontend deployment service account"
  value       = google_service_account.frontend_deploy_sa.email
}

# For GitHub Secrets
output "github_secrets_summary" {
  description = "Summary of values needed for GitHub secrets"
  value = {
    WIF_PROVIDER     = google_iam_workload_identity_pool_provider.github_provider.name
    WIF_SERVICE_ACCOUNT = google_service_account.github_actions_sa.email
  }
}