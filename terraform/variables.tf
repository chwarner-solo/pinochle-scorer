# terraform/variables.tf

# ============================================================================
# Global Configuration
# ============================================================================
variable "project" {
  description = "GCP project ID"
  type        = string
  validation {
    condition     = length(var.project) > 0
    error_message = "Project ID cannot be empty."
  }
}

variable "region" {
  description = "GCP region for resources"
  type        = string
  default     = "us-central1"
}

variable "environment" {
  description = "Environment name (dev, staging, prod)"
  type        = string
  default     = "dev"
  validation {
    condition     = contains(["dev", "staging", "prod"], var.environment)
    error_message = "Environment must be dev, staging, or prod."
  }
}

# ============================================================================
# GitHub Integration
# ============================================================================
variable "github_repository" {
  description = "GitHub repository in format 'owner/repo' for Workload Identity Federation"
  type        = string

  validation {
    condition     = can(regex("^[a-zA-Z0-9._-]+/[a-zA-Z0-9._-]+$", var.github_repository))
    error_message = "GitHub repository must be in format 'owner/repo'."
  }
}

# ============================================================================
# Frontend Configuration (React SPA)
# ============================================================================
variable "bucket_name" {
  description = "Name suffix for the frontend GCS bucket (will be prefixed with project ID)"
  type        = string
  default     = "frontend"
  validation {
    condition     = can(regex("^[a-z0-9][a-z0-9._-]{1,61}[a-z0-9]$", var.bucket_name))
    error_message = "Bucket name must be 3-63 characters, lowercase, and follow GCS naming rules."
  }
}

variable "frontend_enable_versioning" {
  description = "Enable object versioning for the frontend bucket"
  type        = bool
  default     = false
}

# ============================================================================
# Load Balancer Configuration
# ============================================================================
variable "enable_cdn" {
  description = "Enable Cloud CDN for static assets"
  type        = bool
  default     = true
}

variable "domain_name" {
  description = "Custom domain name for SSL certificate (optional)"
  type        = string
  default     = null
  validation {
    condition = var.domain_name == null || can(regex("^[a-zA-Z0-9][a-zA-Z0-9-]{1,61}[a-zA-Z0-9]\\.[a-zA-Z]{2,}$", var.domain_name))
    error_message = "Domain name must be a valid domain format."
  }
}

variable "ssl_redirect" {
  description = "Automatically redirect HTTP to HTTPS"
  type        = bool
  default     = true
}

# ============================================================================
# API Configuration (Rust CloudRUN Service)
# ============================================================================
variable "api_service_name" {
  description = "Name for the CloudRUN API service"
  type        = string
  default     = "pinochle-scorer-api"
  validation {
    condition     = can(regex("^[a-z][a-z0-9-]*[a-z0-9]$", var.api_service_name))
    error_message = "Service name must start with lowercase letter, contain only lowercase letters, numbers, and hyphens."
  }
}

variable "api_container_image" {
  description = "Container image for the API (empty string uses placeholder for initial deployment)"
  type        = string
  default     = ""
}

variable "api_port" {
  description = "Container port for the API service"
  type        = number
  default     = 8080
  validation {
    condition     = var.api_port >= 1 && var.api_port <= 65535
    error_message = "Port must be between 1 and 65535."
  }
}

# Resource Limits
variable "api_cpu_limit" {
  description = "CPU limit for CloudRUN service (e.g., '1', '2', '4')"
  type        = string
  default     = "2"
}

variable "api_memory_limit" {
  description = "Memory limit for CloudRUN service (e.g., '512Mi', '1Gi', '2Gi')"
  type        = string
  default     = "2Gi"
}

# Scaling Configuration
variable "api_min_instances" {
  description = "Minimum number of CloudRUN instances (0 for scale-to-zero)"
  type        = number
  default     = 0
  validation {
    condition     = var.api_min_instances >= 0 && var.api_min_instances <= 1000
    error_message = "Min instances must be between 0 and 1000."
  }
}

variable "api_max_instances" {
  description = "Maximum number of CloudRUN instances"
  type        = number
  default     = 10
  validation {
    condition     = var.api_max_instances >= 1 && var.api_max_instances <= 1000
    error_message = "Max instances must be between 1 and 1000."
  }
}

# Health and Monitoring
variable "api_health_path" {
  description = "Health check endpoint path for the API"
  type        = string
  default     = "/api/health"
}

variable "api_rust_log_level" {
  description = "Rust application log level"
  type        = string
  default     = "info"
  validation {
    condition     = contains(["error", "warn", "info", "debug", "trace"], var.api_rust_log_level)
    error_message = "Log level must be one of: error, warn, info, debug, trace."
  }
}

# Security
variable "api_allow_unauthenticated" {
  description = "Allow unauthenticated requests to the API"
  type        = bool
  default     = true
}

# Environment Variables
variable "api_env_vars" {
  description = "Environment variables for the API service"
  type        = map(string)
  default = {
    RUST_BACKTRACE = "1"
  }
}

variable "api_secret_manager_secrets" {
  description = "Map of environment variable names to Secret Manager secret names"
  type        = map(string)
  default     = {}
}