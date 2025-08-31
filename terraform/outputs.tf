output "registry_url" {
  value = "${module.shared.registry_url}"
}

output "api_url" {
  description = "URL of the deployed Pinochle Scorer API"
  value = "${module.cloudrun.service_url}"
}

output "service_account_email" {
  description = "Email of the GitHub Actions service account"
  value = "${module.shared.service_account_email}"
}

output "cloudrun_service_account" {
  description = "Email of the CloudRun service account"
  value = "${module.cloudrun.service_account_email}"
}

output "database_connection_name" {
  description = "CloudSQL connection name"
  value = "${module.cloudsql.instance_connection_name}"
}

output "database_private_ip" {
  description = "CloudSQL private IP address"
  value = "${module.cloudsql.private_ip_address}"
}

output "vpc_connector_name" {
  description = "VPC connector name"
  value = "${module.vpc_connector.connector_name}"
}

output "frontend_url" {
  description = "Frontend application URL"
  value = "${module.frontend.frontend_url}"
}

output "frontend_bucket" {
  description = "Frontend storage bucket name"
  value = "${module.frontend.bucket_name}"
}

output "frontend_deploy_sa" {
  description = "Service account for frontend deployment"
  value = "${module.frontend.deploy_service_account_email}"
}

output "api_endpoint" {
  description = "API endpoint URL (HTTPS)"
  value = "${module.frontend.api_url}"
}