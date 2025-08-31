variable "project" {
  description = "GCP Project ID"
  type        = string
}

variable "region" {
  description = "GCP Region"
  type        = string
}

variable "bucket_name" {
  description = "Cloud Storage bucket name for frontend"
  type        = string
  default     = "pinochle-scorer-frontend"
}

variable "domain_name" {
  description = "Custom domain name for the frontend (optional)"
  type        = string
  default     = null
}

variable "api_service_url" {
  description = "CloudRun API service URL for load balancer backend"
  type        = string
}

variable "enable_cdn" {
  description = "Enable Cloud CDN"
  type        = bool
  default     = true
}