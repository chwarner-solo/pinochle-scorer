output "bucket_name" {
  description = "Name of the Cloud Storage bucket"
  value       = google_storage_bucket.frontend.name
}

output "bucket_url" {
  description = "URL of the Cloud Storage bucket"
  value       = google_storage_bucket.frontend.url
}

output "frontend_ip" {
  description = "Global static IP for the frontend"
  value       = google_compute_global_address.frontend_ip.address
}

output "frontend_url" {
  description = "Frontend URL"
  value       = var.domain_name != null ? "https://${var.domain_name}" : "http://${google_compute_global_address.frontend_ip.address}"
}

output "api_url" {
  description = "API URL"
  value       = var.domain_name != null ? "https://${var.domain_name}/api" : "http://${google_compute_global_address.frontend_ip.address}/api"
}

output "deploy_service_account_email" {
  description = "Email of the service account for frontend deployment"
  value       = google_service_account.frontend_deploy_sa.email
}

output "cdn_enabled" {
  description = "Whether CDN is enabled"
  value       = var.enable_cdn
}