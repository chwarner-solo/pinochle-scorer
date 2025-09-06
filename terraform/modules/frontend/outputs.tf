output "bucket_name" {
  description = "Name of the Cloud Storage bucket"
  value       = google_storage_bucket.frontend.name
}

output "bucket_url" {
  description = "URL of the Cloud Storage bucket"
  value       = google_storage_bucket.frontend.url
}




output "deploy_service_account_email" {
  description = "Email of the service account for frontend deployment"
  value       = var.frontend_deploy_sa_email
}

output "cdn_enabled" {
  description = "Whether CDN is enabled"
  value       = var.enable_cdn
}