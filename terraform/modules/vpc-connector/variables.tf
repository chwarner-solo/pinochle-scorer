variable "project" {
  description = "GCP Project ID"
  type        = string
}

variable "region" {
  description = "GCP Region"
  type        = string
}

variable "vpc_name" {
  description = "VPC network name"
  type        = string
}

variable "subnet" {
  description = "VPC subnet name"
  type        = string
}

variable "connector_name" {
  description = "VPC connector name"
  type        = string
  default     = "vpc-connector"
}

variable "ip_cidr_range" {
  description = "IP CIDR range for VPC connector"
  type        = string
  default     = "10.8.0.0/28"
}

variable "min_throughput" {
  description = "Minimum throughput for VPC connector"
  type        = number
  default     = 200
}

variable "max_throughput" {
  description = "Maximum throughput for VPC connector"
  type        = number
  default     = 300
}