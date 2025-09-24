# DNS Zone and Records Management
resource "google_dns_managed_zone" "main_zone" {
  count       = var.domain_name != null ? 1 : 0
  project     = var.project
  name        = replace(var.domain_name, ".", "-")
  dns_name    = "${var.domain_name}."
  description = "DNS zone for ${var.domain_name}"

  labels = var.labels
}

# A record pointing to load balancer IP
resource "google_dns_record_set" "a_record" {
  count        = var.domain_name != null ? 1 : 0
  project      = var.project
  name         = "${var.domain_name}."
  managed_zone = google_dns_managed_zone.main_zone[0].name
  type         = "A"
  ttl          = 300

  rrdatas = [var.load_balancer_ip]
}

# AAAA record for IPv6 (optional)
resource "google_dns_record_set" "aaaa_record" {
  count        = var.domain_name != null && var.ipv6_address != null ? 1 : 0
  project      = var.project
  name         = "${var.domain_name}."
  managed_zone = google_dns_managed_zone.main_zone[0].name
  type         = "AAAA"
  ttl          = 300

  rrdatas = [var.ipv6_address]
}

# CNAME for www subdomain
resource "google_dns_record_set" "www_cname" {
  count        = var.domain_name != null && var.create_www_record ? 1 : 0
  project      = var.project
  name         = "www.${var.domain_name}."
  managed_zone = google_dns_managed_zone.main_zone[0].name
  type         = "CNAME"
  ttl          = 300

  rrdatas = ["${var.domain_name}."]
}