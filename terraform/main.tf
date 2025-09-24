# terraform/main.tf - Root orchestration

# ============================================================================
# IAM Module - Foundation for all security and GitHub integration
# ============================================================================
module "iam" {
  source = "./modules/iam"

  project                     = var.project
  github_repository           = var.github_repository
  service_name                = var.api_service_name
  frontend_bucket_name        = "${var.project}-${var.bucket_name}"
  frontend_bucket_dependency  = module.frontend.bucket_name
}

# ============================================================================
# Networking Foundation - VPC, Connectors, Security
# ============================================================================
module "vpc" {
  source = "./modules/vpc"

  project = var.project
  region  = var.region
  env = var.environment
}

module "vpc_connector" {
  source = "./modules/vpc-connector"

  project    = var.project
  region     = var.region
  vpc_name   = module.vpc.vpc_name
  subnet     = module.vpc.subnet

  depends_on = [module.vpc]
}

module "firewall" {
  source = "./modules/firewall"

  project  = var.project
  region   = var.region
  subnet   = module.vpc.subnet
  vpc_name = module.vpc.vpc_name

  depends_on = [module.vpc]
}

# ============================================================================
# Database - Firestore NoSQL database
# ============================================================================
# NOTE: CloudSQL module removed - applying this will destroy CloudSQL resources
# module "cloudsql" {
#   source = "./modules/cloudsql"
#   project = var.project
#   region  = var.region
#   network = module.vpc.network_self_link
#   cloudrun_service_account = module.iam.cloudrun_service_account_email
#   depends_on = [module.vpc, module.iam]
# }

module "firestore" {
  source = "./modules/firestore"

  project               = var.project
  database_id          = var.firestore_database_id
  location_id          = var.firestore_location_id
  database_type        = var.firestore_database_type
  delete_protection    = var.firestore_delete_protection
  service_account_email = module.iam.cloudrun_service_account_email

  depends_on = [module.iam]
}

# ============================================================================
# Container Registry - Docker images for deployment
# ============================================================================
module "artifact_registry" {
  source = "./modules/artifact-registry"

  project                           = var.project
  region                           = var.region
  repository_name                  = var.artifact_registry_repository_name
  labels                          = local.common_labels
  github_actions_service_account  = module.iam.wif_service_account_email
  cloudrun_service_account        = module.iam.cloudrun_service_account_email

  depends_on = [module.iam]
}

# ============================================================================
# Application Layer - Frontend and API
# ============================================================================

# Frontend - GCS bucket for React SPA (static hosting)
module "frontend" {
  source = "./modules/frontend"

  project                  = var.project
  region                   = var.region
  bucket_name              = var.bucket_name
  enable_versioning        = var.frontend_enable_versioning
  frontend_deploy_sa_email = module.iam.frontend_deploy_service_account_email
}

# CloudRUN API - Rust application with database connectivity
module "cloudrun" {
  source = "./modules/cloudrun"

  project                   = var.project
  region                    = var.region
  service_name              = var.api_service_name
  container_image           = var.api_container_image
  port                      = var.api_port
  cpu_limit                 = var.api_cpu_limit
  memory_limit              = var.api_memory_limit
  min_instances             = var.api_min_instances
  max_instances             = var.api_max_instances
  health_path               = var.api_health_path
  rust_log_level            = var.api_rust_log_level
  allow_unauthenticated     = var.api_allow_unauthenticated

  # Networking
  vpc_connector_name        = module.vpc_connector.connector_name

  # Database integration
  firestore_database_url    = module.firestore.database_url

  # Environment variables
  env_vars                  = var.api_env_vars
  secret_manager_secrets    = var.api_secret_manager_secrets

  # Service Account from IAM module
  service_account_email     = module.iam.cloudrun_service_account_email

  depends_on = [
    module.vpc_connector,
    module.iam
  ]
}

# ============================================================================
# Load Balancer - Global HTTP(S) Load Balancer for frontend and API
# ============================================================================
module "load_balancer" {
  source = "./modules/load-balancer"

  project                = var.project
  region                 = var.region
  frontend_bucket_name   = module.frontend.bucket_name
  cloudrun_service_name  = module.cloudrun.service_name
  enable_cdn            = var.enable_cdn
  domain_name           = var.domain_name
  ssl_redirect          = var.ssl_redirect

  depends_on = [module.frontend, module.cloudrun]
}

# ============================================================================
# DNS Management - Domain and DNS records
# ============================================================================
module "dns" {
  source = "./modules/dns"

  project           = var.project
  domain_name       = var.domain_name
  load_balancer_ip  = module.load_balancer.frontend_ip
  create_www_record = true
  labels           = local.common_labels

  depends_on = [module.load_balancer]
}

# ============================================================================
# Local values for computed configurations
# ============================================================================
locals {
  # Common tags for all resources
  common_labels = {
    project     = var.project
    environment = var.environment
    managed_by  = "terraform"
    application = "pinochle-scorer"
  }

  # API configuration computed values
  api_full_name = "${var.project}-${var.api_service_name}"

  # Frontend configuration computed values
  frontend_full_bucket_name = "${var.project}-${var.bucket_name}"
}