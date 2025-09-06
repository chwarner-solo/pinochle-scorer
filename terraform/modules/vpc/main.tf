resource "google_compute_network" "main" {
  project                 = var.project
  name                   = var.env
  auto_create_subnetworks = false
  description            = "VPC network for ${var.env} environment"
}

resource "google_compute_subnetwork" "main" {
  project       = var.project
  name          = "${var.env}-subnet-01"
  ip_cidr_range = "10.0.1.0/24"
  region        = var.region
  network       = google_compute_network.main.self_link
  description   = "Main subnet for ${var.env} environment"
}