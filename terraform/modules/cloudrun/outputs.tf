# modules/cloudrun/outputs.tf

output "service_name" {
  description = "Name of the CloudRUN service"
  value       = google_cloud_run_v2_service.api.name
}

output "service_url" {
  description = "URL of the CloudRUN service"
  value       = google_cloud_run_v2_service.api.uri
}

output "service_id" {
  description = "ID of the CloudRUN service"
  value       = google_cloud_run_v2_service.api.id
}

output "service_account_email" {
  description = "Email of the service account used by CloudRUN"
  value       = var.service_account_email
}

output "location" {
  description = "Location where the service is deployed"
  value       = google_cloud_run_v2_service.api.location
}