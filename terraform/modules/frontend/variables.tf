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

variable "enable_cdn" {
  description = "Enable Cloud CDN"
  type        = bool
  default     = true
}

variable "enable_versioning" {
  description = "Enable object versioning for the frontend bucket"
  type        = bool
  default     = false
}

variable "frontend_deploy_sa_email" {
  description = "Email of the frontend deployment service account"
  type        = string
}