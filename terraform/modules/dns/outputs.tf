output "dns_zone_name" {
  description = "Name of the created DNS managed zone"
  value       = length(google_dns_managed_zone.main_zone) > 0 ? google_dns_managed_zone.main_zone[0].name : null
}

output "dns_zone_id" {
  description = "ID of the created DNS managed zone"
  value       = length(google_dns_managed_zone.main_zone) > 0 ? google_dns_managed_zone.main_zone[0].id : null
}

output "name_servers" {
  description = "Name servers for the DNS zone (configure these with your domain registrar)"
  value       = length(google_dns_managed_zone.main_zone) > 0 ? google_dns_managed_zone.main_zone[0].name_servers : []
}

output "dns_name" {
  description = "The fully qualified DNS name"
  value       = length(google_dns_managed_zone.main_zone) > 0 ? google_dns_managed_zone.main_zone[0].dns_name : null
}