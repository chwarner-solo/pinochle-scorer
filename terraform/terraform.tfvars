
project = "pinochle-scorer-471315"
github_repository = "chwarner-solo/pinochle-scorer"

region = "us-central1"
environment = "dev"

bucket_name = "frontend"
frontend_enable_versioning = false

enable_cdn = true
ssl_redirect = true
domain_name = "pinochle-scorer.app"

api_service_name = "pinochle-scorer-api"

api_cpu_limit = "2"
api_memory_limit = "512Mi"

api_min_instances = 0
api_max_instances = 1

api_port = 8080
api_health_path = "/api/health"
api_rust_log_level = "info"
api_allow_unauthenticated = true

api_env_vars = {
  RUST_BACKTRACE = "1"
  RUST_LOG = "info"
}

