provider "google" {
  project = "${var.project}"
  region = "${var.region}"
}
locals {
  env = "dev"
}

module "shared" {
  source = "./shared"

  project = "${var.project}"
  region = "${var.region}"
  github_repo = "cwarner-solo/pinochle-scorer"
  repository_name = "pinochle-scorer-registry"
  
  depends_on = [time_sleep.api_propagation]
}

module "firewall" {
  source = "./modules/firewall"

  project = "${var.project}"
  region = "${var.region}"
  subnet = "${module.vpc.subnet}"
}

module "vpc" {
  source = "./modules/vpc"

  project = "${var.project}"
  region = "${var.region}"
  env = "${local.env}"
}

module "vpc_connector" {
  source = "./modules/vpc-connector"

  project = "${var.project}"
  region = "${var.region}"
  network = "${module.vpc.network_name}"
  subnet = "${module.vpc.subnet}"
  
  depends_on = [time_sleep.api_propagation]
}

module "cloudsql" {
  source = "./modules/cloudsql"

  project = "${var.project}"
  region = "${var.region}"
  network = "${module.vpc.network_self_link}"
  
  depends_on = [time_sleep.api_propagation]
}

module "cloudrun" {
  source = "./modules/cloudrun"

  project = "${var.project}"
  region = "${var.region}"
  container_image = var.app_version == "hello" ? "us-docker.pkg.dev/cloudrun/container/hello" : "${module.shared.registry_url}/pinochle-scorer:${var.app_version}"
  service_name = "pinochle-scorer-api"
  vpc_connector_name = "${module.vpc_connector.connector_name}"
  database_url = "${module.cloudsql.database_url}"
  
  env_vars = {
    RUST_ENV = "production"
    CORS_ALLOWED_ORIGINS = "${module.frontend.frontend_url}"
    FRONTEND_URL = "${module.frontend.frontend_url}"
    API_BASE_PATH = "/api"
  }
  
  secret_manager_secrets = {
    DATABASE_PASSWORD = "${module.cloudsql.secret_name}"
  }

  depends_on = [module.vpc_connector, module.cloudsql]
}

module "frontend" {
  source = "./modules/frontend"

  project = "${var.project}"
  region = "${var.region}"
  api_service_url = "${module.cloudrun.service_url}"
  bucket_name = "pinochle-scorer-frontend"
  enable_cdn = true
  
  # Leave domain_name null to use Google's temporary domain
  domain_name = null
}