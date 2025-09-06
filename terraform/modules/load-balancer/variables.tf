variable "project" {
  description = "GCP project ID"
  type = string
}

variable "region" {
  description = "GCP region"
  type = string
}

variable "frontend_bucket_name" {
  description = "Name of the GCS bucket for frontend (from frontend module)"
  type = string
}

variable "cloudrun_service_name" {
  description = "Name of the CloudRun service (from cloudrun module)"
  type = string
}

variable "enable_cdn" {
  description = "Enable Cloud CDN for frontend"
  type = bool
}

variable "domain_name" {
  description = "Custom domain name (optional)"
  type = string
  default = null
}

variable "ssl_redirect" {
  description = "Enabled SSL Redirect"
  type = bool
  default = false
}