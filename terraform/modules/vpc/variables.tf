variable "project" {
  description = "GCP project ID"
  type        = string
}

variable "region" {
  description = "GCP region for VPC resources"
  type        = string
}

variable "env" {
  description = "Environment name (dev, staging, prod)"
  type        = string
}