output "network_name" {
  value = "${module.vpc.network_name}"
}

output "network_self_link" {
  value = "${module.vpc.network_self_link}"
}

output "subnet" {
  value = "${element(module.vpc.subnets_names, 0)}"
}

output "subnet_self_link" {
  value = "${element(module.vpc.subnets_self_links, 0)}"
}