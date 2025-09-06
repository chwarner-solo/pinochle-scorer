output "network_name" {
  value = google_compute_network.main.name
}

output "network_self_link" {
  value = google_compute_network.main.self_link
}

output "subnet" {
  value = google_compute_subnetwork.main.name
}

output "subnet_self_link" {
  value = google_compute_subnetwork.main.self_link
}

output "vpc_name" {
  value = google_compute_network.main.name
}

output "vpc_id" {
  value = google_compute_network.main.id
}