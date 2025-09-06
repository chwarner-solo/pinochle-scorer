resource "google_vpc_access_connector" "connector" {
  project       = var.project
  name          = var.connector_name
  region        = var.region
  network       = var.vpc_name
  ip_cidr_range = var.ip_cidr_range
  
  min_throughput = var.min_throughput
  max_throughput = var.max_throughput
}