# Enable required Google Cloud APIs
resource "google_project_service" "apis" {
  for_each = toset([
    "run.googleapis.com",                    # Cloud Run
    "compute.googleapis.com",                # Compute Engine (for load balancer)
    "artifactregistry.googleapis.com",      # Artifact Registry
    "iam.googleapis.com",                    # IAM for service accounts
    "iamcredentials.googleapis.com",         # IAM Service Account Credentials
    "sts.googleapis.com",                    # Security Token Service
    "secretmanager.googleapis.com",          # Secret Manager
    "servicenetworking.googleapis.com",     # Service Networking (for CloudSQL VPC)
    "sqladmin.googleapis.com",               # Cloud SQL
    "vpcaccess.googleapis.com",              # Serverless VPC Access
    "storage.googleapis.com",                # Cloud Storage
  ])

  project = var.project
  service = each.key

  disable_dependent_services = false
  disable_on_destroy = false
}

# Add small delay to allow APIs to propagate
resource "time_sleep" "api_propagation" {
  depends_on = [google_project_service.apis]
  create_duration = "30s"
}