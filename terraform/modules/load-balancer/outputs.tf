output "frontend_ip" {
  description = "Global static IP address for the load balancer"
  value = google_compute_global_address.frontend_ip.address
}

output "frontend_ip_name" {
  description = "Name of hte global staic IP address"
  value = google_compute_global_address.frontend_ip.name
}

output "url_map_id" {
  description = "ID of the main URL map"
  value = google_compute_url_map.main_url_map.id
}

output "frontend_url" {
  description = "Frontend URL (IP-based or custom domain)"
  value = var.domain_name != null ? "https://${var.domain_name}" : "http://${google_compute_global_address.frontend_ip.address}"
}