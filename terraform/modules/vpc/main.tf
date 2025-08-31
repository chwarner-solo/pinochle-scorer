module "vpc" {
  source = "terraform-google-modules/network/google"
  version = "~> 11.1"

  project_id = "${var.project}"
  network_name = "${var.env}"

  subnets = [
    {
      subnet_name = "${var.env}-subnet-01"
      subnet_ip = "10.0.1.0/24"
      subnet_region = "${var.region}"
    }
  ]

  secondary_ranges = {
    "${var.env}-subnet-01" = []
  }
}