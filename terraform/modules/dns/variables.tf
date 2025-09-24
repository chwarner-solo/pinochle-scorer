variable "project" {
  description = "GCP project ID"
  type        = string
}

variable "domain_name" {
  description = "Domain name to manage (e.g., pinochle-scorer.app)"
  type        = string
  default     = null
}

variable "load_balancer_ip" {
  description = "Load balancer IP address to point domain to"
  type        = string
}

variable "ipv6_address" {
  description = "IPv6 address for AAAA record (optional)"
  type        = string
  default     = null
}

variable "create_www_record" {
  description = "Create www CNAME record pointing to main domain"
  type        = bool
  default     = true
}

variable "labels" {
  description = "Labels to apply to DNS resources"
  type        = map(string)
  default     = {}
}